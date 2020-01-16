use crate::grid::{Action, Board, Gridentify};
use crate::random::Random;

#[derive(Copy, Clone)]
pub(crate) struct LocalGridentify<R: Random> {
    random: R,
    score: u64,
    board: Board,
}

impl<R: Random> LocalGridentify<R> {
    pub(crate) fn new(mut random: R) -> Self {
        Self {
            score: 0,
            board: random.new_board(),
            random,
        }
    }
}

impl<R: Random> Gridentify for LocalGridentify<R> {
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
        self.board_mut()[*action.last().unwrap()] *= action.len() as u32;
        for &tile in action[..action.len() - 1].iter() {
            self.board[tile] = self.random.new_num();
        }

        self.score += self.board[*action.last().unwrap()] as u64;
    }
}
