pub mod database;
pub mod server;

use async_rustls::rustls::{NoClientAuth, ServerConfig};
use async_std::task::{self, spawn};
use database::create_database;
use log::LevelFilter;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use rustls_acme::{acme, ResolvesServerCertUsingAcme, TlsAcceptor};
use server::{handle_connection_game, handle_connection_score, listen_port};
use simple_logger::SimpleLogger;
use std::net::IpAddr;
use std::num::NonZeroU32;

fn main() {
    create_database();
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let resolver = ResolvesServerCertUsingAcme::new();
    let config = ServerConfig::new(NoClientAuth::new());
    let acceptor = TlsAcceptor::new(config, resolver.clone());

    let rate_limiter = KeyedRateLimiter::<IpAddr, GCRA>::per_second(NonZeroU32::new(1).unwrap());

    let _ = spawn(listen_port(
        acceptor.clone(),
        "0.0.0.0:21212",
        handle_connection_game,
        rate_limiter.clone(),
    ));
    let _ = spawn(listen_port(
        acceptor,
        "0.0.0.0:12121",
        handle_connection_score,
        rate_limiter,
    ));

    task::block_on(resolver.run(
        acme::LETS_ENCRYPT_STAGING_DIRECTORY,
        vec!["server.lucasholten.com".to_string()],
        Some("cache"),
    ));
}
