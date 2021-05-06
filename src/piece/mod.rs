use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub enum Class {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

pub fn get_class(character: char) -> Option<Class> {
    match character {
        'k' | 'K' => Some(Class::King),
        'q' | 'Q' => Some(Class::Queen),
        'r' | 'R' => Some(Class::Rook),
        'n' | 'N' => Some(Class::Knight),
        'b' | 'B' => Some(Class::Bishop),
        'p' | 'P' => Some(Class::Pawn),
        _ => None,
    }
}

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
pub enum Colour {
    Black,
    White,
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
