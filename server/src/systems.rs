use bevy::ecs::{event::EventWriter, system::Res};
use bevy_web_server::{SendMsg, Connections};

use crate::Message;

pub fn hello_server(connections:Res<Connections>, mut send_writer:EventWriter<SendMsg<Message>>) {
    for (id, _) in connections.connections.iter() {
        send_writer.send(SendMsg { connection: id.clone(), msg: Message::Hello("Hello from Server".to_string()) });
    }
}