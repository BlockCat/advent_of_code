use std::collections::{HashMap, HashSet};

use aoc_2021::{
    grid::{DynamicGrid, Grid},
    vector::Vector2,
};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

type Input = (Vec<bool>, DynamicGrid<bool>);

pub fn main() {
    let input = parse_input(include_str!("../input/day20.txt"));
    println!("Ex1: {}", exercise_1(&input, 2));
    println!("Ex2: {}", exercise_1(&input, 50));
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let algorithm = lines.next().unwrap();
    let algorithm = algorithm.chars().map(|c| c == '#').collect();

    let lines = lines.skip(1);
    let grid = lines.map(parse_line).collect::<Vec<_>>();
    let grid = DynamicGrid::from_vec(grid);

    (algorithm, grid)
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|x| x == '#').collect()
}

fn exercise_1((algo, grid): &Input, steps: usize) -> usize {
    let mut grid = grid.clone();

    for i in 0..steps {
        // Grow the map
        let evals = grid
            .map
            .keys()
            .par_bridge()
            .flat_map(|(x, y)| neighbouring(Vector2::new([*x, *y])))
            .collect::<HashSet<(isize, isize)>>();

        // Flip map
        grid.map = evals
            .par_iter()
            .map(|pos| {
                let index = get_index(Vector2::new([pos.0, pos.1]), &grid, i);
                let pixel = algo[index];
                (*pos, pixel)
            })
            .collect::<HashMap<_, _>>();
    }

    grid.map.values().filter(|x| **x).count()
}

fn neighbouring(pos: Vector2) -> [(isize, isize); 8] {
    [
        (pos[0] - 1, pos[1] + 1),
        (pos[0], pos[1] + 1),
        (pos[0] + 1, pos[1] + 1),
        (pos[0], pos[1] - 1),
        (pos[0], pos[1] + 1),
        (pos[0] - 1, pos[1] - 1),
        (pos[0], pos[1] - 1),
        (pos[0] + 1, pos[1] - 1),
    ]
}
fn get_index(pos: Vector2, grid: &DynamicGrid<bool>, step: usize) -> usize {
    let mut sum = 0;
    for y in -1..=1isize {
        for x in -1..=1isize {
            sum *= 2;
            if *grid.get(pos[0] + x, pos[1] + y).unwrap_or(&(step % 2 == 1)) {
                sum += 1;
            }
        }
    }
    sum
}
