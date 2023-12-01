#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West
];

macro_rules! from_number {
    ($te:ident) => {
        impl From<$te> for Direction {
            fn from(code: $te) -> Self {
                match code {
                    0 => Direction::North,
                    1 => Direction::South,
                    2 => Direction::West,
                    3 => Direction::East,
                    _ => panic!(),
                }
            }
        }
    };
}

from_number!(u8);
from_number!(u16);
from_number!(u32);
from_number!(u64);
from_number!(u128);
from_number!(usize);
from_number!(i8);
from_number!(i16);
from_number!(i32);
from_number!(i64);
from_number!(i128);
from_number!(isize);

impl Direction {
    pub fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    pub fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}
