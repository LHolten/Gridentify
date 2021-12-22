pub mod database;
pub mod server;

use arc_cell::ArcCell;
use database::create_database;
use log::{log, Level, LevelFilter};
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use rustls_pemfile::{read_all, read_one};
use server::{handle_connection_game, handle_connection_score, listen_port};
use simple_logger::SimpleLogger;
use std::io::BufReader;
use std::net::IpAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::{fs::File, time::Duration};
use tokio::{join, time::sleep};
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig};

fn get_config() -> ServerConfig {
    let chain = File::open("/etc/letsencrypt/live/server.lucasholten.com/fullchain.pem").unwrap();
    let key = File::open("/etc/letsencrypt/live/server.lucasholten.com/privkey.pem").unwrap();

    let chain = read_all(&mut BufReader::new(chain))
        .unwrap()
        .into_iter()
        .map(|item| match item {
            rustls_pemfile::Item::X509Certificate(cert) => Certificate(cert),
            _ => panic!("was expecting certs"),
        })
        .collect();

    let key = match read_one(&mut BufReader::new(key)).unwrap().unwrap() {
        rustls_pemfile::Item::RSAKey(key) => PrivateKey(key),
        rustls_pemfile::Item::PKCS8Key(key) => PrivateKey(key),
        _ => panic!("expected key"),
    };

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(chain, key)
        .expect("bad certificate/key")
}

async fn update_loop(config: &ArcCell<ServerConfig>) {
    loop {
        sleep(Duration::from_secs(100_000)).await;
        log!(Level::Info, "refreshing cert");
        config.set(Arc::new(get_config()));
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    create_database();
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let config = ArcCell::new(Arc::new(get_config()));

    let rate_limiter = KeyedRateLimiter::<IpAddr, GCRA>::per_second(NonZeroU32::new(1).unwrap());

    let game = listen_port(
        &config,
        "0.0.0.0:21212",
        handle_connection_game,
        rate_limiter.clone(),
    );
    let score = listen_port(
        &config,
        "0.0.0.0:12121",
        handle_connection_score,
        rate_limiter,
    );
    let update = update_loop(&config);

    join!(game, score, update);
}
