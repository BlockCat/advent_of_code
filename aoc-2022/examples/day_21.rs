use std::collections::HashMap;

type InputType = HashMap<String, MonkeyJob>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_21.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (String, MonkeyJob) {
    let mon = line[0..4].to_string();

    let tokens = line.split(" ").skip(1).collect::<Vec<_>>();

    if tokens.len() == 1 {
        let val = tokens[0].parse().unwrap();
        (mon, MonkeyJob::Value(val))
    } else {
        let l = tokens[0].to_string();
        let r = tokens[2].to_string();

        let job = match tokens[1] {
            "*" => MonkeyJob::Mul(l, r),
            "+" => MonkeyJob::Add(l, r),
            "-" => MonkeyJob::Sub(l, r),
            "/" => MonkeyJob::Div(l, r),
            c => unreachable!("un: {}", c),
        };
        (mon, job)
    }
}

fn exercise_1(input: InputType) -> isize {
    test(&input, &"root".to_string())
}

fn rekt(input: &InputType, monkey: &String) -> String {
    let job = &input[monkey];
    if monkey == "humn" {
        return "x".to_string();
    }
    match job {
        MonkeyJob::Value(a) => format!("{}", a),
        MonkeyJob::Add(a, b) => {
            format!("({}+{})", rekt(input, a), rekt(input, b))
        }
        MonkeyJob::Mul(a, b) => {
            format!("({}*{})", rekt(input, a), rekt(input, b))
        }
        MonkeyJob::Sub(a, b) => {
            format!("({}-{})", rekt(input, a), rekt(input, b))
        }
        MonkeyJob::Div(a, b) => {
            format!("({}/{})", rekt(input, a), rekt(input, b))
        }
    }
}

fn test(input: &InputType, monkey: &String) -> isize {
    let job = &input[monkey];
    match job {
        MonkeyJob::Value(a) => *a,
        MonkeyJob::Add(a, b) => {
            let (a, b) = rayon::join(|| test(input, a), || test(input, b));
            a + b
        }
        MonkeyJob::Mul(a, b) => {
            let (a, b) = rayon::join(|| test(input, a), || test(input, b));
            a * b
        }
        MonkeyJob::Sub(a, b) => {
            let (a, b) = rayon::join(|| test(input, a), || test(input, b));
            a - b
        }
        MonkeyJob::Div(a, b) => {
            let (a, b) = rayon::join(|| test(input, a), || test(input, b));
            a / b
        }
    }
}

fn exercise_2(mut input: InputType) -> isize {
    let root = &input["root"];
    let (l, r) = match root {
        MonkeyJob::Value(_) => unreachable!(),
        MonkeyJob::Add(l, r) => (l.to_string(), r.to_string()),
        MonkeyJob::Mul(l, r) => (l.to_string(), r.to_string()),
        MonkeyJob::Sub(l, r) => (l.to_string(), r.to_string()),
        MonkeyJob::Div(l, r) => (l.to_string(), r.to_string()),
    };

    let mut min = 0;
    let mut max = 10_000_000_000_000;

    while min + 1 < max {
        let pos = (max + min) / 2;
        input.insert("humn".to_string(), MonkeyJob::Value(pos));

        let (l, r) = rayon::join(|| test(&input, &l), || test(&input, &r));

        if l == r {
            return pos;
        } else if l > r {
            min = pos;
        } else {
            max = pos;
        }
    }
    unreachable!()
}

#[derive(Debug, Clone)]

enum MonkeyJob {
    Value(isize),
    Add(String, String),
    Mul(String, String),
    Sub(String, String),
    Div(String, String),
}
