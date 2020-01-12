use crate::connection;
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

        connection::send(&nickname, &mut stream).unwrap();
        let board = connection::receive(&mut stream).unwrap();

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
        connection::send(&action, &mut self.stream).unwrap();

        self.board = connection::receive(&mut self.stream).unwrap();

        self.score += self.board[*action.last().unwrap()] as u64;
    }
}
