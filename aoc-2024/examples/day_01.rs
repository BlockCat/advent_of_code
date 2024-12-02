use aoc_2024::stopwatch;

type Input = (Vec<u64>, Vec<u64>);

pub fn main() {
    let input = include_str!("../input/day_01.txt");
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
    let (mut a, mut b): (Vec<_>, Vec<_>) = input.lines().map(parse_line).unzip();
    a.sort_unstable();
    b.sort_unstable();

    (a, b)
}

fn parse_line(input: &str) -> (u64, u64) {
    let mut i = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());

    (i.next().unwrap(), i.next().unwrap())
}

fn exercise_1((left, right): &Input) -> u64 {
    left.iter()
        .zip(right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum()
}

fn exercise_2((left, right): &Input) -> u64 {
    let mut sum = 0;

    let mut l_index = 0;
    let mut r_index = 0;

    while l_index < left.len() {
        let l = left[l_index];
        let left_count = left[l_index..].iter().take_while(|&&f| f == l).count() as u64;

        r_index += right[r_index..].iter().take_while(|&&f| f < l).count();

        let right_count = right[r_index..].iter().take_while(|&&f| f == l).count() as u64;

        l_index += left_count as usize;
        r_index += right_count as usize;

        sum += l * right_count * left_count;
    }

    sum
}
