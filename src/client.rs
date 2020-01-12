use crate::connection;
use crate::grid::{Action, Board, Gridentify};
use std::net::TcpStream;

pub struct ClientGridentify {
    stream: TcpStream,
    score: u64,
    board: Board,
    data: Vec<u8>,
}

impl ClientGridentify {
    pub fn new(host: &str, nickname: &str) -> Self {
        let mut stream = TcpStream::connect(host).unwrap();
        stream.set_nodelay(true).unwrap();

        let mut data = Vec::new();

        connection::send(&nickname, &mut stream).unwrap();
        let board = connection::receive(&mut data, &mut stream).unwrap();

        println!("{:?}", board);

        Self {
            stream,
            score: 0,
            board,
            data,
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
        connection::send(&action, &mut self.stream);

        self.board_mut()[*action.last().unwrap()] *= action.len() as u32;

        let numbers = connection::receive::<Vec<u32>>(&mut self.data, &mut self.stream).unwrap();
        for (tile, value) in action[..action.len() - 1].iter().zip(numbers.iter()) {
            self.board[*tile] = *value;
        }

        self.score += self.board[*action.last().unwrap()] as u64;
    }
}
