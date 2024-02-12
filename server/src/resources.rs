use std::{collections::HashMap, sync::{Arc, Mutex}};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Message {
    Hello(String)
}

pub struct WebsocketConnection<T> where T : Send + Sync {
    pub sender:tokio::sync::mpsc::Sender<T>,
    pub is_connected:bool
}

pub struct ConnectionManager<T> where T : Send + Sync {
    pub websocket_connections:HashMap<Uuid, WebsocketConnection<T>>
}
impl<T : Send + Sync> Default for ConnectionManager<T> {
    fn default() -> Self {
        Self { websocket_connections: Default::default() }
    }
}

#[derive(Resource)]
pub struct WebServer<T> where T : Send + Sync {
    pub rt:tokio::runtime::Runtime,
    pub connection_manager:Arc<Mutex<ConnectionManager<T>>>,
}