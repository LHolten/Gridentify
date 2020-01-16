use crate::connection::JsonConnection;
use crate::grid::{Action, Gridentify, State};
use std::net::TcpStream;

pub struct ClientGridentify {
    stream: TcpStream,
    pub(crate) state: State,
}

impl ClientGridentify {
    pub fn new(host: &str, nickname: &str) -> Self {
        let mut stream = TcpStream::connect(host).unwrap();
        stream.set_nodelay(true).unwrap();

        stream.send(&nickname).unwrap();
        let board = stream.receive().unwrap();

        Self {
            stream,
            state: State { score: 0, board },
        }
    }
}

impl Gridentify for ClientGridentify {
    fn make_move(&mut self, action: Action) {
        self.stream.send(&action).unwrap();

        self.state.board = self.stream.receive().unwrap();

        self.state.score += self.state.board[*action.last().unwrap()] as u64;
    }
}

pub fn get_scores(host: &str) -> Vec<(String, u32)> {
    let mut stream = TcpStream::connect(host).unwrap();
    stream.set_nodelay(true).unwrap();

    stream.receive().unwrap()
}
