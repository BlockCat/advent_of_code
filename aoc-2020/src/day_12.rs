use std::{unimplemented, vec};

use utils::Vector2;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day12.txt"));
    // println!("{:?}", input);
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

pub fn read_input(input: &str) -> Vec<(char, isize)> {
    input
        .lines()
        .map(|x| (x[0..1].chars().next().unwrap(), x[1..].parse().unwrap()))
        .collect::<Vec<_>>()
}

fn exercise_1(input: &Vec<(char, isize)>) -> isize {
    let ship = input
        .iter()
        .fold((Vector2(0, 0), 0isize), |(ship, dir), (c, val)| match c {
            'N' => (ship + Vector2(0, -val), dir),
            'S' => (ship + Vector2(0, *val), dir),
            'E' => (ship + Vector2(*val, 0), dir),
            'W' => (ship + Vector2(-val, 0), dir),
            'L' => (ship, dir - *val),
            'R' => (ship, dir + *val),
            'F' => match dir.rem_euclid(360) {
                0 => (ship + Vector2(*val, 0), dir),
                90 => (ship + Vector2(0, *val), dir),
                180 => (ship + Vector2(-val, 0), dir),
                270 => (ship + Vector2(0, -val), dir),
                _ => unreachable!(dir.rem_euclid(360)),
            },
            _ => unreachable!(),
        })
        .0;

    println!("{:?}", ship);

    ship.0.abs() + ship.1.abs()
}

fn exercise_2(input: &Vec<(char, isize)>) -> isize {
    let rotate_90 = [[0, -1], [1, 0]];
    let rotate_180 = [[-1, 0], [0, -1]];
    let rotate_270 = [[0, 1], [-1, 0]];

    let waypoint = input
        .iter()
        .fold(
            (Vector2(0, 0), Vector2(10, -1)),
            |(ship, waypoint), (c, val)| match (c, val) {
                ('N', _) => (ship, waypoint + Vector2(0, -val)),
                ('S', _) => (ship, waypoint + Vector2(0, *val)),
                ('E', _) => (ship, waypoint + Vector2(*val, 0)),
                ('W', _) => (ship, waypoint + Vector2(-val, 0)),
                ('L', 90) => (ship, waypoint * rotate_90),
                ('L', 180) => (ship, waypoint * rotate_180),
                ('L', 270) => (ship, waypoint * rotate_270),
                ('R', 90) => (ship, waypoint * rotate_270),
                ('R', 180) => (ship, waypoint * rotate_180),
                ('R', 270) => (ship, waypoint * rotate_90),
                ('F', _) => (ship + waypoint * *val, waypoint),
                _ => unreachable!(),
            },
        )
        .0;

    println!("{:?}", waypoint);

    waypoint.0.abs() + waypoint.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;
    #[test]
    fn d12p1_test() {
        let input = read_input(include_str!("input/day12.txt"));
        assert_eq!(1007, exercise_1(&input));
    }

    #[test]
    fn d12p2_test() {
        let input = read_input(include_str!("input/day12.txt"));
        assert_eq!(41212, exercise_2(&input));
    }

    #[bench]
    fn d12_bench_parse(b: &mut Bencher) {
        b.iter(|| read_input(include_str!("input/day12.txt")));
    }
    #[bench]
    fn d12_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day12.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d12_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day12.txt"));
        b.iter(|| exercise_2(&input));
    }
}
