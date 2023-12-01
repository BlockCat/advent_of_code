use crate::test::Bencher;
use hashbrown::HashMap;
use hashbrown::HashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use utils::Grid;
use utils::{Direction, Vector2};

#[derive(Debug, Clone, PartialEq, Eq)]
enum MazeTile {
    Empty,
    Void,
    Wall,
    Portal(char),
    Connection(Vector2),
}

impl Default for MazeTile {
    fn default() -> Self {
        MazeTile::Empty
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

//#[test]
pub fn run() {
    let input = read_input(include_str!("input/day20.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input)); // 7658
}

fn read_input(input: &str) -> (Grid<MazeTile>, Vector2, Vector2) {
    let mut grid = Grid::from_vec(input.lines().map(read_line).collect());
    let mut map = HashMap::new();

    let mut start = Vector2(0, 0);
    let mut end = Vector2(0, 0);

    for y in 0..grid.height - 1 {
        for x in 0..grid.width - 1 {
            let pos = Vector2(x as isize, y as isize);
            if let Some(MazeTile::Portal(a)) = grid.get_vec(&pos) {
                let result = if let Some(MazeTile::Portal(c)) =
                    grid.get_vec(&(pos + Direction::East))
                {
                    let next_pos = if let Some(MazeTile::Empty) =
                        grid.get_vec(&((pos + Direction::East) + Direction::East))
                    {
                        pos + Direction::East + Direction::East
                    } else {
                        pos + Direction::West
                    };

                    Some((c, next_pos))
                } else if let Some(MazeTile::Portal(c)) = grid.get_vec(&(pos + Direction::South)) {
                    let next_pos = if let Some(MazeTile::Empty) =
                        grid.get_vec(&((pos + Direction::South) + Direction::South))
                    {
                        pos + Direction::South + Direction::South
                    } else {
                        pos + Direction::North
                    };

                    Some((c, next_pos))
                } else {
                    None
                };
                if let Some((b, np)) = result {
                    if (*a, *b) == ('A', 'A') {
                        start = np;
                    }
                    if (*a, *b) == ('Z', 'Z') {
                        end = np;
                    }
                    if let Some((opos, opla)) = map.insert((*a, *b), (pos, np)) {
                        // hey we found our match
                        grid.set_vec(&opla, MazeTile::Connection(np));
                        grid.set_vec(&np, MazeTile::Connection(opla));
                    }
                }
            }
        }
    }

    (grid, start, end)
}

fn read_line(line: &str) -> Vec<MazeTile> {
    line.chars()
        .map(|c| match c {
            '.' => MazeTile::Empty,
            '#' => MazeTile::Wall,
            ' ' => MazeTile::Void,
            c if 'A' as u8 <= c as u8 && c as u8 <= 'Z' as u8 => MazeTile::Portal(c),
            c => panic!("Invalid maze char: {}", c),
        })
        .collect()
}

fn exercise_1((grid, start, end): (Grid<MazeTile>, Vector2, Vector2)) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((node, steps)) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        if node == end {
            return steps;
        }

        for dir in &DIRECTIONS {
            match grid.get_vec(&(node + *dir)) {
                Some(MazeTile::Empty) => {
                    queue.push_back((node + *dir, steps + 1));
                }
                Some(MazeTile::Connection(next)) => {
                    queue.push_back((*next, steps + 2));
                }
                None => {}
                Some(MazeTile::Wall) => {}
                Some(MazeTile::Portal(_)) => {}
                c => panic!("Unexpected: {:?}", c),
            }
        }
    }

    unreachable!()
}

fn find_path(grid: &Grid<MazeTile>, start: Vector2, end: Vector2) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((node, steps)) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        if node == end {
            return Some(steps);
        }

        for dir in &DIRECTIONS {
            let next_pos = node + *dir;
            match grid.get_vec(&next_pos) {
                Some(MazeTile::Empty) | Some(MazeTile::Connection(_)) => {
                    queue.push_back((node + *dir, steps + 1));
                }
                None => {}
                Some(MazeTile::Wall) => {}
                Some(MazeTile::Portal(_)) => {}
                c => panic!("Unexpected: {:?}", c),
            }
        }
    }
    None
}

fn precompute(
    grid: &Grid<MazeTile>,
    start: &Vector2,
    end: &Vector2,
) -> (
    HashMap<(Vector2, Vector2), usize>,
    HashMap<Vector2, Vec<Vector2>>,
) {
    let mut precomputed: HashMap<(Vector2, Vector2), usize> = HashMap::new();
    let mut reachable = HashMap::new();

    let mut portals = Vec::new();
    portals.push(*start);
    portals.push(*end);
    for y in 2..grid.height - 2 {
        for x in 2..grid.width - 2 {
            if let Some(MazeTile::Connection(o)) = grid.get(x, y) {
                portals.push(Vector2(x as isize, y as isize));
            }
        }
    }

    for a in 0..portals.len() {
        for b in a + 1..portals.len() {
            if let Some(steps) = find_path(&grid, portals[a], portals[b]) {
                precomputed.insert((portals[a], portals[b]), steps);
                precomputed.insert((portals[b], portals[a]), steps);
                reachable
                    .entry(portals[a])
                    .or_insert(Vec::new())
                    .push(portals[b]);
                reachable
                    .entry(portals[b])
                    .or_insert(Vec::new())
                    .push(portals[a]);
            }
        }
    }

    (precomputed, reachable)
}
fn exercise_2((grid, start, end): (Grid<MazeTile>, Vector2, Vector2)) -> usize {
    let (precomputed, reachable) = precompute(&grid, &start, &end);

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    let mut max_steps = std::usize::MAX;

    queue.push((Reverse(0), start, 0));

    while let Some((steps, node, level)) = queue.pop() {
        if node == end && level == 0 {
            return steps.0;
        }

        if !visited.insert((node, level)) {
            continue;
        }

        if steps.0 > max_steps {
            continue;
        }

        //println!("{} -- {} ", steps.0, level);
        for portal in &reachable[&node] {
            let new_steps = steps.0 + precomputed[&(node, *portal)];
            let tile = grid.get_vec(portal);

            if portal == &start {
                continue;
            }
            if portal == &end {
                if level == 0 {
                    println!("re: {}", new_steps);
                    max_steps = new_steps;
                    queue.push((Reverse(new_steps), end, 0));
                }
                continue;
            }
            assert!(portal.0 >= 2 && portal.0 <= grid.width as isize - 3);
            assert!(portal.1 >= 2 && portal.1 <= grid.height as isize - 3);
            let outside = portal.0 == 2
                || portal.0 == grid.width as isize - 3
                || portal.1 == 2
                || portal.1 == grid.height as isize - 3;

            if outside && level == 0 {
                continue;
            }

            let next_level = if outside { level - 1 } else { level + 1 };
            match tile {
                Some(MazeTile::Connection(dest)) => {
                    queue.push((Reverse(new_steps + 1), *dest, next_level))
                }
                c => panic!("Unsuported tile: {:?}", c),
            }
        }
    }

    unreachable!()
}

#[test]
fn d20_test() {
    println!("1");
    assert_eq!(
        exercise_1(read_input(include_str!("input/day20_test_1.txt"))),
        23
    );
    println!("2");
    assert_eq!(
        exercise_1(read_input(include_str!("input/day20_test_2.txt"))),
        58
    );

    println!("3");
    assert_eq!(
        exercise_2(read_input(include_str!("input/day20_test_1.txt"))),
        26
    );
    println!("4");
    assert_eq!(
        exercise_2(read_input(include_str!("input/day20_test_3.txt"))),
        396
    );
}

#[bench]
fn d20_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day20.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d20_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day20.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d20_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day20.txt")));
}
