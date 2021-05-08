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

impl Class {
    pub fn from(character: char) -> Option<Self> {
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
}