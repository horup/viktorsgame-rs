use std::{future::Future, process::Output, sync::{Arc, Mutex}};
//use futures::sink::SinkExt;

use bevy::prelude::*;
use futures::{channel::oneshot::channel, SinkExt, StreamExt};
use hyper_tungstenite::{tungstenite::Message, HyperWebsocket};
use crate::{ConnectionManager, WebServer};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn serve_websocket(connection_manager:Arc<Mutex<ConnectionManager>>, websocket:HyperWebsocket) {
    if let Ok(websocket) = websocket.await {
        // client connected
        let (mut sink, mut stream) = websocket.split();
        let uuid = uuid::Uuid::new_v4();
        {
            let (sender, mut receiver) = tokio::sync::mpsc::channel::<Vec<u8>>(100);
            let mut connection_manager = connection_manager.lock().expect("could not acquire mutex in serve_websocket");
            connection_manager.websocket_connections.insert(uuid, crate::WebsocketConnection { sender, is_connected:true });
            tokio::spawn(async move {
                while let Some(msg) = receiver.recv().await {
                    let res = sink.send(Message::Binary(msg)).await;
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


async fn handle_request(connection_manager:Arc<Mutex<ConnectionManager>>, mut request: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<http_body_util::Full<hyper::body::Bytes>>, Error> {
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

// https://docs.rs/hyper-tungstenite/latest/hyper_tungstenite/
// https://users.rust-lang.org/t/how-to-share-stuct-data-as-state-using-hyper-server/42819
pub fn start_web_server(webserver:ResMut<WebServer>) {
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

pub fn hello_server() {
    println!("hello server");
}