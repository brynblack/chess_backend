use chess_backend::{Board, Layouts, Position};

fn main() {
    let mut board = Board::new(Layouts::standard());

    match board.move_piece(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    for rank in board.get_layout() {
        println!("{:?}", rank);
    }
}