use std::collections::HashSet;
use std::iter::FromIterator;

fn algorithm1(input: &str) -> usize {
    input.lines()
        .filter(evaluate)
        .count()
}

fn algorithm2(input: &str) -> usize {
    input.lines()
        .filter(evaluate2)
        .count()
}

fn evaluate(input: &&str) -> bool {
    let slice: Vec<&str> = input.split_whitespace().collect();
    let len = slice.len();

    let set: HashSet<_> = HashSet::from_iter(slice.into_iter());

    set.len() == len
}

fn evaluate2(input: &&str) -> bool {
    let slice: Vec<String> = input
    .split_whitespace()
    .map(|x| {
        let mut m = x.chars().collect::<Vec<_>>();
        m.sort();
        m.into_iter().collect::<String>()
    })
    .collect();
    let len = slice.len();

    let set: HashSet<_> = HashSet::from_iter(slice.into_iter());

    set.len() == len
}


#[test]
fn test_example() {
    assert_eq!(evaluate(&"aa bb cc dd ee"), true);
    assert_eq!(evaluate(&"aa bb cc dd aa"), false);
    assert_eq!(evaluate(&"aa bb cc dd aaa"), true);
}


#[test]
fn test_example2() {
    assert_eq!(evaluate2(&"abcde fghij"), true);
    assert_eq!(evaluate2(&"abcde xyz ecdab"), false);
    assert_eq!(evaluate2(&"a ab abc abd abf abj"), true);
}

#[test]
fn run_first() {
    let input = include_str!("input/day4.txt");

    println!("{}", algorithm1(input));
    println!("{}", algorithm2(input));
}