mod client;
mod connection;
mod grid;
mod local;
mod server;

use crate::client::ClientGridentify;
use crate::grid::Gridentify;
use std::thread;

fn main() {
    thread::spawn(server::start_server);
    let mut grid = ClientGridentify::new("localhost:32123", "hytak");

    while !grid.is_game_over() {
        let actions = grid.valid_moves();
        let action = actions.first().unwrap();
        grid.make_move(action.clone());
    }
}
