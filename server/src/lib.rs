use std::{sync::Arc, time::Duration};
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
mod systems;
mod resources;
use bevy_webserver::BevyWebserver;
pub use resources::*;

pub struct ServerPlugin;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    Hello(String)
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 6.0),
        )))
        .add_plugins(BevyWebserver::new() as BevyWebserver<Message>)
        .add_systems(FixedUpdate, systems::hello_server);
    }
}
