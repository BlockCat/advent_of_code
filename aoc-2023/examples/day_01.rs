use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../input/day_01.txt");
    let a1 = exercise_1(input);
    let a2 = exercise_2(input);
    println!("Ex1: {}", a1);
    println!("Ex2: {}", a2);
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|x| parse_line(x)).collect()
}

fn parse2(input: &str) -> Vec<u32> {
    input.lines().map(|x| parse_line_2(x)).collect()
}

fn parse_line(line: &str) -> u32 {
    let (a, b) = line.chars().fold((None, None), |(a, b), c| {
        if ('0'..='9').contains(&c) {
            if a.is_none() {
                (Some(c), Some(c))
            } else {
                (a, Some(c))
            }
        } else {
            (a, b)
        }
    });

    let a = a.unwrap().to_digit(10).unwrap();
    let b = b.unwrap().to_digit(10).unwrap();
    a * 10 + b
}

fn parse_line_2(line: &str) -> u32 {
    let mapper = {
        let mut map = HashMap::with_capacity(20);
        map.insert("one", 1u32);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);
        map.insert("six", 6);
        map.insert("seven", 7);
        map.insert("eight", 8);
        map.insert("nine", 9);
        map.insert("zero", 0);
        map.insert("1", 1);
        map.insert("2", 2);
        map.insert("3", 3);
        map.insert("4", 4);
        map.insert("5", 5);
        map.insert("6", 6);
        map.insert("7", 7);
        map.insert("8", 8);
        map.insert("9", 9);
        map.insert("0", 0);
        map
    };

    let l = line
        .match_indices("one")
        .chain(line.match_indices("two"))
        .chain(line.match_indices("three"))
        .chain(line.match_indices("four"))
        .chain(line.match_indices("five"))
        .chain(line.match_indices("six"))
        .chain(line.match_indices("seven"))
        .chain(line.match_indices("eight"))
        .chain(line.match_indices("nine"))
        .chain(line.match_indices("zero"))
        .chain(line.match_indices("1"))
        .chain(line.match_indices("2"))
        .chain(line.match_indices("3"))
        .chain(line.match_indices("4"))
        .chain(line.match_indices("5"))
        .chain(line.match_indices("6"))
        .chain(line.match_indices("7"))
        .chain(line.match_indices("8"))
        .chain(line.match_indices("9"))
        .chain(line.match_indices("0"))
        .collect::<Vec<_>>();

    let a = mapper[l.iter().min_by_key(|x| x.0).unwrap().1];
    let b = mapper[l.iter().max_by_key(|x| x.0).unwrap().1];

    a * 10 + b
}

fn exercise_1(input: &str) -> u32 {
    parse(input).iter().sum()
}

fn exercise_2(input: &str) -> u32 {
    parse2(input).iter().sum()
}
