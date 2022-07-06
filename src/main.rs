use chess_backend::{Board, Coord, Layouts};

fn main() {
    let mut board = Board::new(Layouts::standard());

    match board.move_piece(&Coord { x: 0, y: 1 }, &Coord { x: 0, y: 2 }) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    for rank in board.get_layout() {
        println!("{:?}", rank);
    }
}
