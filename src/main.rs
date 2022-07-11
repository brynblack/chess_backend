use chess_backend::board::{Board, Position};

fn main() {
    let mut board = Board::default();

    if let Err(err) = board.move_piece(&Position { x: 0, y: 6 }, &Position { x: 0, y: 5 }) {
        eprintln!("{}", err);
    }

    for rank in board.get_layout() {
        println!("{:?}", rank);
    }

    println!("Current player: {:?}", board.get_player());

    println!("Example coordinate: {:?}", Position { x: 0, y: 0 });
}
