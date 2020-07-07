use file::{to_file, File};
use rank::{to_rank, Rank};
use serde::Serialize;

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
            let file_option = to_file(position.chars().nth(0).unwrap());
            let rank_option = to_rank(position.chars().nth(1).unwrap());

            match file_option {
                Some(file) => match rank_option {
                    Some(rank) => {
                        return Some(Position::new(file, rank));
                    }
                    None => {}
                },
                None => {}
            }
        }
        _ => {
            error!("Position must be of length 2 '{}'.", position);
        }
    }

    return None;
}
