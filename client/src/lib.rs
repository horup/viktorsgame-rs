mod systems;
pub use systems::*;
use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}};
use bevy_web_client::{BevyWebClientPlugin, SendMsg};
use shared::Message;
pub struct ClientPlugin;

fn test(mut send_writer:EventWriter<SendMsg<Message>>) {
    send_writer.send(SendMsg { msg: Message::Hello("Hello from Client".to_string()) });
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
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
        app.add_systems(Update, test);
    }
}