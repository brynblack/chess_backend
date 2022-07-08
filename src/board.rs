pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum Colour {
    Black,
    White,
}

#[derive(Debug)]
pub enum Square {
    Empty,
    Bishop(Colour),
    King(Colour),
    Knight(Colour),
    Pawn(Colour),
    Queen(Colour),
    Rook(Colour),
}

#[derive(Debug)]
pub struct Board {
    layout: Vec<Vec<Square>>,
}

impl Board {
    pub fn new(layout: Vec<Vec<Square>>) -> Board {
        Board { layout }
    }

    pub fn get_layout(&self) -> &Vec<Vec<Square>> {
        &self.layout
    }

    pub fn move_piece(&mut self, old_pos: &Position, new_pos: &Position) -> Result<(), &str> {
        let old_rank = match self.layout.get_mut(old_pos.y) {
            Some(rank) => rank,
            None => return Err("Out of bounds access!"),
        };
        let old_square = match old_rank.get_mut(old_pos.x) {
            Some(square) => square,
            None => return Err("Out of bounds access!"),
        };
        let moved_piece = std::mem::replace(old_square, Square::Empty);
        self.layout[new_pos.y][new_pos.x] = moved_piece;
        Ok(())
    }
}
