use crate::connection::JsonConnection;
use crate::grid::{Action, Board, Gridentify};
use std::net::TcpStream;

pub struct ClientGridentify {
    stream: TcpStream,
    score: u64,
    board: Board,
}

impl ClientGridentify {
    pub fn new(host: &str, nickname: &str) -> Self {
        let mut stream = TcpStream::connect(host).unwrap();
        stream.set_nodelay(true).unwrap();

        stream.send(&nickname).unwrap();
        let board = stream.receive().unwrap();

        Self {
            stream,
            score: 0,
            board,
        }
    }
}

impl Gridentify for ClientGridentify {
    fn board_mut(&mut self) -> &mut [u32; 25] {
        &mut self.board
    }

    fn board(&self) -> &[u32; 25] {
        &self.board
    }

    fn score_mut(&mut self) -> &mut u64 {
        &mut self.score
    }

    fn score(&self) -> &u64 {
        &self.score
    }

    fn make_move(&mut self, action: Action) {
        self.stream.send(&action).unwrap();

        self.board = self.stream.receive().unwrap();

        self.score += self.board[*action.last().unwrap()] as u64;
    }
}

pub fn get_scores(host: &str) -> Vec<(String, u32)> {
    let mut stream = TcpStream::connect(host).unwrap();
    stream.set_nodelay(true).unwrap();

    stream.receive().unwrap()
}
