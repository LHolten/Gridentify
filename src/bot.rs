use lib::action::Action;
use lib::local::Local;
use lib::state::{Board, State};
use std::cmp::Ordering::Equal;
use std::collections::HashMap;

mod lib;

fn main() {
    let mut local = Local::new(123);

    loop {
        if let Some(action) = v(&local.state, 0).0 {
            local.make_move(&action);
            local.state.show_board();
            println!("----------");
            continue;
        }
        break;
    }

    println!("{:?}", local.state.score);
}

fn v(state: &State, prob: usize) -> (Option<Action>, f32) {
    let actions = state.valid_actions();
    if prob >= 3 || actions.len() == 0 {
        return (None, state.score as f32 / 3f32.powi(prob as i32));
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

const GOOD_VALUES: [u32; 17] = [
    1, 2, 3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288, 24578, 49152,
];

fn filtered_actions(state: &State) -> Vec<Action> {
    state
        .valid_actions()
        .into_iter()
        .filter(|action| {
            let result = state.board[action[0]] * action.len() as u32;
            GOOD_VALUES.contains(&result)
        })
        .take(4)
        .collect()
}
