type InputType = Vec<Instruction>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_10.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Instruction {
    let instruction = &line[0..4];

    match instruction {
        "noop" => Instruction::NOOP,
        "addx" => Instruction::ADD(line[5..].parse().unwrap()),
        _ => unimplemented!(),
    }
}

fn exercise_1(input: InputType) -> isize {
    let mut register_x = 1;

    let mut cycle = 0;

    let mut values = Vec::new();

    for entry in input {
        match entry {
            Instruction::NOOP => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    values.push(cycle as isize * register_x);
                    if cycle == 220 {
                        break;
                    }
                }
            }
            Instruction::ADD(s) => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    values.push(cycle as isize * register_x);
                    if cycle == 220 {
                        break;
                    }
                }
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    values.push(cycle as isize * register_x);
                    if cycle == 220 {
                        break;
                    }
                }
                register_x += s;
            }
        }
    }

    values.into_iter().sum()
}

fn exercise_2(input: InputType) -> usize {
    let mut register_x = 1;

    let mut cycle = 0;

    let mut values: Vec<char> = Vec::new();

    let check = move |a: isize, b: isize| {
        let contains = ((a % 40) - b).abs() <= 1;
        if contains {
            '#'
        } else {
            ' '
        }
    };

    for entry in input {
        match entry {
            Instruction::NOOP => {
                values.push(check(cycle, register_x));
                cycle += 1;
            }
            Instruction::ADD(s) => {
                values.push(check(cycle, register_x));
                cycle += 1;
                values.push(check(cycle, register_x));
                cycle += 1;
                register_x += s;
            }
        }
    }

    let mut counter = 0;
    for _ in 0..6 {
        for _ in 0..40 {
            print!("{}", &values[counter]);
            counter += 1;
        }
        println!()
    }

    0
}

#[derive(Debug, Clone)]
enum Instruction {
    NOOP,
    ADD(isize),
}
