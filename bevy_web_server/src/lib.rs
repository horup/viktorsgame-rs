use bevy::prelude::*;
use bevy_web_client::Message;
use futures::{SinkExt, StreamExt};
use hyper_tungstenite::HyperWebsocket;
use uuid::Uuid;
use std::{collections::HashMap, marker::PhantomData, mem, sync::{Arc, Mutex}};

pub struct WebsocketConnection<T>  {
    rt:Arc<tokio::runtime::Runtime>,
    sender:tokio::sync::mpsc::Sender<T>,
    is_connected:bool,
    messages:Vec<T>
}
impl<T : Message> WebsocketConnection<T> {
    pub fn send(&mut self, msg:T) {
        if self.is_connected {
            let sender = self.sender.clone();
            self.rt.spawn(async move {
                let _ = sender.send(msg).await;
            });
        }
    }
}
pub struct WebServerConnectionManager<T> {
    rt:Arc<tokio::runtime::Runtime>,
    websocket_connections:HashMap<Uuid, WebsocketConnection<T>>
}
impl<T> WebServerConnectionManager<T> {
    pub fn new(rt:Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            rt,
            websocket_connections: Default::default(),
        }
    }
}

#[derive(Resource)]
struct WebServer<T> {
    rt:Arc<tokio::runtime::Runtime>,
    connection_manager:Arc<Mutex<WebServerConnectionManager<T>>>,
}


pub struct Connection {
    pub id:Uuid
}

#[derive(Resource)]
pub struct Connections {
    pub connections:HashMap<Uuid, Connection>
}

impl<T> WebServer<T> {
    pub fn connections(&self) -> Vec<Uuid> {
        let mut connections = Vec::default();
        for (uuid, _) in self.connection_manager.lock().unwrap().websocket_connections.iter() {
            connections.push(uuid.clone());
        }
        connections
    }
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn serve_websocket<T : Message>(connection_manager:Arc<Mutex<WebServerConnectionManager<T>>>, websocket:HyperWebsocket) {
    if let Ok(websocket) = websocket.await {
        let (mut sink, mut stream) = websocket.split();
        let uuid = uuid::Uuid::new_v4();
        {
            // insert new connection
            let (sender, mut receiver) = tokio::sync::mpsc::channel::<T>(100);
            let mut connection_manager = connection_manager.lock().expect("could not acquire mutex in serve_websocket");
            let rt = connection_manager.rt.clone();
            connection_manager.websocket_connections.insert(uuid, crate::WebsocketConnection { sender, is_connected:true, messages:Vec::with_capacity(64), rt });

            // wait for messages and send them through the websocket
            tokio::spawn(async move {
                println!("{} Connected", uuid);
                while let Some(msg) = receiver.recv().await {
                    let Ok(bytes) = bincode::serialize(&msg) else { break; };
                    if sink.send(hyper_tungstenite::tungstenite::Message::Binary(bytes)).await.is_err() {
                        break;
                    }
                }

                let _ = sink.close().await;
            });
            
        }

        while let Some(message) = stream.next().await {
            let Ok(message) = message else { break; };
            match message {
                hyper_tungstenite::tungstenite::Message::Binary(bytes)=> {
                    let mut connection_manager = connection_manager.lock().expect("could not acquire mutex in serve_websocket");
                    let Ok(msg) = bincode::deserialize::<T>(&bytes) else {break;};
                    connection_manager.websocket_connections.get_mut(&uuid).expect("failed to get WebSocketConnection").messages.push(msg);
                },
                _=>{}
            }
        }

        let mut connection_manager = connection_manager.lock().expect("could not acquire mutex in serve_websocket");
        connection_manager.websocket_connections.get_mut(&uuid).expect("failed to get WebSocketConnection").is_connected = false;
        println!("{} Disconnected", uuid);
    }
}


async fn handle_http_request<T : Message>(connection_manager:Arc<Mutex<WebServerConnectionManager<T>>>, mut request: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<http_body_util::Full<hyper::body::Bytes>>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        match hyper_tungstenite::upgrade(&mut request, None) {
            Ok((response, websocket)) => {
                tokio::spawn(async move {
                    serve_websocket(connection_manager, websocket).await;
                });
                return Ok(response);
            },
            Err(err) => {
                return Err(Box::new(err));
            },
        }
    } else {
        Ok(hyper::Response::new(http_body_util::Full::new(hyper::body::Bytes::new())))
    }
}

fn start_webserver<T: Message>(webserver:ResMut<WebServer<T>>) {
    let connection_manager = webserver.connection_manager.clone();
    webserver.rt.spawn(async move {
        let addr:std::net::SocketAddr = "0.0.0.0:8080".parse().expect("could not parse address");
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("could not bind to address");
        let mut http = hyper::server::conn::http1::Builder::new();
        http.keep_alive(true);
        loop {
            let Ok((stream, _)) = listener.accept().await else { continue; };
            let connection_manager = connection_manager.clone();
            let connection = http
            .serve_connection(hyper_util::rt::TokioIo::new(stream), hyper::service::service_fn(move |request: hyper::Request<hyper::body::Incoming>| {
                handle_http_request(connection_manager.clone(), request)
            }))
            .with_upgrades();
            tokio::spawn(async move {
                if let Err(err) = connection.await {
                    println!("Error serving HTTP connection: {err:?}");
                }
            });
        }
    });
}

fn check_connections<T: Message>(webserver:ResMut<WebServer<T>>, mut connections:ResMut<Connections>) {
    let conn_manager = webserver.connection_manager.lock().expect("could not lock ConnectionManager");
    for (id, conn) in conn_manager.websocket_connections.iter() {
        if conn.is_connected == false {
            connections.connections.remove(id);
        } else {
            connections.connections.insert(id.clone(), Connection {
                id: id.clone(),
            });
        }
    }
}

fn recv_messages<T: Message>(webserver:ResMut<WebServer<T>>, mut recv_writer:EventWriter<RecvMsg<T>>) {
    let mut conn_manager = webserver.connection_manager.lock().expect("could not lock ConnectionManager");
    for (uuid, conn) in conn_manager.websocket_connections.iter_mut() {
        for msg in conn.messages.drain(..) {
            recv_writer.send(RecvMsg {
                connection:uuid.clone(),
                msg
            });
            dbg!("msg recv");
        }
    }
}

fn send_messages<T: Message>(webserver:ResMut<WebServer<T>>, mut send_writer:EventReader<SendMsg<T>>) {
    let mut conn_manager = webserver.connection_manager.lock().expect("could not lock ConnectionManager");
    for send in send_writer.read() {
        if let Some(conn) = conn_manager.websocket_connections.get_mut(&send.connection) {
            conn.send(send.msg.clone());
        }
    }
}

#[derive(Event)]
pub struct SendMsg<T : Message> {
    pub connection:Uuid,
    pub msg:T
}

#[derive(Event)]
pub struct RecvMsg<T : Message> {
    pub connection:Uuid,
    pub msg:T
}

pub struct BevyWebServerPlugin<T> {
    pub phantom:PhantomData<T>
}

impl<T> BevyWebServerPlugin<T> {
    pub fn new() -> Self {
        Self {
            phantom:PhantomData::default()
        }
    }
}

impl<T : Message> Plugin for BevyWebServerPlugin<T> {
    fn build(&self, app: &mut App) {
        let rt = Arc::new(tokio::runtime::Runtime::new().expect("failed to create runtime"));

        app.add_event::<SendMsg<T>>();
        app.add_event::<RecvMsg<T>>();

        app.insert_resource(Connections {
           connections:Default::default() 
        });
        app.insert_resource::<WebServer<T>>(WebServer {
            rt:rt.clone(),
            connection_manager:Arc::new(std::sync::Mutex::new(WebServerConnectionManager::new(rt.clone())))
        });

        app.add_systems(Startup, start_webserver::<T>);
        app.add_systems(First, (check_connections::<T>, recv_messages::<T>));
        app.add_systems(Last, send_messages::<T>);
    }
}