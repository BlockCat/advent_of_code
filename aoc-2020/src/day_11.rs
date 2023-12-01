use std::{unimplemented, vec};

use utils::Grid;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Seat {
    FLOOR,
    FILLED,
    EMPTY,
}

impl Default for Seat {
    fn default() -> Self {
        Seat::EMPTY
    }
}

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day11.txt"));
    // println!("{:?}", input);
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

pub fn read_input(input: &str) -> Grid<Seat> {
    let lines = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '.' => Seat::FLOOR,
                    'L' => Seat::EMPTY,
                    '#' => Seat::FILLED,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Grid::from_vec(lines)
}

fn exercise_1(input: &Grid<Seat>) -> usize {
    let mut continue_loop = true;

    let mut prev_round = input.clone();
    let mut next_round = prev_round.clone();

    let seen_grid = {
        let mut grid = Grid::new(input.width, input.height);
        input
            .iter()
            .filter(|x| match x.1 {
                Seat::FLOOR => false,
                Seat::FILLED => true,
                Seat::EMPTY => true,
            })
            .for_each(|((x, y), v)| {
                grid.set(x, y, neighbours(&input, x, y));
            });
        grid
    };

    while continue_loop {        
        continue_loop = false;


        for ((x, y), v) in prev_round.iter() {
            let counter = seen_grid
                .get(x, y)
                .unwrap()
                .iter()
                .map(|(a, b)| prev_round.get(*a, *b).unwrap())
                .filter(|&x| x == &Seat::FILLED);

            match v {
                Seat::FLOOR => {}
                Seat::FILLED => {
                    let n = counter.take(4).count();
                    if n >= 4 {
                        continue_loop = true;
                        next_round.set(x, y, Seat::EMPTY);
                    } else {
                        next_round.set(x, y, Seat::FILLED);
                    }
                }
                Seat::EMPTY => {
                    let n = counter.take(1).count();
                    if n == 0 {
                        continue_loop = true;
                        next_round.set(x, y, Seat::FILLED);
                    } else {
                        next_round.set(x, y, Seat::EMPTY);
                    }
                }
            }
        }

        let t = prev_round;
        prev_round = next_round;
        next_round = t;
    }

    // println!("r: {}", round);
    prev_round
        .grid
        .iter()
        .filter(|x| **x == Seat::FILLED)
        .count()
}

fn exercise_2(input: &Grid<Seat>) -> usize {
    let mut continue_loop = true;

    let mut prev_round = input.clone();
    let mut next_round = prev_round.clone();

    let seen_grid = {
        let mut grid = Grid::new(input.width, input.height);
        input
            .iter()
            .filter(|x| match x.1 {
                Seat::FLOOR => false,
                Seat::FILLED => true,
                Seat::EMPTY => true,
            })
            .for_each(|((x, y), v)| {
                grid.set(x, y, seen_neighbours(&input, x, y));
            });
        grid
    };

    while continue_loop {
        continue_loop = false;

        for ((x, y), v) in prev_round.iter() {
            let counter = seen_grid
                .get(x, y)
                .unwrap()
                .iter()
                .map(|(a, b)| prev_round.get(*a, *b).unwrap())
                .filter(|&x| x == &Seat::FILLED);

            match v {
                Seat::FLOOR => {}
                Seat::FILLED => {
                    let n = counter.take(5).count();
                    if n >= 5 {
                        continue_loop = true;
                        next_round.set(x, y, Seat::EMPTY);
                    } else {
                        next_round.set(x, y, Seat::FILLED);
                    }
                }
                Seat::EMPTY => {
                    let n = counter.take(1).count();
                    if n == 0 {
                        continue_loop = true;
                        next_round.set(x, y, Seat::FILLED);
                    } else {
                        next_round.set(x, y, Seat::EMPTY);
                    }
                }
            }
        }

        let t = prev_round;
        prev_round = next_round;
        next_round = t;
    }

    prev_round
        .grid
        .iter()
        .filter(|x| **x == Seat::FILLED)
        .count()
}

fn neighbours(grid: &Grid<Seat>, x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (-1isize, -1isize),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .filter_map(|(dx, dy)| {
        let nx = ((x as isize) + dx) as usize;
        let ny = ((y as isize) + dy) as usize;
        match grid.get(nx, ny) {
            Some(Seat::FILLED) => Some((nx, ny)),
            Some(Seat::EMPTY) => Some((nx, ny)),
            _ => None,
        }
    })
    .collect()
}

fn seen_neighbours(grid: &Grid<Seat>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let w = grid.width;
    let h = grid.height;
    vec![
        (-1isize, -1isize),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .filter_map(|(dx, dy)| {
        let dw = if dx > 0 {
            w - x
        } else if dx < 0 {
            x
        } else {
            std::usize::MAX
        };
        let dh = if dy > 0 {
            h - y
        } else if dy < 0 {
            y
        } else {
            std::usize::MAX
        };
        for i in 1..=(dw.min(dh)) {
            let nx = ((x as isize) + dx * i as isize) as usize;
            let ny = ((y as isize) + dy * i as isize) as usize;

            match grid.get(nx, ny) {
                Some(Seat::FILLED) => {
                    return Some((nx, ny));
                }
                Some(Seat::EMPTY) => {
                    return Some((nx, ny));
                }
                _ => {}
            }
        }
        return None;
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;
    #[test]
    fn d11p1_test() {
        let input = read_input(include_str!("input/day11.txt"));
        assert_eq!(2441, exercise_1(&input));
    }

    #[test]
    fn d11p2_test() {
        let input = read_input(include_str!("input/day11.txt"));
        assert_eq!(2190, exercise_2(&input));
    }

    #[bench]
    fn d11_bench_parse(b: &mut Bencher) {
        b.iter(|| read_input(include_str!("input/day11.txt")));
    }
    #[bench]
    fn d11_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day11.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d11_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day11.txt"));
        b.iter(|| exercise_2(&input));
    }
}
