use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
mod systems;
mod resources;
use bevy_webserver::BevyWebserverPlugin;
pub use resources::*;

pub struct ServerPlugin;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    Hello(String)
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BevyWebserverPlugin::new() as BevyWebserverPlugin<Message>)
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(1),
        )))
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_systems(FixedUpdate, systems::hello_server);
    }
}
