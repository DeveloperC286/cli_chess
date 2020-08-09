use serde::Serialize;

use file::{to_file, File};
use rank::{to_rank, Rank};

pub mod file;
pub mod rank;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Position {
        Position { rank, file }
    }
}

pub fn get_position(position: &str) -> Option<Position> {
    match position.len() {
        2 => {
            let mut characters = position.chars();
            let file_option = to_file(characters.next().unwrap());
            let rank_option = to_rank(characters.next().unwrap());

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
