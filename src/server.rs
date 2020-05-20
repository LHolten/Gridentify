use crate::action::Action;
use crate::connection::JsonConnection;
use crate::database::{get_high_scores, insert_high_score};
use crate::high_score::HighScore;
use crate::local::Local;
use native_tls::{TlsAcceptor, TlsStream};
use ratelimit_meter::algorithms::NonConformance;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use std::io::{Error, ErrorKind, Result};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::{thread, time};
use tungstenite::{accept, WebSocket};

pub(crate) fn handle_connection_score<T: JsonConnection>(mut stream: T) -> Result<()> {
    stream.set_nodelay(true).unwrap();

    let scores = get_high_scores();

    stream.send(&scores)
}

pub(crate) fn handle_connection<T: JsonConnection>(mut stream: T) -> Result<()> {
    stream.set_nodelay(true).unwrap();

    let nickname: String = stream.receive()?;
    println!("{:?}", nickname);

    let mut grid = Local::new(rand::thread_rng());

    loop {
        stream.send(&grid.state.board)?;

        if grid.state.is_game_over() {
            insert_high_score(HighScore {
                name: nickname,
                score: grid.state.score,
            });
            return Ok(());
        }

        let action: Action = stream.receive()?;

        if grid.state.validate_action(action.as_slice()).is_err() {
            return Err(Error::new(ErrorKind::InvalidData, "wrong move"));
        }

        grid.make_move(action.as_slice())
    }
}

pub(crate) fn web_socket_wrapper(
    acceptor: Arc<TlsAcceptor>,
    func: impl Fn(WebSocket<TlsStream<TcpStream>>) -> Result<()>,
) -> impl Fn(TcpStream) -> Result<()> {
    move |stream: TcpStream| {
        let stream = acceptor.accept(stream).or_else(|_| {
            Err(Error::new(
                ErrorKind::ConnectionRefused,
                "could not connect",
            ))
        })?;
        let web_socket = accept(stream).or_else(|_| {
            Err(Error::new(
                ErrorKind::ConnectionRefused,
                "could not connect",
            ))
        })?;
        func(web_socket)
    }
}

pub(crate) fn listen_port(
    port: &str,
    handler: impl Fn(TcpStream) -> Result<()> + Send + Sync + 'static,
    rate_limiter: KeyedRateLimiter<IpAddr, GCRA>,
) {
    let listener = TcpListener::bind(port).unwrap();
    let handler = Arc::new(handler);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            if let Ok(address) = stream.peer_addr() {
                println!("new Client!");

                let address = address.ip();
                let handler_clone = handler.clone();
                let mut shared_limiter = rate_limiter.clone();
                thread::spawn(move || {
                    while match shared_limiter.check(address) {
                        Ok(()) => false,
                        Err(failed) => {
                            let jitter = time::Duration::from_millis(rand::random::<u64>() % 100);
                            thread::sleep(
                                failed.earliest_possible() - time::Instant::now() + jitter,
                            );
                            true
                        }
                    } {}
                    handler_clone(stream)
                });
            }
        }
    }
}
