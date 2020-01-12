use crate::grid::{Action, Board, Gridentify};

#[derive(Copy, Clone)]
pub struct LocalGridentify {
    seed: u64,
    score: u64,
    board: Board,
}

impl LocalGridentify {
    pub fn new(mut seed: u64) -> Self {
        Self {
            seed,
            score: 0,
            board: Self::new_board(&mut seed),
        }
    }

    fn new_board(seed: &mut u64) -> Board {
        array_init::array_init(|_| LocalGridentify::new_num(seed))
    }

    fn new_num(seed: &mut u64) -> u32 {
        let e = (16807 * *seed) % 1924421567;
        *seed = if e > 0 { e } else { e + 3229763266 };
        ((e % 3) + 1) as u32
    }
}

impl Gridentify for LocalGridentify {
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
            self.board[tile] = LocalGridentify::new_num(&mut self.seed);
        }

        self.score += self.board[*action.last().unwrap()] as u64;
    }
}
