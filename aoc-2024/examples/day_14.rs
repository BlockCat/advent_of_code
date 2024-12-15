use std::collections::{HashMap, HashSet};

use aoc_2024::{stopwatch, vector::Vector2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Input = Vec<Bot>;

struct Bot {
    pos: Vector2,
    vel: Vector2,
}

pub fn main() {
    let numbers = input(include_str!("../input/day_14.txt"));
    let width = 101;
    let height = 103;

    // let numbers = input(include_str!("../input/test.txt"));
    // let width = 11;
    // let height = 7;

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers, width, height));
        println!("Exercise 2: {}", exercise_2(&numbers, width, height));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Bot {
    let (p, v) = line.split_once(" ").unwrap();

    let p = p.split_once(",").unwrap();
    let v = v.split_once(",").unwrap();

    Bot {
        pos: Vector2::new([p.0[2..].parse().unwrap(), p.1.parse().unwrap()]),
        vel: Vector2::new([v.0[2..].parse().unwrap(), v.1.parse().unwrap()]),
    }
}

fn exercise_1(input: &Input, width: isize, height: isize) -> usize {
    let mut quadrants = [[0; 2]; 2];

    input
        .iter()
        .map(|b| {
            let x = final_pos_after_seconds(b.pos[0], b.vel[0], width, 100);
            let y = final_pos_after_seconds(b.pos[1], b.vel[1], height, 100);

            (x, y)
        })
        .filter(|(x, y)| *x != width / 2 && *y != height / 2)
        .for_each(|(x, y)| {
            let x = x / ((width + 1) / 2);
            let y = y / ((height + 1) / 2);

            quadrants[x as usize][y as usize] += 1;
        });

    quadrants.iter().flatten().product()
}

fn final_pos_after_seconds(pos: isize, vel: isize, size: isize, seconds: isize) -> isize {
    (pos + vel * seconds).rem_euclid(size)
}

fn exercise_2(input: &Input, width: isize, height: isize) -> usize {
    let mut sets: HashMap<Vec<(isize, isize)>, isize> = HashMap::new();
    for i in 0.. {
        let map = input
            .iter()
            .map(|b| {
                let x = final_pos_after_seconds(b.pos[0], b.vel[0], width, i);
                let y = final_pos_after_seconds(b.pos[1], b.vel[1], height, i);

                (x, y)
            })
            .collect::<HashSet<_>>();

        let mut m = Vec::from_iter(map.iter().copied());
        m.sort();

        let dist = calculate_dist(&m);

        if let Some(x) = sets.insert(m, i) {
            println!("loop found at {} starting at {}", i, x);
            break;
        }
        if dist < 20 {
            let mut builder = String::new();

            builder.push_str(&format!("{} --------\n", i));

            for j in 0..height {
                for i in 0..width {
                    if map.contains(&(i, j)) {
                        builder.push('█');
                    } else {
                        builder.push(' ');
                    }
                }
                builder.push('\n');
            }
            builder.push_str("---\n");

            println!("aaa: {} at {}", dist, i);
            println!("{}", builder);
            return i as usize;
        }
    }

    unreachable!()
}

fn calculate_dist(m: &Vec<(isize, isize)>) -> usize {
    let l = m.len() * m.len();

    m.par_iter()
        .map(|(ax, ay)| {
            m.iter()
                .filter(|x| x.0 > *ax)
                .map(|(bx, by)| (bx - ax).abs() as usize + (by - ay).abs() as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
        / l
}
