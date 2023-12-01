use std::collections::{HashMap, HashSet};

type Input<'a> = Vec<Line<'a>>;

pub fn main() {
    let input = parse_input(include_str!("../input/day08.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line<'a>(line: &'a str) -> Line<'a> {
    let mut split1 = line.split('|');

    let numbers = split1.next().unwrap();
    let output = split1.next().unwrap();

    let numbers = numbers.trim().split(' ').collect();
    let output = output.trim().split(' ').collect();

    Line { numbers, output }
}

fn exercise_1(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|x| {
            x.output.iter().filter(|x| match x.len() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
        })
        .count()
}

fn exercise_2(input: &Input) -> usize {
    input.iter().map(deduce_line).sum()
}

fn deduce_line(line: &Line) -> usize {
    let mut sieve = create_mapping();

    for number_text in &line.numbers {
        handle_unique_str(number_text, &mut sieve)
    }

    let mut known = update_known(&sieve);

    let mut changed = true;
    while changed {
        changed = false;
        for (l, s) in sieve.iter_mut() {
            for (lane, cha) in &known {
                if lane != l {
                    if s.remove(cha) {
                        changed = true;
                    }
                }
            }
        }

        known = update_known(&sieve);
    }

    let mut sum = str_to_number(&line.output[0], &known) * 1000;
    sum += str_to_number(&line.output[1], &known) * 100;
    sum += str_to_number(&line.output[2], &known) * 10;
    sum += str_to_number(&line.output[3], &known);

    sum
}

fn update_known(sieve: &HashMap<char, HashSet<char>>) -> HashMap<char, char> {
    sieve
        .iter()
        .filter(|x| x.1.len() == 1)
        .map(|x| (*x.0, *x.1.iter().next().unwrap()))
        .collect::<HashMap<char, char>>()
}

fn handle_unique_str(number_text: &str, sieve: &mut HashMap<char, HashSet<char>>) {
    let set = number_text.chars().collect::<HashSet<_>>();
    match number_text.len() {
        2 => clean_char(&['F', 'C'], &set, sieve),
        3 => clean_char(&['F', 'C', 'A'], &set, sieve),
        4 => clean_char(&['F', 'C', 'B', 'D'], &set, sieve),
        5 => clean_char(&['A', 'D', 'G'], &set, sieve),
        6 => clean_char(&['A', 'B', 'F', 'G'], &set, sieve),
        _ => {}
    }
}

fn clean_char(char: &[char], set: &HashSet<char>, sieve: &mut HashMap<char, HashSet<char>>) {
    for lane in char {
        sieve
            .entry(*lane)
            .and_modify(|a| *a = a.intersection(set).cloned().collect());
    }
}

fn str_to_number(l: &str, known: &HashMap<char, char>) -> usize {
    match l.len() {
        2 => return 1,
        3 => return 7,
        4 => return 4,
        7 => return 8,
        _ => {}
    }

    let c = known.get(&'C').map(|d| l.contains(*d)).unwrap_or(false);
    let d = known.get(&'D').map(|d| l.contains(*d)).unwrap_or(false);
    let e = known.get(&'E').map(|d| l.contains(*d)).unwrap_or(false);
    let f = known.get(&'F').map(|d| l.contains(*d)).unwrap_or(false);

    match l.len() {
        5 => match (c, f) {
            // 2, 3, 5
            (true, true) => return 3,
            (true, false) => return 2,
            (false, true) => return 5,
            (false, false) => unreachable!(),
        },
        6 => {
            // 0 , 6, 9
            match (e, d) {
                (true, true) => return 6,
                (true, false) => return 0,
                (false, true) => return 9,
                (false, false) => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn create_mapping() -> HashMap<char, HashSet<char>> {
    let all_set = HashSet::from_iter('a'..='g');
    ('A'..='G').map(|x| (x, all_set.clone())).collect()
}

struct Line<'a> {
    numbers: Vec<&'a str>,
    output: Vec<&'a str>,
}
