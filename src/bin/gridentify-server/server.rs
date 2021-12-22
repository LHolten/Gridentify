use crate::database::{get_high_scores, insert_high_score};
use arc_cell::ArcCell;
use futures_util::Future;
use gridentify::game::local::Local;
use gridentify::protocol::connection::{receive_deserialize, send_serialize, MyStream};
use gridentify::protocol::high_score::HighScore;
use log::{log, Level};
use rand::rngs::OsRng;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use simple_error::bail;
use simple_error::SimpleResult;
use std::net::IpAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::accept_async;

pub async fn handle_connection_score(mut stream: MyStream) {
    let scores = get_high_scores();

    let _ = send_serialize(&mut stream, &scores).await;
}

pub async fn handle_connection_game(mut stream: MyStream) {
    async fn handle(stream: &mut MyStream) -> SimpleResult<()> {
        let nickname: String = receive_deserialize(stream).await?;
        if nickname.len() > 16 {
            bail!(format!("nickname too long: {}", nickname).as_str());
        }
        log!(Level::Warn, "playing: {}", nickname);

        let mut grid = Local::new(OsRng);

        loop {
            send_serialize(stream, &grid.state.board).await?;

            if grid.state.is_game_over() {
                insert_high_score(HighScore {
                    name: nickname,
                    score: grid.state.score,
                });
                return Ok(());
            }

            let action: Vec<usize> = receive_deserialize(stream).await?;

            if grid.state.validate_action(action.as_slice()).is_err() {
                bail!(format!("wrong move: {}", nickname).as_str());
            }

            grid.make_move(action.as_slice())
        }
    }

    if let Err(error) = handle(&mut stream).await {
        log!(Level::Warn, "{}", &error.as_str());
        let _ = send_serialize(&mut stream, &error.as_str()).await;
    }
}

pub async fn web_socket_wrapper<F: 'static + Send + Future<Output = ()>>(
    acceptor: TlsAcceptor,
    handler: impl 'static + Send + Sync + Copy + Fn(MyStream) -> F,
    stream: TcpStream,
) {
    if let Ok(tls) = acceptor.accept(stream).await {
        if let Ok(socket) = accept_async(tls).await {
            handler(socket).await;
            log!(Level::Info, "dropping client")
        } else {
            log!(Level::Warn, "failed websocket handshake")
        }
    } else {
        log!(Level::Warn, "failed tls handshake")
    }
}

pub async fn listen_port<F: 'static + Send + Future<Output = ()>>(
    config: &ArcCell<ServerConfig>,
    address: &str,
    handler: fn(MyStream) -> F,
    mut rate_limiter: KeyedRateLimiter<IpAddr, GCRA>,
) {
    let listener = TcpListener::bind(address).await.unwrap();
    log!(Level::Info, "listening on address {}", address);

    loop {
        if let Ok((stream, address)) = listener.accept().await {
            if rate_limiter.check(address.ip()).is_ok() {
                log!(Level::Info, "new client with address {}", address.ip());
                let _ = stream.set_nodelay(true);
                let _ = spawn(web_socket_wrapper(config.get().into(), handler, stream));
            } else {
                log!(Level::Warn, "client got ratelimited {}", address.ip())
            }
        }
    }
}
