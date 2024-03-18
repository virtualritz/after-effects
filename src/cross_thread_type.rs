/// Cross-thread type to persist the same object across different rendering threads when MFR (Multi frame rendering) is enabled
///
/// By default, Adobe clones the `Instance` data object for every rendering thread.
/// This is not ideal for some cases, for example, when you want to share a single `Arc<Mutex<T>>` object across all threads.
///
/// Instead, you could define a cross-thread type, which will be the same across all rendering threads and will persist state on disk as well
///
/// The way it works is that it creates a global static map of RwLocks, and then uses flatten and unflatten to store an instance ID in addition to the flattened object data
///
/// If the instance ID is found in the map, it will return the existing RwLock, otherwise it will create a new one from the serialized data and insert it into the map.
///
/// Example usage:
/// ```
/// use serde::{Serialize, Deserialize};
/// use after_effects as ae;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyInstance {
///     inner_data: String,
/// }
/// ae::define_cross_thread_type!(MyInstance);
/// ```
///
/// This macro will create a new type called `CrossThreadYourType`, in this case `CrossThreadMyInstance`. You can then use that type in your SequenceData (plugin instance).
/// Then in each of your render threads, you can get the instance using `let instance = my_instance.get().unwrap();`
/// This will return an `Arc<RwLock<MyInstance>>`, which you can then use to access and modify the inner data.
///
/// Serialization and deserialization is handled automatically, so you can just use serde's derive macros.
///
/// When you no longer need the instances, you can call `CrossThreadMyInstance::clear_map()` to clear the global static map.
/// You can do it in `GlobalSetdown`.
#[macro_export]
macro_rules! define_cross_thread_type {
    ($type_name:ty) => {
        $crate::paste::item! {
            pub struct [<CrossThread $type_name>] {
                id: u64
            }
            impl Default for [<CrossThread $type_name>] {
                fn default() -> Self {
                    let id = fastrand::u64(..);
                    let mut inst = <$type_name>::default();
                    Self::map().write().insert(id, std::sync::Arc::new($crate::parking_lot::RwLock::new(inst)));
                    Self { id }
                }
            }
            impl [<CrossThread $type_name>] {
                fn map() -> &'static $crate::parking_lot::RwLock<std::collections::HashMap<u64, std::sync::Arc<$crate::parking_lot::RwLock<$type_name>>>> {
                    use std::{ collections::HashMap, sync::{ Arc, OnceLock } };
                    use $crate::parking_lot::RwLock;

                    static MAP: OnceLock<RwLock<HashMap<u64, Arc<RwLock<$type_name>>>>> = OnceLock::new();
                    MAP.get_or_init(|| RwLock::new(HashMap::new()))
                }

                pub fn get(&self) -> Option<std::sync::Arc<$crate::parking_lot::RwLock<$type_name>>> {
                    Some(Self::map().read().get(&self.id)?.clone())
                }
                pub fn clear_map() {
                    Self::map().write().clear();
                }
            }
            impl $crate::serde::Serialize for [<CrossThread $type_name>] {
                fn serialize<S: $crate::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    use $crate::serde::ser::SerializeStruct;
                    let rwlock = self.get().ok_or($crate::serde::ser::Error::custom("Instance not found in static map"))?;

                    let mut state = serializer.serialize_struct(stringify!($type_name), 2)?;
                    state.serialize_field("id", &self.id)?;
                    state.serialize_field("data", &*rwlock.read())?;
                    state.end()
                }
            }
            impl<'de> $crate::serde::Deserialize<'de> for [<CrossThread $type_name>] {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: $crate::serde::Deserializer<'de> {
                    #[derive($crate::serde::Deserialize)]
                    #[serde(field_identifier, rename_all = "lowercase")]
                    enum Field { Id, Data }

                    struct HelperVisitor;
                    impl<'de> $crate::serde::de::Visitor<'de> for HelperVisitor {
                        type Value = [<CrossThread $type_name>];

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str(stringify!($type_name))
                        }

                        fn visit_seq<V>(self, mut seq: V) -> Result<[<CrossThread $type_name>], V::Error> where V: $crate::serde::de::SeqAccess<'de> {
                            let id: u64 = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                            if let Some(inner) = [<CrossThread $type_name>]::map().read().get(&id) {
                                return Ok([<CrossThread $type_name>] { id });
                            }

                            let data: $type_name = seq.next_element()?.ok_or_else(|| $crate::serde::de::Error::invalid_length(1, &self))?;
                            [<CrossThread $type_name>]::map().write().insert(id, std::sync::Arc::new($crate::parking_lot::RwLock::new(data)));

                            Ok([<CrossThread $type_name>] { id })
                        }

                        fn visit_map<V>(self, mut map: V) -> Result<[<CrossThread $type_name>], V::Error> where V: serde::de::MapAccess<'de> {
                            let mut id = None;
                            let mut data = None;
                            while let Some(key) = map.next_key()? {
                                match key {
                                    Field::Id => {
                                        let _id = map.next_value()?;
                                        if [<CrossThread $type_name>]::map().read().contains_key(&_id) {
                                            return Ok([<CrossThread $type_name>] { id: _id });
                                        }
                                        id = Some(_id);
                                    }
                                    Field::Data => {
                                        data = Some(map.next_value()?);
                                    }
                                }
                            }
                            let id: u64 = id.ok_or_else(|| $crate::serde::de::Error::missing_field("id"))?;
                            let data: $type_name = data.ok_or_else(|| $crate::serde::de::Error::missing_field("data"))?;
                            [<CrossThread $type_name>]::map().write().insert(id, std::sync::Arc::new($crate::parking_lot::RwLock::new(data)));
                            Ok([<CrossThread $type_name>] { id })
                        }
                    }

                    deserializer.deserialize_struct(stringify!($type_name), &["id", "data"], HelperVisitor)
                }
            }
        }
    }
}
