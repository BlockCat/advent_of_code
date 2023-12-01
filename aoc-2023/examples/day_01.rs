use std::collections::HashMap;

pub fn main() {
    let input = include_str!("../input/day_01_big.txt");
    let a1 = exercise_1(input);
    println!("Ex1: {}", a1);
    let a2 = exercise_2(input);
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
    let mapper: HashMap<&str, u32> = HashMap::from_iter([
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let min = line
        .match_indices("one")
        .take(1)
        .chain(line.match_indices("two").take(1))
        .chain(line.match_indices("three").take(1))
        .chain(line.match_indices("four").take(1))
        .chain(line.match_indices("five").take(1))
        .chain(line.match_indices("six").take(1))
        .chain(line.match_indices("seven").take(1))
        .chain(line.match_indices("eight").take(1))
        .chain(line.match_indices("nine").take(1))
        .chain(line.match_indices(|c: char| c.is_ascii_digit()).take(1))
        .min_by_key(|x| x.0)
        .unwrap()
        .1;

    let max = line
        .rmatch_indices("one")
        .take(1)
        .chain(line.rmatch_indices("two").take(1))
        .chain(line.rmatch_indices("three").take(1))
        .chain(line.rmatch_indices("four").take(1))
        .chain(line.rmatch_indices("five").take(1))
        .chain(line.rmatch_indices("six").take(1))
        .chain(line.rmatch_indices("seven").take(1))
        .chain(line.rmatch_indices("eight").take(1))
        .chain(line.rmatch_indices("nine").take(1))
        .chain(line.rmatch_indices(|c: char| c.is_ascii_digit()).take(1))
        .max_by_key(|x| x.0)
        .unwrap()
        .1;

    // let (a, b) = line
    //     .match_indices("one")
    //     .chain(line.match_indices("two"))
    //     .chain(line.match_indices("three"))
    //     .chain(line.match_indices("four"))
    //     .chain(line.match_indices("five"))
    //     .chain(line.match_indices("six"))
    //     .chain(line.match_indices("seven"))
    //     .chain(line.match_indices("eight"))
    //     .chain(line.match_indices("nine"))
    //     .chain(line.match_indices(|c: char| c.is_ascii_digit()))
    //     .fold((None, None), |(a, b), x| {
    //         (
    //             if a.is_some() && a < Some(x) {
    //                 a
    //             } else {
    //                 Some(x)
    //             },
    //             if b.is_some() && b > Some(x) {
    //                 b
    //             } else {
    //                 Some(x)
    //             },
    //         )
    //     });

    // let a = mapper[a.unwrap().1];
    // let b = mapper[b.unwrap().1];
    let a = mapper[min];
    let b = mapper[max];

    a * 10 + b
}

fn exercise_1(input: &str) -> u32 {
    parse(input).iter().sum()
}

fn exercise_2(input: &str) -> u32 {
    parse2(input).iter().sum()
}
