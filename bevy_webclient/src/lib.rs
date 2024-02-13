use serde::{Serialize, de::DeserializeOwned};
pub trait Message : Send + Sync + Serialize + DeserializeOwned + 'static {}
impl<T> Message for T where T : Send + Sync + Serialize + DeserializeOwned + 'static {}