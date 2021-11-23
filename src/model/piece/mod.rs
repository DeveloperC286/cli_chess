use crate::model::piece::class::Class;
use crate::model::piece::colour::Colour;

pub mod class;
pub mod colour;

#[derive(Debug, Clone)]
pub struct Piece {
    pub class: Class,
    pub colour: Colour,
}

impl Piece {
    pub fn new(class: Class, colour: Colour) -> Piece {
        Piece { class, colour }
    }

    pub fn get_character(&self) -> char {
        let character = match self.class {
            Class::King => 'k',
            Class::Queen => 'q',
            Class::Rook => 'r',
            Class::Knight => 'n',
            Class::Bishop => 'b',
            Class::Pawn => 'p',
        };

        match self.colour {
            Colour::White => character.to_uppercase().next().unwrap(),
            Colour::Black => character,
        }
    }
}
