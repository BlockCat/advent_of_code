use aoc_2024::stopwatch;

type Input = Vec<Instruction>;

pub fn main() {
    let input = include_str!("../input/day_03.txt");

    let l = stopwatch(|| {
        let input = parse(input);

        let a1 = exercise_1(&input);
        println!("Ex1: {}", a1);
        let a2 = exercise_2(&input);
        println!("Ex2: {}", a2);
    });

    println!("Time: {:?}", l);
}

fn parse(input: &str) -> Input {
    input.lines().map(parse_line).flatten().collect()
}

fn parse_line(input: &str) -> Vec<Instruction> {
    let re = regex::Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|x| {
            if &x[0] == "do()" {
                Instruction::Enable
            } else if &x[0] == "don't()" {
                Instruction::Disable
            } else {
                Instruction::Mul(x[3].parse().unwrap(), x[4].parse().unwrap())
            }
        })
        .collect()
}

fn exercise_1(input: &Input) -> i64 {
    input
        .iter()
        .map(|x| match x {
            Instruction::Mul(a, b) => *a * *b,
            _ => 0,
        })
        .sum()
}

fn exercise_2(input: &Input) -> i64 {
    input
        .iter()
        .fold((0, true), |(sum, enable_mul), x| match x {
            Instruction::Mul(a, b) => {
                if enable_mul {
                    (*a * *b + sum, enable_mul)
                } else {
                    (sum, enable_mul)
                }
            }
            Instruction::Enable => (sum, true),
            Instruction::Disable => (sum, false),
        })
        .0
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(i64, i64),
    Enable,
    Disable,
}
