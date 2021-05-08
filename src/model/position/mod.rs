use serde::Serialize;

use crate::model::position::file::File;
use crate::model::position::rank::Rank;

pub mod file;
pub mod rank;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
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
                let file_option = File::from(characters.next().unwrap());
                let rank_option = Rank::from(characters.next().unwrap());

                if let Some(file) = file_option {
                    if let Some(rank) = rank_option {
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
