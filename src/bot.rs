use lib::action::Action;
use lib::local::Local;
use rand::Rng;
use std::cmp::max;

mod lib;

const GOOD_VALUES: [u32; 17] = [
    1, 2, 3, 6, 12, 24, 48, 96, 192, 384, 768, 1536, 3072, 6144, 12288, 24578, 49152,
];

fn main() {
    let mut local = Local::new(123);

    let mut actions = filtered_actions(&local);
    while !actions.is_empty() {
        let mut best = (Vec::new(), 0);

        for action in actions {
            let mut value = 0;
            for _ in 0..1000 {
                value = max(value, q(local, action.as_slice()));
            }
            if value > best.1 {
                best = (action, value);
            }
        }
        local.make_move(best.0.as_slice());
        local.state.show_board();

        actions = filtered_actions(&local);
    }

    println!("{:?}", local.state.score);
}

fn q(mut local: Local<u64>, action: &[usize]) -> u32 {
    local.make_move(action);
    v(local)
}

fn v(local: Local<u64>) -> u32 {
    let actions = filtered_actions(&local);

    if actions.is_empty() {
        return local.state.score;
    }

    let action = &actions[rand::thread_rng().gen_range(0, actions.len())];
    q(local, action)
}

fn filtered_actions(local: &Local<u64>) -> Vec<Action> {
    let actions = local.state.valid_actions();
    let mut filtered = Vec::new();
    for action in actions {
        let result = local.state.board[action[0]] * action.len() as u32;
        if GOOD_VALUES.contains(&result) {
            filtered.push(action)
        }
    }
    filtered
}
