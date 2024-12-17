use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

use aoc_2024::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};

type Input = StaticGrid<Maze>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Maze {
    #[default]
    None,
    Wall,
    Start,
    End,
}

pub fn main() {
    let numbers = input(include_str!("../input/day_16.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
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

fn exercise_1(input: &Input) -> usize {
    let start = input.iter().find(|x| x.1 == &Maze::Start).unwrap().0;
    let end = input.iter().find(|x| x.1 == &Maze::End).unwrap().0;

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), Direction::East, start));

    let mut visited = HashSet::new();
    while let Some((Reverse(score), reindeer_dir, reindeer_pos)) = heap.pop() {
        if reindeer_pos == end {
            return score;
        }

        if !visited.insert((reindeer_pos, reindeer_dir)) {
            continue;
        }

        let next_pos = reindeer_pos + reindeer_dir;

        if Some(&Maze::Wall) != input.get_vec(&next_pos) {
            heap.push((Reverse(score + 1), reindeer_dir, next_pos));
        }

        heap.push((Reverse(score + 1000), reindeer_dir.left(), reindeer_pos));
        heap.push((Reverse(score + 1000), reindeer_dir.right(), reindeer_pos));
    }

    unreachable!()
}
fn exercise_2(input: &Input) -> usize {
    let start = input.iter().find(|x| x.1 == &Maze::Start).unwrap().0;
    let end = input.iter().find(|x| x.1 == &Maze::End).unwrap().0;

    let best_path = exercise_1(input);

    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), Direction::East, start, None));

    let mut best_tiles = HashSet::new();

    best_tiles.insert(start);
    best_tiles.insert(end);

    let mut paths =
        HashMap::<(Vector2, Direction), (Vec<Option<(Vector2, Direction)>>, usize)>::new();

    while let Some((Reverse(score), reindeer_dir, reindeer_pos, prev_pos)) = heap.pop() {
        if let Some((a, existing)) = paths.get_mut(&(reindeer_pos, reindeer_dir)) {
            if score == *existing {
                a.push(prev_pos);
            }
            continue;
        }

        paths.insert((reindeer_pos, reindeer_dir), (vec![prev_pos], score));

        if reindeer_pos == end {
            let mut queue = vec![(reindeer_pos, reindeer_dir)];

            while let Some((pp, pd)) = queue.pop() {
                if let Some((a, _)) = paths.get(&(pp, pd)) {
                    a.iter().flat_map(|x| x).for_each(|x| {
                        queue.push(x.clone());
                        best_tiles.insert(x.0);
                    });
                }
            }
            continue;
        }

        if score > best_path {
            continue;
        }

        let next_pos = reindeer_pos + reindeer_dir;

        if Some(&Maze::Wall) != input.get_vec(&next_pos) {
            heap.push((
                Reverse(score + 1),
                reindeer_dir,
                next_pos,
                Some((reindeer_pos, reindeer_dir)),
            ));
        }

        heap.push((
            Reverse(score + 1000),
            reindeer_dir.left(),
            reindeer_pos,
            Some((reindeer_pos, reindeer_dir)),
        ));
        heap.push((
            Reverse(score + 1000),
            reindeer_dir.right(),
            reindeer_pos,
            Some((reindeer_pos, reindeer_dir)),
        ));
    }

    best_tiles.len()
}

fn pretty_print(input: &StaticGrid<Maze>, tiles: &HashSet<Vector2>) {
    input.pretty_print(|m, p| {
        if tiles.contains(&p) {
            'O'
        } else {
            match m {
                Maze::Wall => '#',
                Maze::Start => 'S',
                Maze::End => 'E',
                Maze::None => '.',
            }
        }
    });
}
