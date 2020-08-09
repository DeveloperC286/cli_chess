use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub enum Class {
    KING,
    QUEEN,
    ROOK,
    KNIGHT,
    BISHOP,
    PAWN,
}

pub fn get_class(character: char) -> Option<Class> {
    match character {
        'k' | 'K' => Some(Class::KING),
        'q' | 'Q' => Some(Class::QUEEN),
        'r' | 'R' => Some(Class::ROOK),
        'n' | 'N' => Some(Class::KNIGHT),
        'b' | 'B' => Some(Class::BISHOP),
        'p' | 'P' => Some(Class::PAWN),
        _ => None,
    }
}

pub fn get_character(piece: Piece) -> char {
    let character = match piece.class {
        Class::KING => 'k',
        Class::QUEEN => 'q',
        Class::ROOK => 'r',
        Class::KNIGHT => 'n',
        Class::BISHOP => 'b',
        Class::PAWN => 'p',
    };

    match piece.colour {
        Colour::WHITE => character.to_uppercase().next().unwrap(),
        Colour::BLACK => character,
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub enum Colour {
    BLACK,
    WHITE,
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
