use std::collections::HashMap;
use std::str::FromStr;

enum Comparison {
    Equal(String, i32), Less(String, i32), Greater(String, i32), LessEqual(String, i32), GreaterEqual(String, i32), Not(String, i32)
}

impl Comparison {
    fn evaluate(&self, input: &HashMap<String, i32>) -> bool {
        match self {
            Comparison::Equal(reg, val)         => val == input.get(reg).unwrap_or(&0),
            Comparison::Less(reg, val)          => val > input.get(reg).unwrap_or(&0),
            Comparison::Greater(reg, val)       => val < input.get(reg).unwrap_or(&0),
            Comparison::LessEqual(reg, val)     => val >= input.get(reg).unwrap_or(&0),
            Comparison::GreaterEqual(reg, val)  => val <= input.get(reg).unwrap_or(&0),
            Comparison::Not(reg, val)           => val != input.get(reg).unwrap_or(&0),
        }
    }
}

enum Action {
    Increment(i32), Decrement(i32)
}

struct Instruction {
    register: String,
    action: Action,
    compare: Comparison
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let register = parts.next().unwrap().to_string();
        let action = match parts.next().unwrap() {
            "inc" => Action::Increment(parts.next().unwrap().parse::<i32>().unwrap()),
            "dec" => Action::Decrement(parts.next().unwrap().parse::<i32>().unwrap()),
            c => return Err(format!("Invalid action: {}", c))
        };

        // skip the if
        parts.next();

        let compare_register = parts.next().unwrap().to_string();
        let compare = match parts.next().unwrap() {
            "==" => Comparison::Equal(compare_register, parts.next().unwrap().parse::<i32>().unwrap()),
            "<" => Comparison::Less(compare_register, parts.next().unwrap().parse::<i32>().unwrap()),
            ">" => Comparison::Greater(compare_register, parts.next().unwrap().parse::<i32>().unwrap()),
            "<=" => Comparison::LessEqual(compare_register, parts.next().unwrap().parse::<i32>().unwrap()),
            ">=" => Comparison::GreaterEqual(compare_register, parts.next().unwrap().parse::<i32>().unwrap()),
            "!=" => Comparison::Not(compare_register, parts.next().unwrap().parse::<i32>().unwrap()),
            c => return Err(format!("Invalid comparison: {}", c))
        };

        Ok(Instruction {
            register,
            action,
            compare
        })
    }
}

fn read_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| Instruction::from_str(l).unwrap()).collect::<Vec<_>>()
}

fn algorithm1(input: Vec<Instruction>) -> (i32, i32) {
    let mut registers = HashMap::<String, i32>::new();
    let mut highest = std::i32::MIN;
    for instruction in input.into_iter() {
        if instruction.compare.evaluate(&registers) {
            let increment = match instruction.action {
                Action::Increment(amount) => amount,
                Action::Decrement(amount) => -amount
            };

            let v = registers
                .entry(instruction.register)
                .and_modify(|v| *v += increment)
                .or_insert(increment);

            highest = std::cmp::max(highest, *v);
        }
    }
    (*registers.values().max().unwrap(), highest)
}


#[test]
fn test_examples() {
    let input = read_input(r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10");
    assert_eq!(algorithm1(input), (1, 10));
}

#[test]
fn run8() {
    let input = read_input(include_str!("input/day8.txt"));
    let (current, ever) = algorithm1(input);
    println!("Current: {}, Highest: {}", current, ever);
}
