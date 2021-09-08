use super::connection::JsonConnection;
use super::high_score::HighScore;
use crate::game::state::State;
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
    pub state: State,
}

// impl Client {
//     pub async fn new(host: &str, nickname: &str) -> Self {
//         let mut stream = TcpStream::connect(host).unwrap();
//         stream.set_nodelay(true).unwrap();

//         stream.send_serialize(&nickname).await.unwrap();
//         let board = stream.receive().unwrap();

//         Self {
//             stream,
//             state: State { score: 0, board },
//         }
//     }

//     pub async fn make_move(&mut self, action: &[usize]) {
//         self.stream.send_serialize(&action).await.unwrap();

//         self.state.board = self.stream.receive().unwrap();

//         self.state.score += self.state.board[*action.last().unwrap()];
//     }
// }

// pub fn get_scores(host: &str) -> Vec<HighScore> {
//     let mut stream = TcpStream::connect(host).unwrap();
//     stream.set_nodelay(true).unwrap();

//     stream.receive().unwrap()
// }
