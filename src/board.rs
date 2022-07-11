use std::mem;

/// A position on a chessboard.
#[derive(Debug)]
pub struct Position {
    /// The x-coordinate.
    pub x: usize,
    /// The y-coordinate.
    pub y: usize,
}

/// An enum defining all the possible colours of a chess piece.
#[derive(Debug, PartialEq)]
pub enum Colour {
    /// The colour black.
    Black,
    /// The colour white.
    White,
}

/// An enum defining the state of a square on a chessboard.
#[derive(Debug, PartialEq)]
pub enum Square {
    /// An empty square.
    Empty,
    /// A Bishop.
    Bishop(Colour),
    /// A King.
    King(Colour),
    /// A Knight.
    Knight(Colour),
    /// A Pawn.
    Pawn(Colour),
    /// A Queen.
    Queen(Colour),
    /// A Rook.
    Rook(Colour),
}

impl Square {
    /// Returns an optional value containing a reference to the colour of the piece.
    pub fn get_colour(&self) -> Option<&Colour> {
        match self {
            Square::Empty => None,
            Square::Bishop(colour) => Some(colour),
            Square::King(colour) => Some(colour),
            Square::Knight(colour) => Some(colour),
            Square::Pawn(colour) => Some(colour),
            Square::Queen(colour) => Some(colour),
            Square::Rook(colour) => Some(colour),
        }
    }
}

/// A chessboard.
#[derive(Debug)]
pub struct Board {
    layout: Vec<Vec<Square>>,
    player: Colour,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            layout: Layouts::standard(),
            player: Colour::White,
        }
    }
}

impl Board {
    /// Creates a new chessboard with the given layout and initial player.
    pub fn new(layout: Vec<Vec<Square>>, player: Colour) -> Self {
        Self { layout, player }
    }

    /// Returns a reference to the current state of the chessboard.
    pub fn get_layout(&self) -> &Vec<Vec<Square>> {
        &self.layout
    }

    /// Returns a reference to the current player.
    pub fn get_player(&self) -> &Colour {
        &self.player
    }

    /// Moves a piece from one place to another on the chessboard.
    pub fn move_piece(&mut self, old_pos: &Position, new_pos: &Position) -> Result<(), &str> {
        if let Err(err) = self.is_valid_move(old_pos, new_pos) {
            return Err(err);
        }
        let moved_piece = mem::replace(&mut self.layout[old_pos.y][old_pos.x], Square::Empty);
        self.layout[new_pos.y][new_pos.x] = moved_piece;
        self.next_turn();
        Ok(())
    }

    /// Returns a result on whether or not a move is valid.
    fn is_valid_move<'a>(&self, old_pos: &Position, new_pos: &Position) -> Result<(), &'a str> {
        // Bounds checks here...
        // This must always go first, as later on, the [] operator is used to access squares, which can panic if out of bounds.
        if self.layout.get(old_pos.y).is_none() {
            return Err("Error: Origin square is out of bounds!");
        }
        if self.layout[old_pos.y].get(old_pos.x).is_none() {
            return Err("Error: Origin square is out of bounds!");
        }
        if self.layout.get(new_pos.y).is_none() {
            return Err("Error: Destination square is out of bounds!");
        }
        if self.layout[new_pos.y].get(new_pos.x).is_none() {
            return Err("Error: Destination square is out of bounds!");
        }

        let old_square = &self.layout[old_pos.y][old_pos.x];
        let new_square = &self.layout[new_pos.y][new_pos.x];

        // Empty square check
        if let Square::Empty = old_square {
            return Err("Error: An empty square cannot be moved!");
        }

        // Not your piece check...
        if old_square.get_colour().unwrap() != &self.player {
            return Err("Error: You cannot move your opponent's pieces!");
        }

        // Trying to destroy your own pieces check...
        if let Some(new_square) = new_square.get_colour() {
            if old_square.get_colour().unwrap() == new_square {
                return Err(
                    "Error: You cannot move your own piece onto another one of your pieces!",
                );
            }
        };

        // Valid piece move checks here...

        Ok(())
    }

    /// Switches to the next player.
    fn next_turn(&mut self) {
        self.player = match self.player {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black,
        }
    }
}

/// Pre-made layouts ready to use for creating a new chessboard.
pub struct Layouts;

impl Layouts {
    /// Returns the standard chessboard layout.
    pub fn standard() -> Vec<Vec<Square>> {
        vec![
            vec![
                Square::Rook(Colour::Black),
                Square::Knight(Colour::Black),
                Square::Bishop(Colour::Black),
                Square::Queen(Colour::Black),
                Square::King(Colour::Black),
                Square::Bishop(Colour::Black),
                Square::Knight(Colour::Black),
                Square::Rook(Colour::Black),
            ],
            vec![
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
            ],
            vec![
                Square::Rook(Colour::White),
                Square::Knight(Colour::White),
                Square::Bishop(Colour::White),
                Square::Queen(Colour::White),
                Square::King(Colour::White),
                Square::Bishop(Colour::White),
                Square::Knight(Colour::White),
                Square::Rook(Colour::White),
            ],
        ]
    }
}
