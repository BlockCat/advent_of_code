use crate::test::Bencher;
use hashbrown::{HashMap, HashSet};
use std::sync::mpsc;
use utils::intcode;
use utils::{Direction, Vector2};

#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day15.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(mut program: intcode::IntProgram) -> usize {
    let mut visited = HashMap::new();
    let mut grid: HashMap<Vector2, bool> = HashMap::new();
    grid.insert(Vector2(0, 0), false);

    let mut path: Vec<i64> = Vec::new();
    let mut current = Vector2(0, 0);
    let mut max_path = std::usize::MAX;
    let mut traced_back = false;
    loop {
        if path.len() > max_path {
            let d = reverse_direction(path.pop().unwrap());
            program.input(d);
            program.next();
            current = current + Direction::from((d - 1) as usize);
            traced_back = true;
            continue;
        }

        if visited.contains_key(&current) && path.len() != 0 && !traced_back        
        {
            let d = reverse_direction(path.pop().expect("Could not trace back path"));
            program.input(d);
            program.next();
            current = current + Direction::from((d - 1) as usize);
            traced_back = true;
            continue;
        } else {
            visited.insert(current.clone(), path.len());
        }

        let mut reverse = true;

        for i in 0..4 {
            let direction = Direction::from(i);
            let next_position = current + direction;
            // next position is not a wall and not yet visited by someone shorter.
            let contains_next_position = visited.contains_key(&next_position); // && path.len() >= visited[&next_position];

            if !*grid.entry(next_position.clone()).or_insert(false) && !contains_next_position {
                program.input((i + 1) as i64);
                match program.next() {
                    Some(0) => {
                        grid.insert(next_position, true);
                    }
                    Some(1) => {
                        path.push((i + 1) as i64);
                        current = next_position;
                        reverse = false;
                        break;
                    }
                    Some(2) => {
                        path.push((i + 1) as i64);
                        current = next_position;
                        reverse = false;
                        max_path = dbg!(std::cmp::min(max_path, path.len()));
                        //println!("{}", max_path);
                        //print_grid(&grid, &current, max_path);
                        
                    }
                    _ => panic!("Reached a end state that should not happen"),
                }
            }
        }

        if reverse && path.len() > 0 {
            let d = reverse_direction(path.pop().unwrap());
            program.input(d);
            program.next();
            current = current + Direction::from((d - 1) as usize);
            traced_back = true;
        }
        if path.len() == 0 {
            return max_path;
        }
    }
    unreachable!()
}

fn print_grid(grid: &HashMap<Vector2, bool>, pos: &Vector2, max: usize) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    println!("---");
    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| match grid.get(&utils::Vector2(x, y)) {
                Some(true) => '#',
                _ if x == pos.0 && y == pos.1 => 'D',
                _ if x.abs() + y.abs() == max as isize => 'M',
                _ => ' ',
            })
            .collect::<String>();
        println!("{}", line);
    }
    println!("---");
}

fn reverse_direction(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        c => panic!("Invalid direction: {}", c),
    }
}

fn exercise_2(mut program: intcode::IntProgram) -> usize {
    let mut visited = HashMap::new();
    let mut grid: HashMap<Vector2, bool> = HashMap::new();
    grid.insert(Vector2(0, 0), false);

    let mut path: Vec<i64> = Vec::new();
    let mut current = Vector2(0, 0);
    let mut max_path = std::usize::MAX;
    let mut traced_back = false;
    let mut repaired = Vector2(0, 0);
    loop {
        if path.len() > max_path {
            let d = reverse_direction(path.pop().unwrap());
            program.input(d);
            program.next();
            current = current + Direction::from((d - 1) as usize);
            traced_back = true;
            continue;
        }

        if visited.contains_key(&current) && path.len() != 0 && !traced_back        
        {
            let d = reverse_direction(path.pop().expect("Could not trace back path"));
            program.input(d);
            program.next();
            current = current + Direction::from((d - 1) as usize);
            traced_back = true;
            continue;
        } else {
            visited.insert(current.clone(), path.len());
        }

        let mut reverse = true;

        for i in 0..4 {
            let direction = Direction::from(i);
            let next_position = current + direction;
            // next position is not a wall and not yet visited by someone shorter.
            let contains_next_position = visited.contains_key(&next_position); // && path.len() >= visited[&next_position];

            if !*grid.entry(next_position.clone()).or_insert(false) && !contains_next_position {
                program.input((i + 1) as i64);
                match program.next() {
                    Some(0) => {
                        grid.insert(next_position, true);
                    }
                    Some(1) => {
                        path.push((i + 1) as i64);
                        current = next_position;
                        reverse = false;
                        break;
                    }
                    Some(2) => {
                        path.push((i + 1) as i64);
                        current = next_position;
                        reverse = false;
                        //max_path = dbg!(std::cmp::min(max_path, path.len()));
                        repaired = current.clone();          
                        
                    }
                    _ => panic!("Reached a end state that should not happen"),
                }
            }
        }

        if reverse && path.len() > 0 {
            let d = reverse_direction(path.pop().unwrap());
            program.input(d);
            program.next();
            current = current + Direction::from((d - 1) as usize);
            traced_back = true;
        }
        if path.len() == 0 {
            break;
        }
    }

    // j ust find the depth
    let mut hashset = HashMap::new();
    let mut deque = std::collections::VecDeque::new();
    deque.push_back((repaired, 0));

    let mut max_depth = 0;

    while let Some((x, d)) = deque.pop_front() {
        let directions = &[Direction::North, Direction::South, Direction::West, Direction::East];

        if hashset.contains_key(&x) {
            continue;
        }
        
        hashset.insert(x.clone(), d);

        max_depth = std::cmp::max(d, max_depth);

        for dir in directions {
            let next_position = x + *dir;
            if !*grid.entry(next_position).or_insert(false) {
                deque.push_back((next_position, d + 1));
            }
        }
    }


    max_depth
}

#[test]
fn d15_test() {
    let input = intcode::IntProgram::parse(include_str!("input/day15.txt"));
    assert_eq!(exercise_1(input.clone()), 193);
    assert_eq!(exercise_2(input), 10547);
}

#[bench]
fn d15_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day15.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d15_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day15.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d15_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day15.txt")));
}
