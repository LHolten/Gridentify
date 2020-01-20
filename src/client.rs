use crate::action::Action;
use crate::connection::JsonConnection;
use crate::high_score::HighScore;
use crate::state::State;
use std::net::TcpStream;

pub(crate) struct Client {
    stream: TcpStream,
    pub(crate) state: State,
}

impl Client {
    pub(crate) fn new(host: &str, nickname: &str) -> Self {
        let mut stream = TcpStream::connect(host).unwrap();
        stream.set_nodelay(true).unwrap();

        stream.send(&nickname).unwrap();
        let board = stream.receive().unwrap();

        Self {
            stream,
            state: State { score: 0, board },
        }
    }

    pub(crate) fn make_move(&mut self, action: Action) {
        self.stream.send(&action).unwrap();

        self.state.board = self.stream.receive().unwrap();

        self.state.score += self.state.board[*action.last().unwrap()];
    }
}

pub(crate) fn get_scores(host: &str) -> Vec<HighScore> {
    let mut stream = TcpStream::connect(host).unwrap();
    stream.set_nodelay(true).unwrap();

    stream.receive().unwrap()
}
