use crate::action::Action;
use crate::random::Random;
use crate::state::State;

#[derive(Copy, Clone)]
pub(crate) struct Local<R: Random> {
    random: R,
    pub(crate) state: State,
}

impl<R: Random> Local<R> {
    pub(crate) fn new(mut random: R) -> Self {
        Self {
            state: State {
                score: 0,
                board: random.new_board(),
            },
            random,
        }
    }

    pub(crate) fn make_move(&mut self, action: Action) {
        self.state.board[*action.last().unwrap()] *= action.len() as u32;
        for &tile in action[..action.len() - 1].iter() {
            self.state.board[tile] = self.random.new_num();
        }

        self.state.score += self.state.board[*action.last().unwrap()];
    }
}
