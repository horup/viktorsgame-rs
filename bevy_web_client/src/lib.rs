use bevy::prelude::*;
use ewebsock::{WsReceiver, WsSender};
use std::{marker::PhantomData, sync::Mutex};

use serde::{Serialize, de::DeserializeOwned};
pub trait Message : Send + Sync + Clone + Serialize + DeserializeOwned + 'static {}
impl<T> Message for T where T : Send + Sync + Clone + Serialize + DeserializeOwned + 'static {}

#[derive(Resource)]
pub struct WebClientInfo {
    pub url:String,
    pub is_connected:bool
}

#[derive(Event)]
pub struct SendPacket<T:Message> {
    pub msg:T
}

#[derive(Event)]
pub struct RecvPacket<T:Message> {
    pub msg:T
}



struct WebSocket {
    pub sender:WsSender,
    pub receiver:WsReceiver
}

#[derive(Resource)]
struct Client {
    pub url:String,
    pub socket:Option<Mutex<WebSocket>>
}


fn recv_messages<T:Message>(mut info:ResMut<WebClientInfo>, mut client:ResMut<Client>, mut recv_writer:EventWriter<RecvPacket<T>>) {
    if info.url != client.url {
        client.socket = None;
    }
    if client.socket.is_none() {
        let url = info.url.clone();
        let (sender, receiver) = ewebsock::connect(&url).unwrap();
        client.url = url.clone();
        client.socket = Some(Mutex::new(WebSocket {
            sender,
            receiver,
        }));
    }
    let mut recreate = false;
    {
        let Some(socket) = &client.socket else { return };
        let socket = socket.lock().expect("could not lock socket");
        while let Some(msg) = socket.receiver.try_recv() {
            match msg {
                ewebsock::WsEvent::Opened => {
                    info.is_connected = true;
                },
                ewebsock::WsEvent::Message(msg) => {
                    match msg {
                        ewebsock::WsMessage::Binary(bytes) => {
                            let Ok(msg) = bincode::deserialize::<T>(&bytes) else { break; };
                            recv_writer.send(RecvPacket { msg: msg });
                        },
                        _=>{}
                    }
                },
                ewebsock::WsEvent::Error(_) => {
                    recreate = true;
                    break;
                },
                ewebsock::WsEvent::Closed => {
                    recreate = true;
                    break;
                },
            }
        }
    }
    if recreate {
        info.is_connected = false;
        client.socket = None;
    }
}

fn send_messages<T:Message>(mut send_writer:EventReader<SendPacket<T>>, client:ResMut<Client>) {
    let mut messages = Vec::with_capacity(64);
    for msg in send_writer.read() {
        messages.push(&msg.msg);
    }

    let Some(socket) = &client.socket else { return; };
    let mut socket = socket.lock().expect("could not lock socket");
    for msg in messages.drain(..) {
        let Ok(bytes) = bincode::serialize(&msg) else { continue; };
        socket.sender.send(ewebsock::WsMessage::Binary(bytes));
    }
}

pub struct BevyWebClientPlugin<T> {
    phantom:PhantomData<T>
}
impl<T> BevyWebClientPlugin<T> {
    pub fn new() -> Self {
        Self {
            phantom:PhantomData::default()
        }
    }
}
impl<T:Message> Plugin for BevyWebClientPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<SendPacket<T>>();
        app.add_event::<RecvPacket<T>>();
        app.insert_resource(WebClientInfo {
            url:"ws://localhost:8080".to_string(),
            is_connected:false
        });
        app.insert_resource(Client {
            url:Default::default(),
            socket:None
        });
        app.add_systems(First, recv_messages::<T>);
        app.add_systems(Last, send_messages::<T>);
    }
}