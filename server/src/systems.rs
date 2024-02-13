use bevy::ecs::{event::{EventReader, EventWriter}, system::Res};
use bevy_web_server::{Connections, RecvMsg, SendMsg};

use crate::Message;

pub fn hello_server(connections:Res<Connections>, mut send_writer:EventWriter<SendMsg<Message>>, mut recv_reader:EventReader<RecvMsg<Message>>) {
    for (id, _) in connections.connections.iter() {
        send_writer.send(SendMsg { connection: id.clone(), msg: Message::Hello("Hello from Server".to_string()) });
    }

    for msg in recv_reader.read() {
        println!("from {} = {:?}", msg.connection, msg.msg);
    }
}