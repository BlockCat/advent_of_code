#![feature(min_const_generics)]

use std::ops::SubAssign;
use std::ops::{AddAssign, Mul};
use std::ops::{Index, Sub};
use std::{fmt::Debug, ops::Add};

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
        *self = *self
            + match other {
                Direction::North => Vector2::new([0, -1]),
                Direction::East => Vector2::new([1, 0]),
                Direction::South => Vector2::new([0, 1]),
                Direction::West => Vector2::new([-1, 0]),
            }
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
        *self = *self
            - match other {
                Direction::North => Vector2::new([0, -1]),
                Direction::East => Vector2::new([1, 0]),
                Direction::South => Vector2::new([0, 1]),
                Direction::West => Vector2::new([-1, 0]),
            }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<usize> for Direction {
    fn from(code: usize) -> Self {
        match code {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => panic!(),
        }
    }
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

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T>
where
    T: Clone + PartialEq + Eq + Default,
{
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            grid: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    pub fn from_vec(grid: Vec<Vec<T>>) -> Grid<T> {
        let width = grid[0].len();
        let height = grid.len();
        Grid {
            grid: grid.into_iter().flatten().collect(),
            width,
            height,
        }
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.grid.chunks(self.width).map(|x| Vec::from(x)).collect()
    }

    pub fn iter<'a>(&'a self) -> GridIterator<'a, T> {
        GridIterator {
            grid: &self,
            x: 0,
            y: 0,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.grid[x + y * self.width] = value;
    }

    pub fn set_vec(&mut self, pos: &Vector2, value: T) {
        self.set(pos[0] as usize, pos[1] as usize, value);
    }

    pub fn get_vec(&self, pos: &Vector2) -> Option<&T> {
        self.get(pos[0] as usize, pos[1] as usize)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.grid.get(x + y * self.width)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(x + y * self.width)
    }
}

pub struct GridIterator<'a, T>
where
    T: Clone + PartialEq + Eq + Default,
{
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    T: Clone + PartialEq + Eq + Default,
{
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get(self.x, self.y);

        if let Some(v) = val {
            let x = self.x;
            let y = self.y;

            self.x += 1;
            if self.x >= self.grid.width {
                self.x = 0;
                self.y += 1;
            }

            return Some(((x, y), v));
        } else {
            return None;
        }
    }
}

pub mod interval_tree {

    #[derive(Debug)]
    pub struct Interval<T> {
        pub value: T,
        pub start: usize,
        pub end: usize,
    }

    impl<T> Interval<T> {
        pub fn contains(&self, value: usize) -> bool {
            value >= self.start && value <= self.end
        }
    }

    #[derive(Debug)]
    pub struct IntervalTree<T> {
        center: usize,
        intersections: Vec<Interval<T>>,
        left: Option<Box<IntervalTree<T>>>,
        right: Option<Box<IntervalTree<T>>>,
    }

    impl<T> IntervalTree<T> {
        pub fn new(center: usize) -> IntervalTree<T> {
            IntervalTree {
                center,
                intersections: Vec::new(),
                left: None,
                right: None,
            }
        }

        pub fn add(&mut self, interval: Interval<T>) {
            if interval.end < self.center {
                if let Some(child) = &mut self.left {
                    child.as_mut().add(interval);
                } else {
                    let mut new_tree = IntervalTree {
                        center: (interval.start + interval.end) / 2,
                        intersections: Vec::new(),
                        left: None,
                        right: None,
                    };
                    new_tree.add(interval);
                    self.left = Some(Box::new(new_tree));
                }
            } else if interval.start > self.center {
                if let Some(child) = &mut self.right {
                    child.as_mut().add(interval);
                } else {
                    let mut new_tree = IntervalTree {
                        center: (interval.start + interval.end) / 2,
                        intersections: Vec::new(),
                        left: None,
                        right: None,
                    };
                    new_tree.add(interval);
                    self.right = Some(Box::new(new_tree));
                }
            } else {
                self.intersections.push(interval);
            }
        }

        pub fn intersecting<'a>(
            &'a self,
            value: usize,
        ) -> impl Iterator<Item = &'a Interval<T>> + 'a {
            let intersections = self.intersections.iter().filter(move |x| x.contains(value));

            let b: Box<dyn Iterator<Item = &Interval<T>>> = if value < self.center {
                if let Some(child) = &self.left {
                    Box::new(intersections.chain(child.intersecting(value)))
                } else {
                    Box::new(intersections)
                }
            } else if value > self.center {
                if let Some(child) = &self.right {
                    Box::new(intersections.chain(child.intersecting(value)))
                } else {
                    Box::new(intersections)
                }
            } else {
                Box::new(intersections)
            };
            return b;
        }
    }
}

pub mod intcode {
    use std::collections::VecDeque;

    #[derive(PartialEq, Eq, Debug)]
    enum ParamMode {
        Position,
        Immediate,
        Relative,
    }
    impl From<i32> for ParamMode {
        fn from(mode: i32) -> Self {
            match mode {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                2 => ParamMode::Relative,
                _ => unreachable!(),
            }
        }
    }
    impl From<i64> for ParamMode {
        fn from(mode: i64) -> Self {
            match mode {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                2 => ParamMode::Relative,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Clone)]
    pub struct Memory {
        input: Vec<i64>,
        overmem: hashbrown::HashMap<usize, i64>,
    }

    impl std::ops::Index<usize> for Memory {
        type Output = i64;
        fn index(&self, index: usize) -> &Self::Output {
            if index < self.input.len() {
                &self.input[index]
            } else {
                self.overmem.get(&index).unwrap_or(&0)
            }
        }
    }

    impl std::ops::IndexMut<usize> for Memory {
        fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
            if index < self.input.len() {
                &mut self.input[index]
            } else {
                self.overmem.entry(index).or_insert(0)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum IntProgramResult {
        Value(i64),
        Stalled,
    }

    #[derive(Clone)]
    pub struct IntProgram {
        pub memory: Memory,
        pc: usize,
        relative_position: isize,
        pub input_stack: VecDeque<i64>,
        default_input: Option<i64>,
    }

    impl IntProgram {
        pub fn parse(opcodes: &str) -> IntProgram {
            let memory = Memory {
                overmem: hashbrown::HashMap::new(),
                input: opcodes
                    .split(',')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            };

            IntProgram {
                memory,
                pc: 0,
                relative_position: 0,
                input_stack: VecDeque::new(),
                default_input: None,
            }
        }

        pub fn input(&mut self, input: i64) {
            self.input_stack.push_back(input);
        }

        pub fn has_input(&self) -> bool {
            self.input_stack.len() > 0
        }

        pub fn remove_default_input(&mut self) {
            self.default_input = None;
        }
        pub fn default_input(&mut self, input: i64) {
            self.default_input = Some(input);
        }

        fn get_index(&self, mode: ParamMode, i: usize) -> usize {
            match mode {
                ParamMode::Position => self.memory[i] as usize,
                ParamMode::Immediate => self.memory[i] as usize,
                ParamMode::Relative => (self.memory[i] as isize + self.relative_position) as usize,
            }
        }

        fn get_value(&self, mode: ParamMode, i: usize) -> i64 {
            match mode {
                ParamMode::Immediate => self.memory[i],
                ParamMode::Position => self.memory[self.memory[i] as usize],
                ParamMode::Relative => {
                    self.memory[(self.memory[i] as isize + self.relative_position) as usize]
                }
            }
        }
    }

    impl Iterator for IntProgram {
        type Item = IntProgramResult;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let instruction = self.memory[self.pc];
                let opcode = instruction % 100;
                let mode_1 = ParamMode::from((instruction / 100) % 10);
                let mode_2 = ParamMode::from((instruction / 1_000) % 10);
                let mode_3 = ParamMode::from((instruction / 10_000) % 10);
                //println!("{} - {:?}", opcode, self.memory);
                match opcode {
                    1 => {
                        // add
                        let a = self.get_value(mode_1, self.pc + 1);
                        let b = self.get_value(mode_2, self.pc + 2);
                        let index = self.get_index(mode_3, self.pc + 3);
                        self.memory[index as usize] = a + b;
                        self.pc += 4;
                    }
                    2 => {
                        // mul
                        let a = self.get_value(mode_1, self.pc + 1);
                        let b = self.get_value(mode_2, self.pc + 2);
                        let index = self.get_index(mode_3, self.pc + 3);
                        self.memory[index as usize] = a * b;
                        self.pc += 4;
                    }
                    3 => {
                        // input
                        let index = self.get_index(mode_1, self.pc + 1);
                        self.memory[index as usize] =
                            if let Some(input) = self.input_stack.pop_front() {
                                input
                            } else {
                                return Some(IntProgramResult::Stalled);
                            };

                        self.pc += 2;
                    }
                    4 => {
                        // output
                        let value = self.get_value(mode_1, self.pc + 1);
                        self.pc += 2;
                        return Some(IntProgramResult::Value(value));
                    }
                    5 => {
                        // jump not 0
                        let tester = self.get_value(mode_1, self.pc + 1);
                        let jumper = self.get_value(mode_2, self.pc + 2);
                        if tester != 0 {
                            self.pc = jumper as usize;
                        } else {
                            self.pc += 3;
                        }
                    }
                    6 => {
                        // jump if 0
                        let tester = self.get_value(mode_1, self.pc + 1);
                        let jumper = self.get_value(mode_2, self.pc + 2);
                        if tester == 0 {
                            self.pc = jumper as usize;
                        } else {
                            self.pc += 3;
                        }
                    }
                    7 => {
                        // a < b
                        let a = self.get_value(mode_1, self.pc + 1);
                        let b = self.get_value(mode_2, self.pc + 2);
                        let index = self.get_index(mode_3, self.pc + 3);
                        if a < b {
                            self.memory[index as usize] = 1;
                        } else {
                            self.memory[index as usize] = 0;
                        }
                        self.pc += 4;
                    }
                    8 => {
                        // a == b
                        let a = self.get_value(mode_1, self.pc + 1);
                        let b = self.get_value(mode_2, self.pc + 2);
                        let index = self.get_index(mode_3, self.pc + 3);
                        if a == b {
                            self.memory[index as usize] = 1;
                        } else {
                            self.memory[index as usize] = 0;
                        }
                        self.pc += 4;
                    }
                    9 => {
                        let a = self.get_value(mode_1, self.pc + 1);
                        self.relative_position += a as isize;
                        self.pc += 2;
                    }
                    99 => return None,
                    _ => panic!("Unexpected opcode: {}", opcode),
                }
            }
        }
    }
    /*
    fn self.get_value(mode: ParamMode, mem: &Memory, relative_position: i64, i: usize) -> i64 {
        match mode {
            ParamMode::Immediate => mem[i],
            ParamMode::Position => mem[mem[i] as usize],
            ParamMode::Relative => mem[(mem[i] + relative_position) as usize],
        }
    }*/
}
