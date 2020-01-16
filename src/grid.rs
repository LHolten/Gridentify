use array_init;

pub type Board = [u32; 25];

pub type Action = Vec<usize>;

pub enum ActionValidation {
    TooShort,
    OutOfBoard,
    ValueConflict,
}

#[derive(Copy, Clone)]
pub struct State {
    pub(crate) board: Board,
    pub(crate) score: u32,
}

impl State {
    pub(crate) fn valid_moves(&self) -> Vec<Action> {
        let neighbours_of = self.get_neighbours();
        let mut moves = Vec::new();

        for i in 0..25 {
            Self::find_extensions(&mut moves, &neighbours_of, &vec![i])
        }
        moves
    }

    fn find_extensions(moves: &mut Vec<Action>, neighbours_of: &[Vec<usize>; 25], action: &Action) {
        for neighbour in neighbours_of[*action.last().unwrap()].iter() {
            if !action.contains(neighbour) {
                let mut branch = action.clone();

                branch.push(*neighbour);

                Self::find_extensions(moves, neighbours_of, &branch);

                moves.push(branch);
            }
        }
    }

    pub(crate) fn validate_move(&self, action: &Action) -> Result<(), ActionValidation> {
        if action.len() < 2 {
            return Err(ActionValidation::TooShort);
        }
        let action_value = self
            .board
            .get(action[0])
            .ok_or(ActionValidation::OutOfBoard)?;
        for tile in action[1..].iter() {
            let value = self.board.get(*tile).ok_or(ActionValidation::OutOfBoard)?;
            if value != action_value {
                return Err(ActionValidation::ValueConflict);
            }
        }
        Ok(())
    }

    pub(crate) fn get_neighbours(&self) -> [Vec<usize>; 25] {
        array_init::array_init(|i| {
            let value = self.board[i];
            let mut neighbours = Vec::new();
            let x = i % 5;
            let y = i / 5;
            if x < 4 && self.board[i + 1] == value {
                neighbours.push(i + 1)
            }
            if y < 4 && self.board[i + 5] == value {
                neighbours.push(i + 5)
            }
            if x > 0 && self.board[i - 1] == value {
                neighbours.push(i - 1)
            }
            if y > 0 && self.board[i - 5] == value {
                neighbours.push(i - 5)
            }
            neighbours
        })
    }

    pub(crate) fn is_game_over(&self) -> bool {
        for i in 0..self.board.len() {
            let value = self.board[i];
            let x = i % 5;
            let y = i / 5;
            if x < 4 && self.board[i + 1] == value {
                return false;
            }
            if y < 4 && self.board[i + 5] == value {
                return false;
            }
        }
        true
    }

    pub(crate) fn show_board(&self) {
        for i in 0..5 {
            println!("{:?}", &self.board[i * 5..i * 5 + 5]);
        }
    }
}

pub trait Gridentify {
    fn make_move(&mut self, action: Action);
}

pub fn show_move(action: Action) {
    let mut board = [0; 25];
    for (order, tile) in action.iter().enumerate() {
        board[*tile] = order + 1;
    }
    for i in 0..5 {
        println!("{:?}", &board[i * 5..i * 5 + 5]);
    }
}
