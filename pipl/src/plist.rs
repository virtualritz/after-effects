use plist::{Dictionary, Value};

use crate::PIPLType;

pub(crate) fn produce_plist(path: String, kind: &PIPLType, name: &str) {
    let mut dict = Dictionary::new();

    dict.insert(
        "CFBundleIdentifier".to_string(),
        Value::String(format!("com.adobe.AfterEffects.{}", name)),
    );
    dict.insert(
        "CFBundleSignature".to_string(),
        Value::String("FXTC".to_string()),
    );
    dict.insert(
        "CFBundlePackageType".to_string(),
        Value::String(String::from_utf8_lossy(&kind.as_bytes()).to_string()),
    );

    match kind {
        PIPLType::AEGP => {
            dict.insert("LSRequiresCarbon".to_string(), Value::Boolean(true));
        }
        _ => {}
    };

    Value::from(dict).to_file_xml(path).unwrap();
}
