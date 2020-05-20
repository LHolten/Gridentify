use crate::lib::action::Action;
use array_init;

pub type Board = [u32; 25];

pub enum ActionValidation {
    TooShort,
    OutOfBoard,
    ValueConflict,
    NotNextToEachOther,
    AlreadyGotten,
}

#[derive(Copy, Clone)]
pub struct State {
    pub board: Board,
    pub score: u32,
}

impl State {
    pub fn valid_actions(&self) -> Vec<Action> {
        let neighbours_of = self.get_neighbours();
        let mut moves = Vec::new();

        for i in 0..25 {
            Self::find_extensions(&mut moves, &neighbours_of, &[i])
        }
        moves
    }

    fn find_extensions(
        moves: &mut Vec<Action>,
        neighbours_of: &[Vec<usize>; 25],
        action: &[usize],
    ) {
        for neighbour in neighbours_of[*action.last().unwrap()].iter() {
            if !action.contains(neighbour) {
                let mut branch = action.to_owned();

                branch.push(*neighbour);

                Self::find_extensions(moves, neighbours_of, &branch);

                moves.push(branch);
            }
        }
    }

    pub(crate) fn validate_action(&self, action: &[usize]) -> Result<(), ActionValidation> {
        if action.len() < 2 {
            return Err(ActionValidation::TooShort);
        }
        let action_value = self
            .board
            .get(action[0])
            .ok_or(ActionValidation::OutOfBoard)?;
        let mut coords = [action[0]].to_vec();
        for &tile in action[1..].iter() {
            let value = self.board.get(tile).ok_or(ActionValidation::OutOfBoard)?;
            if value != action_value {
                return Err(ActionValidation::ValueConflict);
            }
            if coords.contains(&tile) {
                return Err(ActionValidation::AlreadyGotten);
            }
            let x = tile % 5;
            let y = tile / 5;
            let prev_x = coords.last().unwrap().clone() % 5;
            let prev_y = coords.last().unwrap().clone() / 5;
            if prev_x.max(x) + prev_y.max(y) - prev_x.min(x) - prev_y.min(y) != 1 {
                return Err(ActionValidation::NotNextToEachOther);
            }
            coords.push(tile);
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

    pub fn show_board(&self) {
        for i in 0..5 {
            println!("{:?}", &self.board[i * 5..i * 5 + 5]);
        }
    }
}
