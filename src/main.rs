mod client;
mod connection;
mod grid;
mod local;
mod server;

use crate::client::ClientGridentify;
use crate::grid::Gridentify;
use std::thread;

fn main() {
    let handle = thread::spawn(server::main);
    let mut grid = ClientGridentify::new("localhost:32123", "hyta k");

    while !grid.is_game_over() {
        let actions = grid.valid_moves();
        let action = actions.first().unwrap();
        grid.make_move(action.clone());
    }

    handle.join();
    //    server::main();
}
