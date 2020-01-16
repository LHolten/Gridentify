use crate::grid::{Action, Gridentify, State};
use crate::random::Random;

#[derive(Copy, Clone)]
pub(crate) struct LocalGridentify<R: Random> {
    random: R,
    pub(crate) state: State,
}

impl<R: Random> LocalGridentify<R> {
    pub(crate) fn new(mut random: R) -> Self {
        Self {
            state: State {
                score: 0,
                board: random.new_board(),
            },
            random,
        }
    }
}

impl<R: Random> Gridentify for LocalGridentify<R> {
    fn make_move(&mut self, action: Action) {
        self.state.board[*action.last().unwrap()] *= action.len() as u32;
        for &tile in action[..action.len() - 1].iter() {
            self.state.board[tile] = self.random.new_num();
        }

        self.state.score += self.state.board[*action.last().unwrap()];
    }
}
