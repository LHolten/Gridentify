use lib::action::Action;
use lib::client::Client;
use lib::local::Local;
use lib::lucid::ActionData;
use lib::lucid::CreateActions;
use lib::lucid::LucidState;
use lib::state::{Board, State};
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs::read_to_string;

mod lib;

fn main() {
    let mut local = Local::new(123);

    let data = ActionData::create();
    println!("created data with size {}", data.len());

    while !local.state.is_game_over() {
        let lucid = LucidState::create(&local.state, &data);
        local.make_move(&lucid.best_action());
        local.state.show_board();
        println!("----------");
    }

    println!("{:?}", local.state.score);
}

fn v(state: &State, prob: usize) -> (Option<Action>, f32) {
    let actions = state.actions();
    if actions.len() == 0 {
        return (None, state.score as f32 / 3f32.powi(prob as i32));
    }
    if prob >= 3 {
        return (
            None,
            state_value(state, actions.len()) as f32 / 3f32.powi(prob as i32),
        );
    }
    let value = actions
        .into_iter()
        .map(|action| {
            let new_prob = prob + action.len() - 1;
            let states = state.next_states(&action);
            let value: f32 = states.iter().map(|s| v(s, new_prob).1).sum();
            (Some(action), value)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal))
        .unwrap();
    value
}

fn state_value(state: &State, num_actions: usize) -> u32 {
    fn value_tile(board: &Board, c: usize) -> u32 {
        let x = c % 5;
        let y = c / 5;
        let mut largest = 0;
        let mut index = 0;
        for &(t, v) in &[
            (x != 0, c - 1),
            (x != 4, c + 1),
            (y != 0, c - 5),
            (y != 4, c + 5),
        ] {
            if t && board[v] > largest && board[v] <= board[c] && board[v] % 3 == 0 {
                largest = board[v];
                index = v;
            }
        }
        return if largest != 0 {
            if largest != board[c] {
                board[c] + value_tile(board, index)
            } else {
                board[c] * 2
            }
        } else {
            board[c]
        };
    }

    let mut sum = 0;
    for &c in &[0, 4, 20, 24] {
        if state.board[c] % 3 == 0 {
            sum += value_tile(&state.board, c)
        }
    }
    sum * num_actions as u32 + state.score
}
