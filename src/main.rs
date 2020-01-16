use crate::database::create_database;
use crate::server::{handle_connection, handle_connection_score, listen_port, web_socket_wrapper};
use std::thread;

mod connection;
mod database;
mod grid;
mod local;
mod server;

fn main() {
    create_database();

    thread::spawn(|| listen_port("0.0.0.0:32123", handle_connection));
    thread::spawn(|| listen_port("0.0.0.0:12321", handle_connection_score));
    thread::spawn(|| listen_port("localhost:21212", web_socket_wrapper(handle_connection)));
    listen_port(
        "localhost:12121",
        web_socket_wrapper(handle_connection_score),
    );
}
