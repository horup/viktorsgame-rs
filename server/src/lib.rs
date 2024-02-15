use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
mod systems;
mod resources;
use bevy_web_server::BevyWebServerPlugin;
pub use resources::*;

use shared::Message;
pub struct ServerPlugin;


impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BevyWebServerPlugin::new() as BevyWebServerPlugin<Message>)
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(1),
        )))
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_systems(Startup, systems::start)
        .add_systems(FixedUpdate, systems::transmit);
    }
}
