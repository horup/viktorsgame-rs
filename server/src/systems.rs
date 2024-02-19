use bevy::prelude::*;
use bevy_web_server::{Connection, RecvPacket, SendPacket};
use shared::*;

use crate::{misc, Message, PlayerBundle};

pub fn start(mut commands: Commands) {
    for i in 0..10 {
        let r = 10.0;
        let x = rand::random::<f32>() * r - r / 2.0;
        let y = rand::random::<f32>() * r - r / 2.0;
        let p = shared::glam::Vec3::new(x, y, 0.0);
        commands
            .spawn(Thing {
                pos: p,
                vel: Default::default(),
            })
            .insert(Prev(Thing::default()))
            .insert(Replicate);
    }
}


pub fn connected(
    new_connections: Query<(Entity, &Connection), Added<Connection>>,
    mut send_writer: EventWriter<SendPacket<Message>>,
    mut replicates: misc::AllReplicatesQuery
) {
    let snapshot = misc::new_complete_snapshot(&mut replicates);
    for (_, connection) in new_connections.iter() {
        send_writer.send(SendPacket {
            connection_id: connection.id.clone(),
            msg: Message::CompleteSnapshot(snapshot.clone()),
        });
    }
}

pub fn move_a_bit_for_fun(mut things:Query<&mut Thing>) {
    for mut thing in things.iter_mut() {
        thing.pos.x += 0.01;
    }
}

type O<T> = Option<T>;
pub fn transmit_changes(
    connections: Query<&Connection>,
    mut send_writer: EventWriter<SendPacket<Message>>,
    mut replicates: misc::AllReplicatesQuery
) {
    // create complete snapshot
    let snapshot = misc::new_complete_snapshot(&mut replicates);

    // send snapshot
    for connection in connections.iter() {
        send_writer.send(SendPacket {
            connection_id: connection.id.clone(),
            msg: Message::CompleteSnapshot(snapshot.clone()),
        });
    }
}
