use aoc_2024::{
    grid::{Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{HashMap, HashSet, VecDeque};

type Input = StaticGrid<Maze>;

pub fn main() {
    let numbers = input(include_str!("../input/day_20.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers, 2));
        println!("Exercise 2: {}", exercise_1(&numbers, 20));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> StaticGrid<Maze> {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<Maze> {
    line.chars()
        .map(|c| match c {
            '.' => Maze::None,
            '#' => Maze::Wall,
            'S' => Maze::Start,
            'E' => Maze::End,
            _ => unreachable!(),
        })
        .collect()
}

fn exercise_1(input: &Input, cheat_time: usize) -> usize {
    let start = input.iter().find(|x| x.1 == &Maze::Start).unwrap().0;
    let end = input.iter().find(|x| x.1 == &Maze::End).unwrap().0;

    let mut input = input.clone();
    input.set_vec(&start, Maze::None);
    input.set_vec(&end, Maze::None);

    let route = find_route(start, end, &input)
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();

    let route_map = route.iter().map(|x| (x.1, x.0)).collect::<HashMap<_, _>>();

    route
        .into_par_iter()
        .map(|(i, pos)| find_short_cuts(i, pos, &route_map, &input, cheat_time))
        .sum::<usize>()
}

fn find_short_cuts(
    initial_score: usize,
    start: Vector2,
    route_map: &HashMap<Vector2, usize>,
    input: &Input,
    cheat_time: usize,
) -> usize {
    let cheat_time = cheat_time as isize;

    let mut counter = 0;

    for x in -cheat_time..=cheat_time {
        let y_max = cheat_time - x.abs();
        for y in -y_max..=y_max {
            let pos = Vector2::new([x, y]);
            let rp = start + pos;
            if let Some(section) = input.get_vec(&rp) {
                if section == &Maze::None {
                    let distance = (x.abs() + y.abs()) as usize;
                    let prev_score = *route_map.get(&rp).unwrap();
                    let score = initial_score + distance;

                    if prev_score > score {
                        let shortcut = prev_score - score;
                        if shortcut >= 100 {
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    counter
}

fn find_route(start: Vector2, end: Vector2, input: &Input) -> Vec<Vector2> {
    let mut heap = VecDeque::new();
    heap.push_back((start, vec![start]));

    let mut visited = HashSet::new();

    while let Some((pos, r)) = heap.pop_front() {
        if pos == end {
            return r;
        }
        if !visited.insert(pos) {
            continue;
        }

        input
            .get_neighbours_4(&pos)
            .iter()
            .filter(|x| x.1 == &Maze::None)
            .for_each(|x| {
                let mut r = r.clone();
                r.push(x.0.clone());
                heap.push_back((x.0, r));
            });
    }

    unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Maze {
    #[default]
    None,
    Wall,
    Start,
    End,
}
