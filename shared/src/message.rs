use serde::{Serialize, Deserialize};

use crate::Snapshot;
#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Hello(String),
    CompleteSnapshot(Snapshot)
}
