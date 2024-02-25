use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
mod systems;
use systems::*;
mod bundles;
pub use bundles::*;
mod resources;
use bevy_web_server::BevyWebServerPlugin;
pub use resources::*;
mod misc;
pub use misc::*;

use shared::Message;
pub struct ServerPlugin;


impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(BevyWebServerPlugin::new() as BevyWebServerPlugin<Message>)
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(1),
        )))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 20.0))
        .add_systems(Startup, start)
        .add_systems(FixedUpdate, (connected, move_a_bit_for_fun, transmit_changes));
    }
}
