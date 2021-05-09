use std::slice::Iter;

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn iter() -> Iter<'static, Self> {
        [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ]
        .iter()
    }

    pub fn from(file: char) -> Option<Self> {
        match file.to_ascii_lowercase() {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => {
                error!("Can not convert '{}' to a file.", file);
                None
            }
        }
    }
}
