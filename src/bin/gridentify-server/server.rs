use crate::database::{get_high_scores, insert_high_score};
use gridentify::game::local::Local;
use gridentify::protocol::connection::JsonConnection;
use gridentify::protocol::high_score::HighScore;
use native_tls::{TlsAcceptor, TlsStream};
use ratelimit_meter::{KeyedRateLimiter, GCRA};
use simple_error::bail;
use simple_error::SimpleResult;
use std::net::{IpAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
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

            let action: Vec<usize> = stream.receive()?;

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
    mut rate_limiter: KeyedRateLimiter<IpAddr, GCRA>,
) {
    let listener = TcpListener::bind(port).unwrap();
    let handler = Arc::new(handler);

    for stream in listener.incoming().flatten() {
        if let Ok(address) = stream.peer_addr() {
            println!("new Client!");

            let address = address.ip();
            let handler_clone = handler.clone();
            if let Ok(()) = rate_limiter.check(address) {
                thread::spawn(move || {
                    handler_clone(stream);
                });
            }
        }
    }
}
