use bevy::{prelude::*, reflect::erased_serde::Serialize};
use futures::StreamExt;
use hyper_tungstenite::HyperWebsocket;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use std::{collections::HashMap, marker::PhantomData, sync::{Arc, Mutex}};

pub trait Message : Send + Sync + Serialize + DeserializeOwned + 'static {}
impl<T> Message for T where T : Send + Sync + Serialize + DeserializeOwned + 'static {}

pub struct WebsocketConnection<T>  {
    pub sender:tokio::sync::mpsc::Sender<T>,
    pub is_connected:bool
}
pub struct ConnectionManager<T> {
    pub websocket_connections:HashMap<Uuid, WebsocketConnection<T>>
}
impl<T : Send + Sync> Default for ConnectionManager<T> {
    fn default() -> Self {
        Self { websocket_connections: Default::default() }
    }
}

#[derive(Resource)]
pub struct WebServer<T> {
    pub rt:tokio::runtime::Runtime,
    pub connection_manager:Arc<Mutex<ConnectionManager<T>>>,
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn serve_websocket<T : Send + Sync + 'static>(connection_manager:Arc<Mutex<ConnectionManager<T>>>, websocket:HyperWebsocket) {
    if let Ok(websocket) = websocket.await {
        // client connected
        let (mut sink, mut stream) = websocket.split();
        let uuid = uuid::Uuid::new_v4();
        {
            let (sender, mut receiver) = tokio::sync::mpsc::channel::<T>(100);
            let mut connection_manager = connection_manager.lock().expect("could not acquire mutex in serve_websocket");
            connection_manager.websocket_connections.insert(uuid, crate::WebsocketConnection { sender, is_connected:true });
            tokio::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    //let res = sink.send(Message::Binary(msg)).await;
                }
            });
        }
        while let Some(message) = stream.next().await {
            let Ok(message) = message else { break; };
            match message {
                _=> {
                    dbg!("message recev");
                }
            }
        }
    }

    // client disconnect
}


async fn handle_request<T : Send + Sync + 'static>(connection_manager:Arc<Mutex<ConnectionManager<T>>>, mut request: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<http_body_util::Full<hyper::body::Bytes>>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        //let Ok((response, websocket)) = hyper_tungstenite::upgrade(&mut request, None) else { return Err(Box::new("hehe".to_owned()).into()) };

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

        // Return the response so the spawned future can continue.
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
                handle_request(connection_manager.clone(), request)
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

pub struct BevyWebserver<T> {
    pub phantom:PhantomData<T>
}

impl<T> BevyWebserver<T> {
    pub fn new() -> Self {
        Self {
            phantom:PhantomData::default()
        }
    }
}


impl<T : Message> Plugin for BevyWebserver<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_webserver::<T>);
        app.insert_resource::<WebServer<T>>(WebServer {
            rt:tokio::runtime::Runtime::new().expect("failed to create runtime"),
            connection_manager:Arc::new(std::sync::Mutex::new(Default::default()))
        });
    }
}