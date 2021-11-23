use crate::model::position::file::File;
use crate::model::position::rank::Rank;

pub mod file;
pub mod rank;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Self {
        Position { rank, file }
    }

    pub fn from(position: &str) -> Option<Self> {
        match position.len() {
            2 => {
                let mut characters = position.chars();

                if let Some(file) = File::from(characters.next().unwrap()) {
                    if let Some(rank) = Rank::from(characters.next().unwrap()) {
                        return Some(Position::new(file, rank));
                    }
                }
            }
            _ => {
                error!("Position must be of length 2 '{}'.", position);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests;
