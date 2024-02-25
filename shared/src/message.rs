use serde::{Serialize, Deserialize};

use crate::Snapshot;
#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    Hello(String),
    ServerInfo {
        timestep_sec:f32
    },
    CompleteSnapshot(Snapshot)
}
