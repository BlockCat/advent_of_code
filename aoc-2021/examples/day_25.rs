use std::str::FromStr;

type Input = Vec<usize>;

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> usize {
    
}

fn exercise_1(input: &Input) -> usize {
    0
}

fn exercise_2(input: &Input) -> usize {
    0
}

enum Instruction {
    Input(u8),
    Add(u8, Val),
    Mul(u8, Val),
    Div(u8, Val),
    Mod(u8, Val),
    Eql(u8, Val),
}

impl Instruction {
    fn execute(&self, registry: &mut [isize], number: &[u8; 14], index: &mut usize) {
        match self {
            Instruction::Input(a) => {
                registry[*a as usize] = number[*index] as isize;
                *index += 1;
            }
            Instruction::Add(a, b) => registry[*a as usize] += b.value(registry),
            Instruction::Mul(a, b) => registry[*a as usize] *= b.value(registry),
            Instruction::Div(a, b) => registry[*a as usize] /= b.value(registry),
            Instruction::Mod(a, b) => registry[*a as usize] %= b.value(registry),
            Instruction::Eql(a, b) => {
                registry[*a as usize] = isize::from(registry[*a as usize] == b.value(&registry))
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        match s.next().unwrap() {
            "inp" => Ok(Instruction::Input(
                s.next().unwrap().chars().next().unwrap() as u8 - 'w' as u8,
            )),
            "add" => Ok(Instruction::Add(
                s.next().unwrap().chars().next().unwrap() as u8 - 'w' as u8,
                s.next().unwrap().parse().unwrap(),
            )),
            "mul" => Ok(Instruction::Mul(
                s.next().unwrap().chars().next().unwrap() as u8 - 'w' as u8,
                s.next().unwrap().parse().unwrap(),
            )),
            "div" => Ok(Instruction::Div(
                s.next().unwrap().chars().next().unwrap() as u8 - 'w' as u8,
                s.next().unwrap().parse().unwrap(),
            )),
            "mod" => Ok(Instruction::Mod(
                s.next().unwrap().chars().next().unwrap() as u8 - 'w' as u8,
                s.next().unwrap().parse().unwrap(),
            )),
            "eql" => Ok(Instruction::Eql(
                s.next().unwrap().chars().next().unwrap() as u8 - 'w' as u8,
                s.next().unwrap().parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Val {
    Registry(u8),
    Value(i16),
}

impl Val {
    fn value(self, registry: &[isize]) -> isize {
        match self {
            Val::Registry(a) => registry[a as usize],
            Val::Value(v) => v as isize,
        }
    }
}

impl FromStr for Val {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(d) = s.parse() {
            Ok(Val::Value(d))
        } else {
            Ok(Val::Registry(s.chars().next().unwrap() as u8 - 'w' as u8))
        }
    }
}
