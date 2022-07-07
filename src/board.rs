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
        Ok(())
    }
}
