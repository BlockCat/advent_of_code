use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    sync::atomic::AtomicBool,
};

use aoc_2022::{direction::Direction, grid::StaticGrid, vector::Vector2};
use dashmap::DashMap;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

type InputType = HashSet<Vector2>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    StaticGrid::from_vec(
        include_str!("../input/day_23.txt")
            .lines()
            .map(parse_line)
            .collect(),
    )
    .iter()
    .filter(|x| x.1 == &'#')
    .map(|x| x.0.clone())
    .collect::<HashSet<_>>()
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(mut elves: InputType) -> usize {
    let mut dirs = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for _ in 0..10 {
        do_elve_step(&mut elves, &mut dirs);
    }

    let xmin = elves.iter().map(|x| x[0]).min().unwrap();
    let xmax = elves.iter().map(|x| x[0]).max().unwrap();
    let ymin = elves.iter().map(|x| x[1]).min().unwrap();
    let ymax = elves.iter().map(|x| x[1]).max().unwrap();

    ((xmax - xmin + 1) * (ymax - ymin + 1)) as usize - elves.len()
}
fn exercise_2(mut elves: InputType) -> usize {
    let mut dirs = VecDeque::from(vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for i in 1.. {
        let moved = do_elve_step(&mut elves, &mut dirs);

        if !moved {
            return i;
        }
    }

    unreachable!()
}

fn do_elve_step(
    elves: &mut HashSet<aoc_2022::vector::VectorN<2>>,
    dirs: &mut VecDeque<Direction>,
) -> bool {
    let proposals = DashMap::new();
    let map: DashMap<Vector2, usize> = Default::default();
    elves.par_iter().for_each(|elve| {
        let nw = elves.contains(&(elve.clone() + Direction::North + Direction::West));
        let n = elves.contains(&(elve.clone() + Direction::North));
        let ne = elves.contains(&(elve.clone() + Direction::North + Direction::East));

        let w = elves.contains(&(elve.clone() + Direction::West));
        let e = elves.contains(&(elve.clone() + Direction::East));

        let sw = elves.contains(&(elve.clone() + Direction::South + Direction::West));
        let s = elves.contains(&(elve.clone() + Direction::South));
        let se = elves.contains(&(elve.clone() + Direction::South + Direction::East));

        if nw || n || ne || w || e || sw || e || sw || s || se {
            let proposal = dirs
                .iter()
                .filter(|x| match x {
                    Direction::North => !nw && !n && !ne,
                    Direction::East => !e && !ne && !se,
                    Direction::South => !s && !se && !sw,
                    Direction::West => !w && !nw && !sw,
                })
                .next();

            if let Some(dir) = proposal {
                let proposal_vec = elve.clone() + *dir;
                proposals.insert(elve.clone(), proposal_vec.clone());
                *map.entry(proposal_vec).or_default() += 1;
            }
        }
    });

    let moved = AtomicBool::new(false);

    *elves = elves
        .par_iter()
        .map(|elve| {
            // It made a proposal
            if let Some(prop) = proposals.get(&elve) {
                if map.get(prop.value()).unwrap().value() == &1 {
                    moved.store(true, std::sync::atomic::Ordering::Relaxed);
                    prop.clone()
                } else {
                    *elve
                }
            } else {
                *elve
            }
        })
        .collect();
    dirs.rotate_left(1);

    moved.into_inner()
}

#[derive(Debug)]
pub struct Elve {}
