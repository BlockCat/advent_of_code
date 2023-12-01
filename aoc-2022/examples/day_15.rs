use aoc_2022::vector::Vector2;
use rayon::prelude::*;

type InputType = Vec<Sensor>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise2(numbers));
}

const MAX_BOUND: isize = 4_000_000;

fn input() -> InputType {
    include_str!("../input/day_15.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Sensor {
    let mut parts = line.split(' ');

    let x_pos = parts.nth(2).unwrap();
    let y_pos = parts.next().unwrap();

    let x_pos = x_pos[2..x_pos.len() - 1].parse().unwrap();
    let y_pos = y_pos[2..y_pos.len() - 1].parse().unwrap();

    let x_beacon = parts.nth(4).unwrap();
    let y_beacon = parts.next().unwrap();

    let x_beacon = x_beacon[2..x_beacon.len() - 1].parse().unwrap();
    let y_beacon = y_beacon[2..].parse().unwrap();

    Sensor {
        position: Vector2::new([x_pos, y_pos]),
        closest_beacon: Vector2::new([x_beacon, y_beacon]),
    }
}

fn exercise_1(input: InputType) -> isize {
    let mut ranges = input
        .par_iter()
        .filter_map(|x| {
            let pos = x.position[0];
            let diff = Vector2::manhattan(&x.position, &x.closest_beacon) as isize;

            let dy = (2_000_000 - x.position[1]).abs();

            if dy <= diff {
                let dx = (diff - dy).abs();
                Some((pos - dx, pos + dx))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    ranges.sort();

    let mut count = 0;
    let mut left = isize::MIN;

    for (l, r) in ranges {
        assert!(l <= r);
        if l > left {
            left = l;
        }

        if left >= r {
            continue;
        }

        count += r - left;
        left = r;
    }

    count
}

fn exercise2(input: InputType) -> isize {
    return (0..=MAX_BOUND)
        .par_bridge()
        .find_map_first(|y| exercise_2_helper(&input, y).map(|x| (x, y)))
        .map(|(x, y)| x * 4_000_000 + y)
        .unwrap();
}

fn exercise_2_helper(input: &InputType, y: isize) -> Option<isize> {
    let mut ranges = input
        .iter()
        .filter_map(|x| {
            let pos = x.position[0];
            let diff = Vector2::manhattan(&x.position, &x.closest_beacon) as isize;

            let dy = (y - x.position[1]).abs();

            if dy <= diff {
                let dx = (diff - dy).abs();
                if pos + dx >= 0 {
                    return Some((pos - dx, pos + dx));
                }
            }
            None
        })
        .collect::<Vec<_>>();
    ranges.sort();

    let mut left = 0;

    for (l, r) in ranges {
        assert!(l <= r);
        if l > left {
            if l - left == 2 {
                return Some(l + 1);
            }
            left = l;
        }

        if left >= r {
            continue;
        }

        left = r;
    }

    None
}

#[derive(Debug, Clone)]
struct Sensor {
    position: Vector2,
    closest_beacon: Vector2,
}
