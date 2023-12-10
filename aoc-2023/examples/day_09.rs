type InputType = Vec<Vec<isize>>;

pub fn main() {
    let input = parse(include_str!("../input/day_09.txt"));

    let (ex2, ex1) = exercise_1(input);

    println!("Exercise 1: {}", ex1);
    println!("Exercise 2: {}", ex2);
}

fn parse<'a>(input: &'a str) -> InputType {
    input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn exercise_1(input: InputType) -> (isize, isize) {
    input
        .iter()
        .map(find_prev_next_val)
        .fold((0isize, 0isize), |acc, (prev, next)| {
            (acc.0 + prev, acc.1 + next)
        })
}

fn find_prev_next_val(sequence: &Vec<isize>) -> (isize, isize) {
    let mut list = vec![sequence.clone()];

    loop {
        let next = difference_sequence(list.last().unwrap());

        // is all same
        let f = next[0];
        if next.iter().all(|&x| x == f) {
            return list
                .iter()
                .rev()
                .map(|s| (s.first().unwrap(), s.last().unwrap()))
                .fold((next[0], next[0]), |(prev, next), (first, last)| {
                    (first - prev, last + next)
                });
        } else {
            list.push(next);
        }
    }
}

fn difference_sequence(sequence: &[isize]) -> Vec<isize> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}
