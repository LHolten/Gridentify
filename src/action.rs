pub type Action = Vec<usize>;

pub fn show_action(action: Action) {
    let mut board = [0; 25];
    for (order, tile) in action.iter().enumerate() {
        board[*tile] = order + 1;
    }
    for i in 0..5 {
        println!("{:?}", &board[i * 5..i * 5 + 5]);
    }
}
