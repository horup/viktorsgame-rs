use std::{collections::HashMap, sync::{Arc, Mutex}};

use bevy::prelude::*;
use uuid::Uuid;

pub struct WebsocketConnection {
    pub sender:tokio::sync::mpsc::Sender<Vec<u8>>,
    pub is_connected:bool
}

#[derive(Default)]
pub struct ConnectionManager {
    pub websocket_connections:HashMap<Uuid, WebsocketConnection>
    
}

#[derive(Resource)]
pub struct WebServer {
    pub rt:tokio::runtime::Runtime,
    pub connection_manager:Arc<Mutex<ConnectionManager>>,
}