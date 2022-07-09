use chess_backend::board::{Board, Position};

fn main() {
    let mut board = Board::default();

    if let Err(err) = board.move_piece(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }) {
        eprintln!("{}", err);
    }

    for rank in board.get_layout() {
        println!("{:?}", rank);
    }
}
