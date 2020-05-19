use crate::database::create_database;
use crate::server::{handle_connection, handle_connection_score, listen_port, web_socket_wrapper};
use native_tls::{Identity, TlsAcceptor};
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::thread;

mod action;
mod connection;
mod database;
mod high_score;
mod local;
mod random;
mod server;
mod state;

fn main() {
    create_database();

    let mut file = File::open("certificate.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = Identity::from_pkcs12(&identity, "grid").unwrap();

    let acceptor = Arc::new(TlsAcceptor::new(identity).unwrap());
    let wrapped_connection = web_socket_wrapper(acceptor.clone(), handle_connection);
    let wrapped_connection_score = web_socket_wrapper(acceptor, handle_connection_score);

    let rate_limiter =
        KeyedRateLimiter::<SocketAddr, GCRA>::per_second(NonZeroU32::new(1).unwrap());
    let shared_limiter = rate_limiter.clone();
    thread::spawn(|| listen_port("0.0.0.0:32123", handle_connection, shared_limiter));
    let shared_limiter = rate_limiter.clone();
    thread::spawn(|| listen_port("0.0.0.0:12321", handle_connection_score, shared_limiter));
    let shared_limiter = rate_limiter.clone();
    thread::spawn(|| listen_port("0.0.0.0:21212", wrapped_connection, shared_limiter));
    listen_port("0.0.0.0:12121", wrapped_connection_score, rate_limiter);
}
