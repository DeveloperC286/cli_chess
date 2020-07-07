use serde::Serialize;
use std::slice::Iter;

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
    pub fn iterator() -> Iter<'static, File> {
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
}

pub fn to_file(file: char) -> Option<File> {
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
            return None;
        }
    }
}
