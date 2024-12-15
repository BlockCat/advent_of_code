use aoc_2024::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    stopwatch,
    vector::Vector2,
};
use std::collections::HashMap;

type Input = (StaticGrid<Location>, Vec<Direction>);

pub fn main() {
    let numbers = input(include_str!("../input/day_15.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers.0, &numbers.1));
        println!("Exercise 2: {}", exercise_2(&numbers.0, &numbers.1));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    let (maze, instructions) = input.split_once("\n\n").unwrap();
    (parse_maze(maze), parse_instructions(instructions))
}

fn parse_maze(maze: &str) -> StaticGrid<Location> {
    StaticGrid::from_vec(
        maze.lines()
            .map(|x| {
                x.chars()
                    .map(|c| match c {
                        '.' => Location::None,
                        '#' => Location::Wall,
                        '@' => Location::Robot,
                        'O' => Location::Box,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    )
}

fn parse_instructions(instructions: &str) -> Vec<Direction> {
    instructions
        .lines()
        .flat_map(|x| x.chars())
        .map(|c| match c {
            '<' => Direction::West,
            '>' => Direction::East,
            '^' => Direction::North,
            'v' => Direction::South,
            _ => unreachable!(),
        })
        .collect()
}

fn exercise_1(maze: &StaticGrid<Location>, instructions: &Vec<Direction>) -> usize {
    let mut maze = maze.clone();
    let mut robot = maze.iter().find(|(_, x)| x == &&Location::Robot).unwrap().0;
    maze.set_vec(&robot, Location::None);

    for &instruction in instructions {
        (maze, robot) = execute_instruction(robot, maze, instruction);

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        maze_printer(&maze, robot);
    }

    check_sum(&maze)
}

fn exercise_2(maze: &StaticGrid<Location>, instructions: &Vec<Direction>) -> usize {
    let maze = StaticGrid::from_slice(
        maze.width * 2,
        maze.height,
        maze.grid
            .iter()
            .flat_map(|x| match x {
                Location::None => [Location::None, Location::None],
                Location::Box => [Location::BoxLeft, Location::BoxRight],
                Location::Wall => [Location::Wall, Location::Wall],
                Location::Robot => [Location::Robot, Location::None],
                _ => unreachable!(),
            })
            .collect(),
    );

    exercise_1(&maze, instructions)
}

fn check_sum(maze: &StaticGrid<Location>) -> usize {
    maze.iter()
        .filter(|x| x.1 == &Location::Box || x.1 == &Location::BoxLeft)
        .map(|(p, _)| (p[0] + 100 * p[1]) as usize)
        .sum()
}

fn execute_instruction(
    robot: Vector2,
    mut maze: StaticGrid<Location>,
    direction: Direction,
) -> (StaticGrid<Location>, Vector2) {
    let direction_check = Vector2::from(direction);

    let next_location = maze.get_vec(&(robot + direction_check)).unwrap();

    match *next_location {
        Location::Wall => (maze, robot),
        Location::None => (maze, robot + direction_check),
        Location::Box => {
            for i in 2isize.. {
                let pos = robot + (direction_check * i);
                match *maze.get_vec(&pos).unwrap() {
                    Location::None => {
                        maze.set_vec(&(robot + direction_check), Location::None);
                        maze.set_vec(&pos, Location::Box);

                        return (maze, robot + direction_check);
                    }
                    Location::Box => continue,
                    Location::Wall => return (maze, robot),
                    _ => unreachable!(),
                }
            }
            unreachable!()
        }
        Location::BoxLeft if direction == Direction::East => handle_big_hor_slide(
            maze,
            robot,
            direction_check,
            Location::BoxLeft,
            Location::BoxRight,
        ),
        Location::BoxRight if direction == Direction::West => handle_big_hor_slide(
            maze,
            robot,
            direction_check,
            Location::BoxRight,
            Location::BoxLeft,
        ),
        Location::BoxLeft if direction == Direction::North || direction == Direction::South => {
            handle_big_ver_slide(maze, robot, direction)
        }

        Location::BoxRight if direction == Direction::North || direction == Direction::South => {
            handle_big_ver_slide(maze, robot, direction)
        }
        _ => unreachable!("error with: {:?}, {:?}", next_location, direction),
    }
}

fn handle_big_hor_slide(
    mut maze: StaticGrid<Location>,
    robot: Vector2,
    direction_check: Vector2,
    side: Location,
    other_side: Location,
) -> (StaticGrid<Location>, Vector2) {
    for i in (3isize..).step_by(2) {
        let pos = robot + (direction_check * i);
        match *maze.get_vec(&pos).unwrap() {
            Location::None => {
                maze.set_vec(&(robot + direction_check), Location::None);

                for j in (2..i).step_by(2) {
                    maze.set_vec(&(robot + direction_check * j), side);
                    maze.set_vec(&(robot + direction_check * j + direction_check), other_side);
                }

                return (maze, robot + direction_check);
            }
            Location::BoxRight => continue,
            Location::BoxLeft => continue,
            Location::Wall => return (maze, robot),
            c => unreachable!("{:?}", c),
        }
    }
    unreachable!()
}

fn handle_big_ver_slide(
    mut maze: StaticGrid<Location>,
    robot: Vector2,
    direction: Direction,
) -> (StaticGrid<Location>, Vector2) {
    let mut queue = vec![robot + direction];

    let mut handled = HashMap::new();

    while let Some(po) = queue.pop() {
        let location = *maze.get_vec(&po).unwrap();
        handled.insert(po, location);
        match location {
            Location::None => {}
            Location::Box => unreachable!(),
            Location::BoxLeft => {
                handled.insert(po + Direction::East, Location::BoxRight);
                queue.push(po + direction);
                queue.push(po + direction + Direction::East);
            }
            Location::BoxRight => {
                handled.insert(po + Direction::West, Location::BoxLeft);
                queue.push(po + direction);
                queue.push(po + direction + Direction::West);
            }
            Location::Wall => {
                return (maze, robot);
            }
            Location::Robot => unreachable!(),
        }
    }

    // It ok
    handled
        .iter()
        .filter(|x| x.1 != &Location::None)
        .for_each(|x| {
            maze.set_vec(&x.0, Location::None);
        });
    handled
        .iter()
        .filter(|x| x.1 != &Location::None)
        .for_each(|x| {
            maze.set_vec(&(*x.0 + direction), *x.1);
        });
    (maze, robot + direction)
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    #[default]
    None,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
    Robot,
}

fn maze_printer(maze: &StaticGrid<Location>, robot: Vector2) {
    maze.pretty_print(|r, p| {
        if p == robot {
            '@'
        } else {
            match *r {
                Location::None => '.',
                Location::Box => 'O',
                Location::BoxLeft => '[',
                Location::BoxRight => ']',
                Location::Wall => '#',
                Location::Robot => '@',
            }
        }
    });
}
