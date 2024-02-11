use bevy::prelude::*;
use client::ClientPlugin;

fn run_server() {
    App::new()
    .add_plugins(server::ServerPlugin)
    .run()
}

fn run_client() {
    App::new()
    .add_plugins(ClientPlugin)
    .run();
}

fn main() {
    let _ = std::thread::Builder::new().name("server thread".to_owned()).spawn(run_server).expect("could not spawn server");
    run_client();
    //server.join().unwrap();
}

fn hello_world_system() {
    println!("hello world");
}