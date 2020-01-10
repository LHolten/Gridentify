use rand::Rng;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod grid;

use crate::grid::Action;
use grid::Gridentify;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
                thread::spawn(|| handle_connection(stream));
            }
            Err(_) => {}
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut data = Vec::new();
    BufReader::new(&stream).read_until(b'\n', &mut data)?;

    let nickname: String = serde_json::from_slice(data.as_slice())?;

    let mut grid = Gridentify::new(rand::thread_rng().gen());

    loop {
        stream.write(serde_json::to_vec(&grid.board)?.as_slice())?;

        let moves = grid.valid_moves();
        if moves.len() == 0 {
            return Ok(handle_high_score(&nickname, grid.score));
        }

        data.clear();
        BufReader::new(&stream).read_until(b'\n', &mut data)?;

        let action: Action = serde_json::from_slice(data.as_slice())?;

        if !moves.contains(&action) {
            return Err(Error::new(ErrorKind::Other, "wrong move"));
        }

        grid.make_move(action)
    }
}

fn handle_high_score(name: &str, score: u64) {
    println!("{:?} got {:?}", name, score);
}
