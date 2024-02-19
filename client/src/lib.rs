mod systems;
pub use systems::*;
mod components;
pub use components::*;
mod resources;
pub use resources::*;
use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}};
use bevy_web_client::{BevyWebClientPlugin, SendPacket};
use shared::Message;
pub struct ClientPlugin;

fn test(mut send_writer:EventWriter<SendPacket<Message>>) {
    send_writer.send(SendPacket { msg: Message::Hello("Hello from Client".to_string()) });
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityMapper::default());
        app.add_plugins(BevyWebClientPlugin::new() as BevyWebClientPlugin<shared::Message>);
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation:bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                backends:Some(Backends::VULKAN),
                ..Default::default()
            }),
            ..Default::default()
        }));
        app.add_systems(First, recv);
        app.add_systems(Startup, setup);
        app.add_systems(Update, (test, thing_spawned));
    }
}