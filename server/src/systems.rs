use bevy::ecs::{event::EventWriter, system::Res};
use bevy_webserver::{SendMsg, WebServer};

use crate::Message;

pub fn hello_server(webserver:Res<WebServer<Message>>, mut send_writer:EventWriter<SendMsg<Message>>) {
    for conn in webserver.connections() {
        send_writer.send(SendMsg { connection: conn, msg: Message::Hello("Hello from Server".to_string()) });
    }
}