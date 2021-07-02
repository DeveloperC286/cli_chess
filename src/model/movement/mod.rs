use serde::Serialize;

use crate::model::piece::class::Class;
use crate::model::position::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub struct Movement {
    pub class: Option<Class>,
    pub destination: Position,
}

impl Movement {
    pub fn from(movement: &str) -> Option<Self> {
        match movement.len() {
            3 => {
                let mut characters = movement.chars();
                let character = characters.next().unwrap();
                let position = characters.collect::<String>();

                if let Some(class) = Class::from(character) {
                    if let Some(destination) = Position::from(&position) {
                        return Some(Movement {
                            class: Some(class),
                            destination,
                        });
                    }
                }
            }
            2 => {
                if let Some(destination) = Position::from(movement) {
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
}

#[cfg(test)]
mod tests;
