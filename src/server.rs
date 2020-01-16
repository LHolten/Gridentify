use crate::connection::JsonConnection;
use crate::database::{get_high_scores, insert_high_score};
use crate::grid::{Action, Gridentify};
use crate::local::LocalGridentify;
use rand::Rng;
use std::io::{Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::thread;
use tungstenite::{accept, WebSocket};

pub(crate) fn handle_connection_score<T: JsonConnection>(mut stream: T) -> Result<(), Error> {
    stream.set_nodelay(true).unwrap();

    let scores = get_high_scores();

    stream.send(&scores)
}

pub(crate) fn handle_connection<T: JsonConnection>(mut stream: T) -> Result<(), Error> {
    stream.set_nodelay(true).unwrap();

    let nickname: String = stream.receive()?;
    println!("{:?}", nickname);

    let mut grid = LocalGridentify::new(rand::thread_rng().gen::<u32>() as u64);

    loop {
        stream.send(&grid.board())?;

        if grid.is_game_over() {
            return Ok(insert_high_score(&nickname, grid.score()));
        }

        let action: Action = stream.receive()?;

        if grid.validate_move(&action).is_err() {
            return Err(Error::new(ErrorKind::InvalidData, "wrong move"));
        }

        grid.make_move(action)
    }
}

pub(crate) fn web_socket_wrapper(
    func: impl FnOnce(WebSocket<TcpStream>) -> Result<(), Error> + Copy + 'static,
) -> impl FnOnce(TcpStream) -> Result<(), Error> + Copy + 'static {
    move |stream: TcpStream| {
        let web_socket = accept(stream).unwrap();
        func(web_socket)
    }
}

pub(crate) fn listen_port(
    port: &str,
    handler: impl FnOnce(TcpStream) -> Result<(), Error> + Send + 'static + Copy + Sync,
) {
    let listener = TcpListener::bind(port).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
                thread::spawn(move || handler(stream));
            }
            Err(_) => {}
        }
    }
}
