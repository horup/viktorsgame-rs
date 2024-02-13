use bevy::{prelude::*, render::{settings::{Backends, WgpuSettings}, RenderPlugin}};
use bevy_web_client::BevyWebClientPlugin;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyWebClientPlugin::new() as BevyWebClientPlugin<shared::Message>);
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation:bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                backends:Some(Backends::DX12),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}