use hashbrown::HashMap;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Ord, PartialOrd)]
pub struct Vector2(pub isize, pub isize);

impl Vector2 {
    pub fn length_sq(&self) -> isize {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn manhattan(lhs: &Self, other: &Self) -> usize {
        ((lhs.0 - other.0).abs() + (lhs.1 - other.1).abs()) as usize
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2(self.0 - other.0, self.1 - other.1)
    }
}

impl Add<Direction> for Vector2 {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        self + match other {
            Direction::North => Vector2(0, -1),
            Direction::East => Vector2(1, 0),
            Direction::South => Vector2(0, 1),
            Direction::West => Vector2(-1, 0),
        }
    }
}

impl AddAssign<Direction> for Vector2 {
    fn add_assign(&mut self, other: Direction) {
        *self = *self
            + match other {
                Direction::North => Vector2(0, -1),
                Direction::East => Vector2(1, 0),
                Direction::South => Vector2(0, 1),
                Direction::West => Vector2(-1, 0),
            }
    }
}

impl Sub<Direction> for Vector2 {
    type Output = Self;

    fn sub(self, other: Direction) -> Self {
        self - match other {
            Direction::North => Vector2(0, -1),
            Direction::East => Vector2(1, 0),
            Direction::South => Vector2(0, 1),
            Direction::West => Vector2(-1, 0),
        }
    }
}

impl SubAssign<Direction> for Vector2 {
    fn sub_assign(&mut self, other: Direction) {
        *self = *self
            - match other {
                Direction::North => Vector2(0, -1),
                Direction::East => Vector2(1, 0),
                Direction::South => Vector2(0, 1),
                Direction::West => Vector2(-1, 0),
            }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vector3(pub isize, pub isize, pub isize);

impl Vector3 {
    pub fn sign(self) -> Vector3 {
        Vector3(self.0.signum(), self.1.signum(), self.2.signum())
    }

    pub fn manhattan(lhs: &Self, other: &Self) -> usize {
        ((lhs.0 - other.0).abs() + (lhs.1 - other.1).abs() + (lhs.2 - other.2).abs()) as usize
    }

    pub fn length_sq(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
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

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.grid[x + y * self.width] = value;
    }

    pub fn set_vec(&mut self, pos: &Vector2, value: T) {
        self.set(pos.0 as usize, pos.1 as usize, value);
    }

    pub fn get_vec(&self, pos: &Vector2) -> Option<&T> {
        self.get(pos.0 as usize, pos.1 as usize)
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
        Value(i64), Stalled
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
                        self.memory[index as usize] = if let Some(input) = self.input_stack.pop_front() {
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
