use bevy::prelude::*;
use bevy_web_server::{Connections, RecvMsg, SendMsg};
use shared::*;

use crate::Message;

pub fn start() {

}

type O<T> = Option<T>;
pub fn transmit(connections:Res<Connections>, mut send_writer:EventWriter<SendMsg<Message>>, entities:Query<(Entity, O<&Thing>, O<&Player>)>) {
    let mut snapshot = Snapshot::default();
    /*for (id, _) in connections.connections.iter() {
        send_writer.send(SendMsg { connection: id.clone(), msg: Message::Hello("Hello from Server".to_string()) });
    }

    for msg in recv_reader.read() {
        println!("from {} = {:?}", msg.connection, msg.msg);
    }*/
}