mod client;
mod connection;
mod grid;
mod local;
mod server;

use crate::client::ClientGridentify;
use std::thread;

fn main() {
    thread::spawn(server::start_server);
    let grid = ClientGridentify::new("localhost:32123", "hytak");
}
