use serde::Serialize;

use crate::model::class::Class;
use crate::model::colour::Colour;

pub fn get_character(piece: Piece) -> char {
    let character = match piece.class {
        Class::King => 'k',
        Class::Queen => 'q',
        Class::Rook => 'r',
        Class::Knight => 'n',
        Class::Bishop => 'b',
        Class::Pawn => 'p',
    };

    match piece.colour {
        Colour::White => character.to_uppercase().next().unwrap(),
        Colour::Black => character,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub struct Piece {
    pub class: Class,
    pub colour: Colour,
}

impl Piece {
    pub fn new(class: Class, colour: Colour) -> Piece {
        Piece { class, colour }
    }
}
