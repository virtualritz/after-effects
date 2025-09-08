use crate::*;
use pr_sys::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ClipOperator {
    pub node_id: i32,
    pub node_type: String,
    pub hash: prPluginID,
    pub flags: i32,

    pub effect: Option<EffectDetails>
}

#[derive(Debug, Clone)]
pub struct EffectDetails {
    pub name: Option<String>,
    pub instance_id: Option<u32>,
    pub params: Option<PropertyData>
}

pub type ClipOperatorsMap = HashMap<i32, ClipOperator>;

pub struct VideoSequenceParser {
    segment_suite: suites::VideoSegment,
}

impl VideoSequenceParser {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            segment_suite: suites::VideoSegment::new()?,
        })
    }

    pub fn parse_clip_operators(&self, clip_node_id: i32) -> Result<ClipOperatorsMap, Error> {
        let clip_node_operators = self.segment_suite.node_operator_count(clip_node_id)?;
        let mut operators_map: ClipOperatorsMap = HashMap::new();

        for operator_node_index in 0..clip_node_operators {
            let operator_node_id = self
                .segment_suite
                .acquire_operator_node_id(clip_node_id, operator_node_index)?;

            let (operator_node_type, operator_node_hash, operator_node_flags) =
                self.segment_suite.node_info(operator_node_id)?;

            let effect = if operator_node_type == String::from_utf8_lossy(kVideoSegment_NodeType_Effect) {
                let effect_name = self
                    .segment_suite
                    .node_property(operator_node_id, Property::Effect_FilterMatchName)
                    .unwrap_or_else(|_| PropertyData::String("<Unknown Effect>".to_string()));

                let effect_instance_id = self
                    .segment_suite
                    .node_property(operator_node_id, Property::Effect_RuntimeInstanceID);

                let filter_params = self
                    .segment_suite
                    .node_property(operator_node_id, Property::Effect_FilterParams);

                Some(EffectDetails {
                    name: match effect_name {
                        PropertyData::String(s) => Some(s),
                        _ => None
                    },
                    instance_id: match effect_instance_id {
                        Ok(PropertyData::UInt32(x)) => Some(x),
                        _ => None
                    },
                    params: filter_params.ok()
                })
            } else {
                None
            };

            operators_map.insert(
                operator_node_id,
                ClipOperator {
                    node_id: operator_node_id,
                    node_type: operator_node_type,
                    hash: operator_node_hash,
                    flags: operator_node_flags,

                    effect
                },
            );
            self.segment_suite.release_video_node_id(operator_node_id)?;
        }

        Ok(operators_map)
    }
}
