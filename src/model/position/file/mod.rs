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
        static FILES: [File; 8] = [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ];
        FILES.iter()
    }

    pub fn from(file: char) -> Option<Self> {
        match file {
            'A' | 'a' => Some(File::A),
            'B' | 'b' => Some(File::B),
            'C' | 'c' => Some(File::C),
            'D' | 'd' => Some(File::D),
            'E' | 'e' => Some(File::E),
            'F' | 'f' => Some(File::F),
            'G' | 'g' => Some(File::G),
            'H' | 'h' => Some(File::H),
            _ => {
                error!("Can not convert '{}' to a file.", file);
                None
            }
        }
    }
}
