use bevy::prelude::*;
use bevy_web_server::{Connections, RecvPacket, SendPacket};
use shared::*;

use crate::Message;

pub fn start(mut commands:Commands) {
    for i in 0..10 {
        let r = 10.0;
        let x = rand::random::<f32>() * r - r / 2.0;
        let y = rand::random::<f32>() * r - r / 2.0;
        let p = shared::glam::Vec3::new(x, y, 0.0);
        commands.spawn(Thing {
            pos: p,
            vel: Default::default()
        }).insert(Replicate);
    }
}

type O<T> = Option<T>;
pub fn transmit(connections:Res<Connections>, mut send_writer:EventWriter<SendPacket<Message>>, replicates:Query<(Entity, O<&Thing>, O<&Player>), With<Replicate>>) {
    // create complete snapshot
    let mut snapshot = Snapshot::default();
    for (id, thing, player) in replicates.iter() {
        snapshot.entities.push(EntitySnapshot {
            id,
            thing: thing.map(|thing| ThingSnapshot {
                x: Some(thing.pos.x),
                y: Some(thing.pos.y),
                vx: Some(thing.vel.x),
                vy: Some(thing.vel.y),
            }),
            player: player.map(|player| PlayerSnapshot {
                name: Some(player.name.clone()),
            }),
        })
    }

    // send snapshot
    for (id, _) in connections.connections.iter() {
        send_writer.send(SendPacket { connection: id.clone(), msg: Message::CompleteSnapshot(snapshot.clone()) });
    }
}