use crate::piece::{get_class, Class};
use crate::position::{get_position, Position};
use serde::Serialize;
use std::iter::FromIterator;

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
            match get_class(character) {
                Some(class) => match get_position(position) {
                    Some(destination) => {
                        return Some(Movement {
                            class: Some(class),
                            destination,
                        });
                    }
                    None => {}
                },
                None => {}
            }
        }
        2 => match get_position(movement) {
            Some(destination) => {
                return Some(Movement {
                    class: None,
                    destination,
                });
            }
            None => {}
        },
        _ => {
            error!("Do not know how to handle movement '{}'.", movement);
        }
    }

    return None;
}

#[cfg(test)]
mod tests;
