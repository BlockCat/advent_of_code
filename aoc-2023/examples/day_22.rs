use std::collections::{HashMap, HashSet};

use aoc_2023::vector::Vector3;
use rayon::iter::{ParallelBridge, ParallelIterator};

type InputType = Vec<(Vector3, Vector3)>;

pub fn main() {
    let input = parse(include_str!("../input/day_22.txt"));

    // println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    println!("Ex 1: {}", exercise_1(input.clone()));
    println!("Ex 2: {}", exercise_2(input));
    // assert_eq!(62, exercise_1(input.clone()));
    // assert_eq!(952408144115, exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Vector3, Vector3) {
    let (v1, v2) = line.split_once("~").unwrap();
    let v1 = v1
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let v2 = v2
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    (
        Vector3::new([v1[0], v1[1], v1[2]]),
        Vector3::new([v2[0], v2[1], v2[2]]),
    )
}

fn create_supporting_map(mut input: InputType) -> HashMap<usize, HashSet<usize>> {
    input.sort_by_key(|(start, end)| start[2].min(end[2]));
    let mut domain: HashMap<(isize, isize, isize), usize> = HashMap::new();
    let mut supporting_map = HashMap::new();

    for (i, (start, end)) in input.iter().enumerate() {
        let min_z = start[2].min(end[2]);
        let mut start_z = 1;

        let mut supporting = HashSet::new();

        for z in (1..min_z).rev() {
            for x in start[0]..=end[0] {
                for y in start[1]..=end[1] {
                    if let Some(i) = domain.get(&(x, y, z)) {
                        supporting.insert(*i);
                    }
                }
            }
            if supporting.len() > 0 {
                start_z = z + 1;
                break;
            }
        }

        supporting_map.insert(i, supporting);

        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                for z in start_z..=(start_z + end[2] - start[2]) {
                    domain.insert((x, y, z), i);
                }
            }
        }
    }
    supporting_map
}

// fn exercise_1(input: InputType) -> usize {
//     let ile = input.len();
//     let supporting_map = create_supporting_map(input);

//     let r = supporting_map
//         .iter()
//         .filter(|s| s.1.len() == 1)
//         .collect::<Vec<_>>();
//     let keypoints = r.iter().flat_map(|s| s.1).collect::<HashSet<_>>();

//     ile - keypoints.len()
// }

fn exercise_2(input: InputType) -> usize {
    let le = input.len();
    let supporting_map = create_supporting_map(input);

    (0..le)
        .par_bridge()
        .map(|br| count_falling(br, supporting_map.clone()))
        .sum()
}

fn count_falling(deleting_brick: usize, r: HashMap<usize, HashSet<usize>>) -> usize {
    let mut falling = HashSet::new();
    falling.insert(deleting_brick);

    let mut prev_count = 0;

    while prev_count != falling.len() {
        prev_count = falling.len();
        let depending_bricks = r
            .iter()
            .filter(|s| {
                let c = falling.intersection(&s.1).count();

                c > 0 && c == s.1.len()
            })
            .collect::<Vec<_>>();
        falling.extend(depending_bricks.iter().map(|s| *s.0));
    }

    falling.len() - 1
}
