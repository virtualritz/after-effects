//! Cross-thread type to persist the same object across different rendering threads when MFR (Multi frame rendering) is enabled
//!
//! By default, Adobe clones the `Instance` data object for every rendering thread.
//! This is not ideal for some cases, for example, when you want to share a single `Arc<Mutex<T>>` object across all threads.
//!
//! Instead, you could define a cross-thread type, which will be the same across all rendering threads and will persist state on disk as well
//!
//! The way it works is that it creates a global static map of RwLocks, and then uses flatten and unflatten to store an instance ID, in addition to the flattened object data
//! If the instance ID is found in the map, it will return the existing RwLock, otherwise it will create a new one from the serialized data and insert it into the map.
//!
//! Example usage:
//! ```
//! use serde::{Serialize, Deserialize};
//! use after_effects as ae;
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyInstance {
//!     inner_data: String,
//! }
//! ae::define_cross_thread_type!(MyInstance);
//! ```
//!
//! This macro will create a new type called `CrossThreadYourType`, in this case `CrossThreadMyInstance`. You can then use that type in your SequenceData (plugin instance).
//! Then in each of your render threads, you can get the instance using `let instance = my_instance.get().unwrap();`
//! This will return an Arc<RwLock<MyInstance>>, which you can then use to access and modify the inner data.
//!
//! Serialization and deserialization is handled automatically, so you can just use serde's derive macros.
//!
//! When you no longer need the instances, you can call `CrossThreadMyInstance::clear_map()` to clear the global static map.
//! You can do it in `GlobalSetdown`.

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
            impl serde::Serialize for [<CrossThread $type_name>] {
                fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    #[derive(serde::Serialize)]
                    struct Helper<'a, $type_name> {
                        id: u64,
                        data: &'a $type_name,
                    };
                    let rwlock = self.get().ok_or(serde::ser::Error::custom("Instance not found in static map"))?;
                    let locked = rwlock.read();
                    Helper {
                        id: self.id,
                        data: &*locked
                    }.serialize(serializer)
                }
            }
            impl<'de> serde::Deserialize<'de> for [<CrossThread $type_name>] {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
                    #[derive(serde::Deserialize)]
                    struct Helper<$type_name> {
                        id: u64,
                        data: $type_name,
                    };
                    let helper = Helper::deserialize(deserializer)?;
                    if let Some(inner) = Self::map().read().get(&helper.id) {
                        return Ok(Self { id: helper.id });
                    }

                    Self::map().write().insert(helper.id, std::sync::Arc::new($crate::parking_lot::RwLock::new(helper.data)));
                    Ok(Self { id: helper.id })
                }
            }
        }
    }
}
