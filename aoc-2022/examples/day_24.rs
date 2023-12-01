use aoc_2022::{
    direction::{Direction, ALL_DIRECTIONS},
    grid::{Grid, StaticGrid},
    vector::{Vector2, VectorN},
};
use std::{
    borrow::Cow,
    collections::{BinaryHeap, HashMap, HashSet},
};

type InputType = StaticGrid<char>;

type BlizzardMap = StaticGrid<u8>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    StaticGrid::from_vec(
        include_str!("../input/day_24.txt")
            .lines()
            .map(parse_line)
            .collect(),
    )
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: InputType) -> usize {
    let start = Vector2::new([1, 0]);
    let end = Vector2::new([input.width as isize - 2, input.height as isize - 1]);

    let blizzards = extract_blizzards(input.clone());

    let (steps, _) = find_path(start, blizzards, end, &input);
    steps
}

fn exercise_2(input: InputType) -> usize {
    let start = Vector2::new([1, 0]);
    let end = Vector2::new([input.width as isize - 2, input.height as isize - 1]);

    let blizzards = extract_blizzards(input.clone());

    let (steps_1, blizzards) = find_path(start, blizzards, end, &input);
    println!("forward: {}", steps_1);
    let (steps_2, blizzards) = find_path(end, blizzards, start, &input);
    println!("backward: {}", steps_2);
    let (steps_3, _) = find_path(start, blizzards, end, &input);
    println!("forward: {}", steps_3);

    steps_1 + steps_2 + steps_3
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrderEntry {
    p: u16,
    steps: u16,
    pos: Vector2,
}

impl Ord for OrderEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for OrderEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.p.partial_cmp(&other.p).map(|x| x.reverse())
    }
}

fn find_path<'a>(
    start: VectorN<2>,
    blizzards: BlizzardMap,
    end: VectorN<2>,
    input: &StaticGrid<char>,
) -> (usize, StaticGrid<u8>) {
    let mut queue = BinaryHeap::default();
    let mut visited = HashSet::new();
    let mut blizzard_map = HashMap::new();
    blizzard_map.insert(0u16, blizzards);
    queue.push(OrderEntry {
        p: Vector2::manhattan(&start, &end) as u16,
        steps: 0,
        pos: start,
    });

    while let Some(OrderEntry { p: _, steps, pos }) = queue.pop() {
        if !visited.insert((pos, steps)) {
            continue;
        }

        let field = if let Some(field) = blizzard_map.get(&steps) {
            field
        } else {
            let field = blizzard_step(blizzard_map.get(&(steps - 1)).unwrap());
            blizzard_map.insert(steps, field);
            blizzard_map.get(&steps).unwrap()
        };

        if pos == end {
            return (steps as usize, blizzard_map.remove(&steps).unwrap());
        }

        let field = blizzard_step(field);

        for dir in ALL_DIRECTIONS {
            let pos = pos + dir;
            if field.get_vec(&pos).unwrap_or(&std::u8::MAX) == &0
                && input.get_vec(&pos) != Some(&'#')
            {
                queue.push(OrderEntry {
                    p: steps + 1 + Vector2::manhattan(&pos, &end) as u16,
                    steps: steps + 1,
                    pos,
                });
            }
        }
        if field.get_vec(&pos).unwrap_or(&std::u8::MAX) == &0 {
            queue.push(OrderEntry {
                p: steps + 1 + Vector2::manhattan(&pos, &end) as u16,
                steps: steps + 1,
                pos,
            });
        }
    }
    unreachable!()
}

fn blizzard_step(grid: &BlizzardMap) -> BlizzardMap {
    let mut ngrid = StaticGrid::new(grid.width, grid.height);

    for (pos, dirs) in grid.iter() {
        if dirs == &0 {
            continue;
        }
        for dir in ALL_DIRECTIONS {
            if (dirs & (1 << dir as u8)) != 0 {
                let p = pos + dir;
                let p = match dir {
                    Direction::North if p[1] == 0 => Vector2::new([p[0], grid.height as isize - 2]),
                    Direction::South if p[1] == grid.height as isize - 1 => Vector2::new([p[0], 1]),
                    Direction::East if p[0] == grid.width as isize - 1 => Vector2::new([1, p[1]]),
                    Direction::West if p[0] == 0 => Vector2::new([grid.width as isize - 2, p[1]]),
                    _ => p,
                };
                *ngrid.get_mut_vec(&p).unwrap() |= 1 << dir as u8;
            }
        }
    }

    ngrid
}
fn extract_blizzards(input: StaticGrid<char>) -> BlizzardMap {
    let blizzards = input
        .iter()
        .filter_map(|(a, b)| {
            let dir = match b {
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                _ => None,
            };
            dir.map(|d| (a, d))
        })
        .collect::<Vec<_>>();
    let blizzards = {
        let mut x: StaticGrid<u8> = StaticGrid::new(input.width, input.height);

        for (v, d) in blizzards {
            *x.get_mut_vec(&v).unwrap() |= 1 << d as u8;
        }
        x
    };

    blizzards
}
