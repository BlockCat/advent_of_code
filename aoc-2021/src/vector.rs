use std::ops::{Add, AddAssign, Index, Mul, Sub, SubAssign};

use crate::direction::Direction;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VectorN<const N: usize> {
    pub value: [isize; N],
}

pub type Vector2 = VectorN<2>;
pub type Vector3 = VectorN<3>;
impl<const N: usize> VectorN<N> {
    pub fn length_sq(&self) -> isize {
        self.value.iter().map(|x| x * x).product()
    }

    pub fn manhattan(left: &Self, right: &Self) -> usize {
        left.value
            .iter()
            .zip(right.value.iter())
            .map(|(l, r)| (l - r).abs() as usize)
            .sum()
    }

    pub fn new(value: [isize; N]) -> Self {
        VectorN { value }
    }

    pub fn abs(&self) -> Self {
        let mut d = self.clone();
        d.value.iter_mut().for_each(|x| *x = x.abs());

        d
    }

    pub fn zero() -> Self {
        VectorN { value: [0; N] }
    }
}

impl<const N: usize> Index<usize> for VectorN<N> {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl<const N: usize> Add for VectorN<N> {
    type Output = VectorN<N>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.value
            .iter_mut()
            .zip(rhs.value.iter())
            .for_each(|(l, r)| {
                *l += r;
            });
        self
    }
}

impl<const N: usize> AddAssign for VectorN<N> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const N: usize> Mul<isize> for VectorN<N> {
    type Output = VectorN<N>;

    fn mul(mut self, rhs: isize) -> Self::Output {
        self.value.iter_mut().for_each(|l| {
            *l *= rhs;
        });
        self
    }
}

impl<const N: usize> Sub for VectorN<N> {
    type Output = VectorN<N>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.value
            .iter_mut()
            .zip(rhs.value.iter())
            .for_each(|(l, r)| {
                *l -= r;
            });
        self
    }
}

impl<const N: usize> SubAssign for VectorN<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<[[isize; 2]; 2]> for Vector2 {
    type Output = Vector2;

    // [0, 1] * [[1, 0], [0, 1]]
    fn mul(self, rhs: [[isize; 2]; 2]) -> Self::Output {
        Vector2::new([
            self[0] * rhs[0][0] + self[1] * rhs[1][0],
            self[0] * rhs[0][1] + self[1] * rhs[1][1],
        ])
    }
}

impl Add<Direction> for Vector2 {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        self + match other {
            Direction::North => Vector2::new([0, -1]),
            Direction::East => Vector2::new([1, 0]),
            Direction::South => Vector2::new([0, 1]),
            Direction::West => Vector2::new([-1, 0]),
        }
    }
}

impl AddAssign<Direction> for Vector2 {
    fn add_assign(&mut self, other: Direction) {
        *self = *self + other;
    }
}

impl Sub<Direction> for Vector2 {
    type Output = Self;

    fn sub(self, other: Direction) -> Self {
        self - match other {
            Direction::North => Vector2::new([0, -1]),
            Direction::East => Vector2::new([1, 0]),
            Direction::South => Vector2::new([0, 1]),
            Direction::West => Vector2::new([-1, 0]),
        }
    }
}

impl SubAssign<Direction> for Vector2 {
    fn sub_assign(&mut self, other: Direction) {
        *self = *self - other;
    }
}
