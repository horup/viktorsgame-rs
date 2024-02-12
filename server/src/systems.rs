use std::{future::Future, process::Output, sync::{Arc, Mutex}};

use bevy::prelude::*;
use crate::{ConnectionManager, WebServer};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
async fn handle_request(connection_manager:Arc<Mutex<ConnectionManager>>, mut request: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<http_body_util::Full<hyper::body::Bytes>>, Error> {
    if hyper_tungstenite::is_upgrade_request(&request) {
        //let Ok((response, websocket)) = hyper_tungstenite::upgrade(&mut request, None) else { return Err(Box::new("hehe".to_owned()).into()) };

        match hyper_tungstenite::upgrade(&mut request, None) {
            Ok((response, websocket)) => {
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