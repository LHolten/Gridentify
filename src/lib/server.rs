use crate::lib::action::Action;
use crate::lib::connection::JsonConnection;
use crate::lib::database::{get_high_scores, insert_high_score};
use crate::lib::high_score::HighScore;
use crate::lib::local::Local;
use native_tls::{TlsAcceptor, TlsStream};
use ratelimit_meter::algorithms::NonConformance;
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use simple_error::bail;
use simple_error::SimpleResult;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::{thread, time};
use tungstenite::{accept, WebSocket};

pub fn handle_connection_score<T: JsonConnection>(mut stream: T) {
    stream.set_nodelay(true).unwrap();

    let scores = get_high_scores();

    let _ = stream.send(&scores);
}

pub fn handle_connection_game<T: JsonConnection>(mut stream: T) {
    fn handle<T: JsonConnection>(stream: &mut T) -> SimpleResult<()> {
        stream.set_nodelay(true).unwrap();

        let nickname: String = stream.receive()?;
        if nickname.len() > 16 {
            bail!(format!("nickname too long: {}", nickname).as_str());
        }
        println!("playing: {}", nickname);

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
                bail!(format!("wrong move: {}", nickname).as_str());
            }

            grid.make_move(action.as_slice())
        }
    }

    if let Err(error) = handle(&mut stream) {
        // let _ = stream.send(&error.as_str());
        println!("{}", &error.as_str())
    }
}

pub fn web_socket_wrapper(
    acceptor: Arc<TlsAcceptor>,
    func: impl Fn(WebSocket<TlsStream<TcpStream>>),
) -> impl Fn(TcpStream) {
    move |stream: TcpStream| {
        if let Ok(tls_stream) = acceptor.accept(stream) {
            if let Ok(web_socket) = accept(tls_stream) {
                func(web_socket)
            }
        }
    }
}

pub fn listen_port(
    port: &str,
    handler: impl Fn(TcpStream) + Send + Sync + 'static,
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
                    handler_clone(stream);
                });
            }
        }
    }
}
