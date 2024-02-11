use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
mod systems;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 6.0),
        )))
        .add_systems(Update, systems::hello_server);
    }
}
