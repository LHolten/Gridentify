use crate::lib::action::Action;
use array_init;
use rusqlite::version_number;

pub type Board = [u32; 25];

pub enum ActionValidation {
    TooShort,
    OutOfBoard,
    ValueConflict,
    NotNextToEachOther,
    AlreadyGotten,
}

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub score: u32,
}

impl State {
    pub fn valid_actions(&self) -> Vec<Action> {
        let neighbours_of = self.get_neighbours();
        let mut moves = Vec::new();

        fn find_extensions(
            moves: &mut Vec<Action>,
            neighbours_of: &[Vec<usize>; 25],
            action: &[usize],
            two: bool,
            three: bool,
            four: bool,
        ) {
            for neighbour in neighbours_of[*action.last().unwrap()].iter() {
                if !action.contains(neighbour) {
                    let mut branch = action.to_owned();

                    branch.push(*neighbour);

                    let len = branch.len();
                    if four && len < 4 || three && len < 3 {
                        find_extensions(moves, neighbours_of, &branch, two, three, four);
                    }
                    if two && len == 2 || three && len == 3 || four && len == 4 {
                        moves.push(branch);
                    }
                }
            }
        }

        for i in 0..25 {
            let three = self.board[i] % 3 != 0;
            let four = self.board[i] % 3 == 0;
            let two = four || self.board[i] < 4;
            find_extensions(&mut moves, &neighbours_of, &[i], two, three, four)
        }
        moves
    }

    pub(crate) fn validate_action(&self, action: &[usize]) -> Result<(), ActionValidation> {
        if action.len() < 2 {
            return Err(ActionValidation::TooShort);
        }
        let action_value = self
            .board
            .get(action[0])
            .ok_or(ActionValidation::OutOfBoard)?;
        let mut coords = vec![action[0]];
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

    pub fn next_states(&self, action: &Action) -> Vec<State> {
        let mut new_state = self.clone();
        let &last_index = action.last().unwrap();
        new_state.board[last_index] *= action.len() as u32;
        new_state.score += new_state.board[last_index];

        fn other_states(mut state: State, mut wildcards: &[usize]) -> Vec<State> {
            if wildcards.len() == 0 {
                return vec![state];
            }
            state.board[wildcards[0]] = 1;
            let mut result = other_states(state.clone(), &wildcards[1..]);
            state.board[wildcards[0]] = 2;
            result.append(&mut other_states(state.clone(), &wildcards[1..]));
            state.board[wildcards[0]] = 3;
            result.append(&mut other_states(state, &wildcards[1..]));
            result
        }

        other_states(new_state, &action[..action.len() - 1])
    }
}
