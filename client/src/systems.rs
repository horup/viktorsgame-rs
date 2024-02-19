use bevy::prelude::*;
use bevy_web_client::RecvPacket;
use shared::{Message, Player, Prev, Thing};

use crate::{EntityMapper, ServerEntity};

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
 /*   // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });*/
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn thing_spawned(mut commands: Commands, new_things:Query<(Entity, &Thing), Added<Thing>>) {
    for (id, thing) in new_things.iter() {
    }
}

pub fn recv(mut commands: Commands, mut reader:EventReader<RecvPacket<Message>>, mut entity_mapper:ResMut<EntityMapper>) {
    for msg in reader.read() {
        match &msg.msg {
            Message::Hello(_) => {

            },
            Message::CompleteSnapshot(snapshot) => {
                for entity_snapshot in &snapshot.entities {
                    let server_entity = entity_snapshot.id;
                    let mut spawned = false;
                    let client_entity = entity_mapper.server_to_client(&server_entity).copied().unwrap_or_else(||{
                        let id = commands.spawn(ServerEntity(entity_snapshot.id.clone())).id();
                        id
                    });
                    entity_mapper.map(server_entity, client_entity);

                    let mut client_entity = commands.entity(client_entity);
                    if let Some(thing) = &entity_snapshot.thing {
                        let thing = Thing {
                            pos: Vec3::new(thing.x.unwrap_or_default(), thing.y.unwrap_or_default(), 0.0),
                            vel: Vec3::new(thing.vx.unwrap_or_default(), thing.vy.unwrap_or_default(), 0.0),
                        };
                        let prev_thing = Prev(thing.clone());
                        client_entity.insert((thing, prev_thing));
                    }
                    if let Some(player) = &entity_snapshot.player {
                        let player = Player {
                            name: player.name.clone().unwrap_or_default(),
                        };
                        client_entity.insert(player);
                    }
                }
            },
        }
    }
}