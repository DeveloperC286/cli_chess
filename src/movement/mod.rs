use std::iter::FromIterator;

use serde::Serialize;

use crate::model::class::*;
use crate::position::{get_position, Position};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub struct Movement {
    pub class: Option<Class>,
    pub destination: Position,
}

pub fn to_movement(movement: &str) -> Option<Movement> {
    match movement.len() {
        3 => {
            let mut characters = movement.chars();
            let character = characters.next().unwrap();
            let position = &String::from_iter(characters);
            if let Some(class) = Class::from(character) {
                if let Some(destination) = get_position(position) {
                    return Some(Movement {
                        class: Some(class),
                        destination,
                    });
                }
            }
        }
        2 => {
            if let Some(destination) = get_position(movement) {
                return Some(Movement {
                    class: None,
                    destination,
                });
            }
        }
        _ => {
            error!("Do not know how to handle movement '{}'.", movement);
        }
    }

    None
}

#[cfg(test)]
mod tests;
