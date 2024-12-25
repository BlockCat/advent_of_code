use aoc_2024::stopwatch;

type Input = Vec<(bool, [u8; 5])>;

pub fn main() {
    let numbers = input(include_str!("../input/day_25.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        // println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.split("\n\n").map(parse_part).collect()
}

fn parse_part(line: &str) -> (bool, [u8; 5]) {
    let lines = line
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let is_key = lines[0][0] == '#';

    let mut fit = [0u8; 5];

    for i in 0..5 {
        let mut h = 0u8;
        for j in 0..7 {
            if is_key {
                if lines[j as usize][i] == '#' {
                    h = j;
                } else {
                    break;
                }
            } else {
                if lines[6 - j as usize][i] == '#' {
                    h = j;
                } else {
                    break;
                }
            }
        }
        fit[i] = h;
    }

    (is_key, fit)
}

fn exercise_1(input: &Input) -> usize {
    let keys = input.iter().filter(|x| x.0).cloned().collect::<Vec<_>>();
    let locks = input.iter().filter(|x| !x.0).cloned().collect::<Vec<_>>();

    let mut counter = 0;

    for (_, key) in &keys {
        for (_, lock) in &locks {
            // println!("{:?}, {:?} = {}", key, lock, fits(key,lock));
            if fits(key, lock) {
                counter += 1;
            }
        }
    }

    counter
}

fn fits(key: &[u8; 5], lock: &[u8; 5]) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

fn exercise_2(input: &Input) -> usize {
    unimplemented!()
}
