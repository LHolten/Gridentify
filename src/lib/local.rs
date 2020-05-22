use crate::lib::random::Random;
use crate::lib::state::State;

#[derive(Clone)]
pub struct Local<R: Random> {
    random: R,
    pub state: State,
}

impl<R: Random> Local<R> {
    pub fn new(mut random: R) -> Self {
        Self {
            state: State {
                score: 0,
                board: random.new_board(),
            },
            random,
        }
    }

    pub fn make_move(&mut self, action: &[usize]) {
        self.state.board[*action.last().unwrap()] *= action.len() as u32;
        for &tile in action[..action.len() - 1].iter() {
            self.state.board[tile] = self.random.new_num();
        }

        self.state.score += self.state.board[*action.last().unwrap()];
    }
}
