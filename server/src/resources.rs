use bevy::prelude::*;
use std::sync::Arc;

#[derive(Resource)]
pub struct Runtime {
    pub rt:tokio::runtime::Runtime
}