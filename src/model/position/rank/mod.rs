use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Rank {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl Rank {
    pub fn iter() -> Iter<'static, Self> {
        [
            Rank::_1,
            Rank::_2,
            Rank::_3,
            Rank::_4,
            Rank::_5,
            Rank::_6,
            Rank::_7,
            Rank::_8,
        ]
        .iter()
    }

    pub fn from(rank: char) -> Option<Self> {
        match rank {
            '1' => Some(Rank::_1),
            '2' => Some(Rank::_2),
            '3' => Some(Rank::_3),
            '4' => Some(Rank::_4),
            '5' => Some(Rank::_5),
            '6' => Some(Rank::_6),
            '7' => Some(Rank::_7),
            '8' => Some(Rank::_8),
            _ => {
                error!("Can not convert '{}' to a rank.", rank);
                None
            }
        }
    }
}
