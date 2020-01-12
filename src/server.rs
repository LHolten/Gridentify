use crate::connection;
use crate::grid::{Action, Gridentify};
use crate::local::LocalGridentify;
use rand::Rng;
use rusqlite::{params, Connection};
use std::io::{Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub(crate) fn main() {
    let conn = Connection::open("scores.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS scores (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    name        TEXT NOT NULL,
                    score       UNSIGNED BIG INT NOT NULL
                    )",
        params![],
    )
    .unwrap();
    drop(conn);

    let listener = TcpListener::bind("localhost:32123").unwrap();

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
    stream.set_nodelay(true).unwrap();

    let nickname: String = connection::receive(&mut stream)?;
    println!("{:?}", nickname);

    let mut grid = LocalGridentify::new(rand::thread_rng().gen::<u32>() as u64);

    loop {
        connection::send(&grid.board(), &mut stream)?;

        if grid.is_game_over() {
            return Ok(handle_high_score(&nickname, grid.score()));
        }

        let action: Action = connection::receive(&mut stream)?;

        if grid.validate_move(&action).is_err() {
            return Err(Error::new(ErrorKind::InvalidData, "wrong move"));
        }

        grid.make_move(action)
    }
}

fn handle_high_score(name: &str, score: &u64) {
    let conn = Connection::open("./scores.db").unwrap();
    conn.execute(
        "INSERT INTO scores (name, score) VALUES (?1, ?2)",
        params![name, *score as u32],
    )
    .unwrap();

    println!("{:?} got {:?}", name, score);
}
