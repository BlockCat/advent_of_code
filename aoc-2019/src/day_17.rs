use crate::test::Bencher;
use hashbrown::HashMap;

use utils::intcode;
use utils::{Direction, Vector2};

#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day17.txt"));
    //println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

#[derive(PartialEq, Eq, Clone)]
enum GridState {
    Scaffold,
    Empty,
}

struct Grid {
    pub grid: HashMap<Vector2, GridState>,
    pub dimensions: (usize, usize),
    pub robot: Vector2,
    pub robot_direction: Direction,
}

fn create_grid(mut program: intcode::IntProgram) -> Grid {
    let mut grid: HashMap<Vector2, GridState> = HashMap::new();

    let mut x = 0usize;
    let mut y = 0usize;

    let mut width = 0;
    let mut height = 0;
    let mut rx = 0;
    let mut ry = 0;
    let mut rd = Direction::North;

    while let Some(code) = program.next() {
        match (code as u8) as char {
            '#' => {
                grid.insert(Vector2(x as isize, y as isize), GridState::Scaffold);
                x += 1;
            }
            '.' => {
                grid.insert(Vector2(x as isize, y as isize), GridState::Empty);
                x += 1;
            }
            '\n' => {
                width = std::cmp::max(width, x);
                height = std::cmp::max(height, y);
                x = 0;
                y += 1;
            }
            '^' => {
                rx = x;
                ry = y;
                grid.insert(Vector2(x as isize, y as isize), GridState::Scaffold);
                x += 1;
                rd = Direction::North;
            }
            '>' => {
                rx = x;
                ry = y;
                grid.insert(Vector2(x as isize, y as isize), GridState::Scaffold);
                x += 1;
                rd = Direction::East;
            }
            '<' => {
                rx = x;
                ry = y;
                grid.insert(Vector2(x as isize, y as isize), GridState::Scaffold);
                x += 1;
                rd = Direction::West;
            }
            'v' => {
                rx = x;
                ry = y;
                grid.insert(Vector2(x as isize, y as isize), GridState::Scaffold);
                x += 1;
                rd = Direction::South;
            }
            c => println!("huh? {}", c), //panic!("Error: {} not found", c),
        }
    }

    Grid {
        grid,
        dimensions: (width, height),
        robot: Vector2(rx as isize, ry as isize),
        robot_direction: rd,
    }
}

fn exercise_1(program: intcode::IntProgram) -> usize {
    let grid = create_grid(program);
    let mut sum = 0;
    for key in grid.grid.keys() {
        let a = &[
            key,
            &(*key + Direction::North),
            &(*key + Direction::South),
            &(*key + Direction::West),
            &(*key + Direction::East),
        ];
        if a.into_iter()
            .all(|x| grid.grid.get(x).unwrap_or(&GridState::Empty) == &GridState::Scaffold)
        {
            sum += key.0 * key.1;
        }
    }

    sum as usize
}

fn print_grid(grid: &HashMap<Vector2, GridState>, pos: &Vector2, direction: &Direction) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    println!("---");
    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| match grid.get(&utils::Vector2(x, y)) {
                _ if x == pos.0 && y == pos.1 => match direction {
                    Direction::North => '^',
                    Direction::South => 'v',
                    Direction::West => '<',
                    Direction::East => '>',
                },
                Some(GridState::Scaffold) => '#',
                _ => ' ',
            })
            .collect::<String>();
        println!("{}", line);
    }
    println!("---");
}

fn find_path(program: intcode::IntProgram) {
    let grid = create_grid(program);

    let mut direction = grid.robot_direction;
    let mut position = grid.robot;
    // move forward
    loop {
        let mut dist = 0;
        position = position + direction;
        while let Some(GridState::Scaffold) = grid.grid.get(&position) {
            dist += 1;
            position = position + direction;
        }

        // check left
        // check right
        position = position - direction;
        //println!("{:?}, {:?}", position, direction);
        print!("{},", dist);
        match (
            grid.grid.get(&(position + direction.left())),
            grid.grid.get(&(position + direction.right())),
        ) {
            (Some(GridState::Scaffold), _) => {
                print!("L,");
                direction = direction.left();
            }
            (_, Some(GridState::Scaffold)) => {
                print!("R,");
                direction = direction.right();
            }
            _ => {
                print!("Done");
                return;
            }
        }
    }
}
fn exercise_2(mut program: intcode::IntProgram) -> usize {
    find_path(program.clone());

    
    //return 0;
    program.memory[0] = 2;
    provide_array(&mut program, "C,A,C,A,B,C,A,B,C,B\n");
    provide_array(&mut program, "L,8,L,6,L,10,L,6\n");
    provide_array(&mut program, "R,6,L,8,L,10,R,6\n");
    provide_array(&mut program, "R,6,L,6,L,10\n"); 

    let debug = false;
    if debug {
        provide_array(&mut program, "y\n");
    } else {
        provide_array(&mut program, "n\n");
    }

    let output = program.collect::<Vec<_>>();

    let dust = output.last().unwrap();
/*
    for c in &output {
        print!("{}", (*c as u8) as char);
    }*/

    *dust as usize
}

fn provide_array(program: &mut intcode::IntProgram, array: &str) {
    for c in array.chars() {
        program.input(c as i64);
    }
}

#[test]
fn d17_test() {
    let input = intcode::IntProgram::parse(include_str!("input/day17.txt"));
    assert_eq!(exercise_1(input.clone()), 1544);
    assert_eq!(exercise_2(input), 696373);
}

#[bench]
fn d17_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day17.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d17_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day17.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d17_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day17.txt")));
}

//R
