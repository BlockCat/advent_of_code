use crate::test::Bencher;
use hashbrown::HashMap;
use hashbrown::HashSet;

use utils::Direction;
use utils::Grid;
use utils::Vector2;

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

//#[test]
pub fn run() {
    let input = read_input(include_str!("input/day24.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input, 200));
}

fn iteration_1(input: Grid<bool>) -> Grid<bool> {
    let mut output = Grid::new(input.width, input.height);
    for y in 0..input.height {
        for x in 0..input.width {
            let pos = Vector2(x as isize, y as isize);

            let alive = DIRECTIONS
                .iter()
                .filter(|x| input.get_vec(&(pos + **x)) == Some(&true))
                .count();
            if input.get_vec(&pos) == Some(&true) {
                if alive == 1 {
                    output.set_vec(&pos, true)
                }
            } else {
                if alive == 1 || alive == 2 {
                    output.set_vec(&pos, true);
                }
            }
        }
    }
    output
}

fn iteration_2(
    inner: Option<&Grid<bool>>,
    middle: &Grid<bool>,
    outer: Option<&Grid<bool>>,
) -> Grid<bool> {
    let mut output = Grid::new(middle.width, middle.height);

    let (outer_up, outer_down, outer_left, outer_right) = if let Some(outer) = outer {
        (
            *outer.get(2, 1).unwrap() as u32,
            *outer.get(2, 3).unwrap() as u32,
            *outer.get(1, 2).unwrap() as u32,
            *outer.get(3, 2).unwrap() as u32,
        )
    } else {
        (0, 0, 0, 0)
    };

    let (inner_up, inner_down, inner_left, inner_right) = if let Some(inner) = inner {
        (
            (0..5).map(|x| *inner.get(x, 0).unwrap() as u32).sum(),
            (0..5).map(|x| *inner.get(x, 4).unwrap() as u32).sum(),
            (0..5).map(|x| *inner.get(0, x).unwrap() as u32).sum(),
            (0..5).map(|x| *inner.get(4, x).unwrap() as u32).sum(),
        )
    } else {
        (0, 0, 0, 0)
    };

    // println!("o: {}, {}, {}, {}", outer_up, outer_down, outer_left, outer_right);
    // println!("i: {}, {}, {}, {}", inner_up, inner_down, inner_left, inner_right);

    for y in 0..middle.height {
        for x in 0..middle.width {
            if x == 2 && y == 2 {
                continue;
            }
            let pos = Vector2(x as isize, y as isize);
            let alive = DIRECTIONS
                .iter()
                .map(|x| {
                    let next_pos = pos + *x;
                    match (next_pos, *x) {
                        (Vector2(-1, _), _) => outer_left,
                        (Vector2(5, _), _) => outer_right,
                        (Vector2(_, -1), _) => outer_up,
                        (Vector2(_, 5), _) => outer_down,
                        (Vector2(2, 2), Direction::South) => inner_up,
                        (Vector2(2, 2), Direction::North) => inner_down,
                        (Vector2(2, 2), Direction::East) => inner_left,
                        (Vector2(2, 2), Direction::West) => inner_right,
                        (c, _) => (middle.get_vec(&c) == Some(&true)) as u32,
                    }
                })
                .sum::<u32>();

            if middle.get_vec(&pos) == Some(&true) {
                if alive == 1 {
                    output.set_vec(&pos, true)
                }
            } else {
                if alive == 1 || alive == 2 {
                    output.set_vec(&pos, true);
                }
            }
        }
    }
    output
}

fn exercise_1(mut input: Grid<bool>) -> u32 {
    let mut visited = HashSet::new();
    visited.insert(input.clone());
    loop {
        input = iteration_1(input);        
        if !visited.insert(input.clone()) {
            let mut sum = 0;
            for i in 0..(input.width * input.height) {
                sum += (input.grid[i] as u32) << i
            }
            return sum;
        }
    }
}

pub fn print(grid: &Grid<bool>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!(
                "{}",
                if grid.grid[x + y * grid.width] {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}

fn exercise_2(input: Grid<bool>, steps: usize) -> u32 {
    let mut map = HashMap::with_capacity(200);
    map.insert(0i32, input.clone());

    for _ in 0..steps {
        let mut new_map = HashMap::new();
        for (level, level_grid) in &map {
            let inner = map.get(&(level + 1));
            let outer = map.get(&(level - 1));
            let step = iteration_2(inner, level_grid, outer);
            new_map.insert(*level, step);

            if inner.is_none() {
                new_map.insert(
                    level + 1,
                    iteration_2(
                        None,
                        &Grid::new(input.width, input.height),
                        Some(level_grid),
                    ),
                );
            }
            if outer.is_none() {
                new_map.insert(
                    level - 1,
                    iteration_2(
                        Some(level_grid),
                        &Grid::new(input.width, input.height),
                        None,
                    ),
                );
            }
        }
        map = new_map;
    }

    map.values()
        .map(|g| g.grid.iter().filter(|x| **x).count() as u32)
        .sum()
}

fn read_input(input: &str) -> Grid<bool> {
    Grid::from_vec(
        input
            .lines()
            .map(|x| x.trim().chars().map(|x| x == '#').collect::<Vec<_>>())
            .collect(),
    )
}

#[test]
fn d24_test() {
    let input = read_input(
        r"....#
    #..#.
    #..##
    ..#..
    #....",
    );
    assert_eq!(exercise_1(input.clone()), 2129920);
    assert_eq!(exercise_2(input, 10), 99);
}

#[bench]
fn d24_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day24.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d24_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day24.txt"));
    b.iter(|| exercise_2(input.clone(), 200));
}

#[bench]
fn d24_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day24.txt")));
}
