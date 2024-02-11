use std::sync::{Arc, Mutex};

use bevy::prelude::*;

#[derive(Default)]
pub struct ConnectionManager {

}

#[derive(Resource)]
pub struct WebServer {
    pub rt:tokio::runtime::Runtime,
    pub connection_manager:Arc<Mutex<ConnectionManager>>
}