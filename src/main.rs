use chess_framework::board::{Board, Position};

fn main() {
    let mut board = Board::default();

    if let Err(err) = board.move_piece(&Position { x: 0, y: 6 }, &Position { x: 0, y: 5 }) {
        eprintln!("{}", err);
    }

    for rank in board.get_layout() {
        for square in rank {
            println!("{:?}", square);
        }
    }

    println!("Current player: {:?}", board.get_player());

    println!("Example coordinate: {:?}", Position { x: 0, y: 0 });
}
