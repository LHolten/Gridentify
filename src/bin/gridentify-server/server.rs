use crate::database::{get_high_scores, insert_high_score};
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task::spawn;
use async_tungstenite::accept_async;
use async_tungstenite::WebSocketStream;
use futures_util::Future;
use gridentify::game::local::Local;
use gridentify::protocol::connection::JsonConnection;
use gridentify::protocol::high_score::HighScore;
use rand::rngs::OsRng;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use rustls_acme::TlsAcceptor;
use rustls_acme::TlsStream;
use simple_error::bail;
use simple_error::SimpleResult;
use std::net::IpAddr;

pub async fn handle_connection_score<T: JsonConnection>(mut stream: T) {
    let scores = get_high_scores();

    let _ = stream.send_serialize(&scores).await;
}

pub async fn handle_connection_game<T: JsonConnection>(mut stream: T) {
    async fn handle<T: JsonConnection>(stream: &mut T) -> SimpleResult<()> {
        let nickname: String = stream.receive_deserialize().await?;
        if nickname.len() > 16 {
            bail!(format!("nickname too long: {}", nickname).as_str());
        }
        println!("playing: {}", nickname);

        let mut grid = Local::new(OsRng);

        loop {
            stream.send_serialize(&grid.state.board).await?;

            if grid.state.is_game_over() {
                insert_high_score(HighScore {
                    name: nickname,
                    score: grid.state.score,
                });
                return Ok(());
            }

            let action: Vec<usize> = stream.receive_deserialize().await?;

            if grid.state.validate_action(action.as_slice()).is_err() {
                bail!(format!("wrong move: {}", nickname).as_str());
            }

            grid.make_move(action.as_slice())
        }
    }

    if let Err(error) = handle(&mut stream).await {
        // let _ = stream.send(&error.as_str());
        println!("{}", &error.as_str())
    }
}

pub async fn web_socket_wrapper<F: 'static + Send + Future<Output = ()>>(
    acceptor: TlsAcceptor,
    handler: impl 'static + Send + Sync + Copy + Fn(WebSocketStream<TlsStream>) -> F,
    stream: TcpStream,
) {
    if let Ok(Some(tls)) = acceptor.accept(stream).await {
        if let Ok(socket) = accept_async(tls).await {
            handler(socket).await
        }
    }
}

pub async fn listen_port<F: 'static + Send + Future<Output = ()>>(
    acceptor: TlsAcceptor,
    address: &str,
    handler: fn(WebSocketStream<TlsStream>) -> F,
    mut rate_limiter: KeyedRateLimiter<IpAddr, GCRA>,
) {
    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        if let Ok((stream, address)) = listener.accept().await {
            if rate_limiter.check(address.ip()).is_ok() {
                let _ = stream.set_nodelay(true);
                let _ = spawn(web_socket_wrapper(acceptor.clone(), handler, stream));
            }
        }
    }
}
