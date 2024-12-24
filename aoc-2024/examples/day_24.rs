use aoc_2024::stopwatch;
use std::collections::{HashMap, VecDeque};

type Input = (
    HashMap<String, bool>,
    HashMap<(String, String, Operation), String>,
);
// gbs,hwq,thm,wrm,wss,z08,z22,z22
// gbs,hwq,thm,wrm,wss,z08,z22,z29

pub fn main() {
    let numbers = input(include_str!("../input/day_24.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!(
            "Exercise 1: {}",
            exercise_1(numbers.0.clone(), &numbers.1, HashMap::new())
        );
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    let (a, b) = input.split_once("\n\n").unwrap();

    (parse_wires(a), parse_connections(b))
}

fn parse_wires(line: &str) -> HashMap<String, bool> {
    line.lines()
        .map(|x| {
            let (name, val) = x.split_once(": ").unwrap();

            (name.to_string(), val == "1")
        })
        .collect()
}

fn parse_connections(line: &str) -> HashMap<(String, String, Operation), String> {
    line.lines()
        .map(|x| {
            let mut x = x.split_whitespace();
            let a = x.next().unwrap().to_string();
            let op = match x.next().unwrap() {
                "AND" => Operation::AND,
                "XOR" => Operation::XOR,
                "OR" => Operation::OR,
                _ => unreachable!(),
            };
            let b = x.next().unwrap().to_string();
            let c = x.skip(1).next().unwrap().to_string();

            ((a, b, op), c)
        })
        .collect()
}

fn exercise_1(
    mut wires: HashMap<String, bool>,
    gates: &HashMap<(String, String, Operation), String>,
    replacement: HashMap<String, String>,
) -> u64 {
    let mut event_queue = wires.keys().cloned().collect::<VecDeque<_>>();

    while let Some(event) = event_queue.pop_front() {
        for ((a, b, op), c) in gates {
            if a == &event || b == &event {
                let (value_a, value_b) = match (wires.get(a), wires.get(b)) {
                    (Some(va), Some(vb)) => (*va, *vb),
                    _ => continue,
                };

                let result = match op {
                    Operation::AND => value_a & value_b,
                    Operation::XOR => value_a ^ value_b,
                    Operation::OR => value_a | value_b,
                };

                let c = if let Some(replace) = replacement.get(c) {
                    replace
                } else {
                    c
                };

                event_queue.push_back(c.clone());
                wires.insert(c.clone(), result);
            }
        }
    }

    wire_to_number(&wires, 'z')
}

fn exercise_2(input: &Input) -> String {
    let mut replacement = HashMap::new();

    fn add_replacement(a: &str, b: &str, replacement: &mut HashMap<String, String>) {
        replacement.insert(a.to_string(), b.to_string());
        replacement.insert(b.to_string(), a.to_string());
    }

    add_replacement("thm", "z08", &mut replacement);
    add_replacement("wss", "wrm", &mut replacement);
    add_replacement("hwq", "z22", &mut replacement);
    add_replacement("gbs", "z29", &mut replacement);

    // print graphviz:

    println!("digraph G {{");

    for ((a, b, op), t) in &input.1 {
        let op = match op {
            Operation::AND => "and",
            Operation::XOR => "xor",
            Operation::OR => "or",
        };
        let t = replacement.get(t).unwrap_or(t);
        println!("\t{} [label=\"{}, {}\"];", t, t, op);
        println!("\t{} -> {};", a, t);
        println!("\t{} -> {};", b, t);
    }
    
    println!("}}");

    let a = 0b111111111111111111111111101111111111111111111;
    let b = 0b000000000000000000000000010000000000000000001;

    let mut stuff = replacement.keys().cloned().collect::<Vec<_>>();
    stuff.sort();
    println!("{}", stuff.join(","));

    tester(a, b, &input.1, replacement, true);

    stuff.join(",")
}

fn tester(
    a: u64,
    b: u64,
    gates: &HashMap<(String, String, Operation), String>,
    replacement: HashMap<String, String>,
    print: bool,
) -> u64 {
    let wires = number_to_wire(45, a, 'x')
        .into_iter()
        .chain(number_to_wire(45, b, 'y'))
        .collect::<HashMap<String, bool>>();

    let x = wire_to_number(&wires, 'x');
    let y = wire_to_number(&wires, 'y');

    assert_eq!(x, a);
    assert_eq!(y, b);

    let r = exercise_1(wires, gates, replacement);

    let wrong = r ^ (a + b);

    if print {
        println!("x: {:050b}", x);
        println!("y: {:050b}", y);
        println!("a: {:050b}", r);
        println!("e: {:050b}", a + b);
        println!("d: {:050b}\n", wrong);
    }
    wrong
}

fn number_to_wire(size: usize, mut number: u64, prefix: char) -> Vec<(String, bool)> {
    let mut counter = 0;
    let mut v = Vec::new();
    for _ in 0..size {
        let name = format!("{}{:02}", prefix, counter);

        v.push((name, number & 1 == 1));

        counter += 1;
        number >>= 1;
    }

    v
}

fn wire_to_number(wires: &HashMap<String, bool>, prefix: char) -> u64 {
    let mut z = wires
        .iter()
        .filter(|(a, _)| a.starts_with(prefix))
        .map(|x| (x.0.as_str(), *x.1))
        .collect::<Vec<_>>();
    z.sort();

    bits_to_number(z)
}
fn bits_to_number(mut bits: Vec<(&str, bool)>) -> u64 {
    let mut num = 0;
    bits.sort();

    for (name, val) in bits {
        let bit: usize = name[1..].parse().unwrap();
        if val {
            num |= 1 << bit
        }
    }
    num
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    AND,
    XOR,
    OR,
}
