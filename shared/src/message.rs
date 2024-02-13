use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Hello(String)
}
