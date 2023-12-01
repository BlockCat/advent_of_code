use std::ops::Add;
use std::ops::AddAssign;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector2(pub isize, pub isize);

impl Add<Direction> for Vector2 {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        self + match other {
            Direction::North => Vector2(0, -1),
            Direction::East => Vector2(1, 0),
            Direction::South => Vector2(0, 1),
            Direction::West => Vector2(-1, 0)            
        }
    }
}

impl AddAssign<Direction> for Vector2 {    

    fn add_assign(&mut self, other: Direction) {
        *self = *self + match other {
            Direction::North => Vector2(0, -1),
            Direction::East => Vector2(1, 0),
            Direction::South => Vector2(0, 1),
            Direction::West => Vector2(-1, 0)            
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North, East, South, West
}

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