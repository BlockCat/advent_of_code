use aoc_2022::{
    direction::Direction,
    grid::{DynamicGrid, Grid},
    vector::Vector2,
};

type InputType = Vec<Vec<Vector2>>;
type DayGrid = DynamicGrid<()>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_14.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Vec<Vector2> {
    line.split(" -> ")
        .map(|x| {
            let mut r = x.split(",").map(|x| x.parse::<isize>().unwrap());

            Vector2::new([r.next().unwrap(), r.next().unwrap()])
        })
        .collect()
}

fn grid(input: InputType) -> DayGrid {
    input
        .iter()
        .flat_map(|x| x.windows(2))
        .fold(DayGrid::default(), |mut acc, x| {
            if x[0][0] == x[1][0] {
                add_ver_line(&mut acc, x[0], x[1]);
            } else {
                add_hor_line(&mut acc, x[0], x[1]);
            }
            acc
        })
}

fn add_hor_line(grid: &mut DayGrid, start: Vector2, end: Vector2) {
    let s = start[0].min(end[0]);
    let e = start[0].max(end[0]);
    for x in s..=e {
        grid.set(x as isize, start[1] as isize, ());
    }
}
fn add_ver_line(grid: &mut DayGrid, start: Vector2, end: Vector2) {
    let s = start[1].min(end[1]);
    let e = start[1].max(end[1]);
    for y in s..=e {
        grid.set(start[0] as isize, y as isize, ());
    }
}

fn exercise_1(input: InputType) -> usize {
    let y_bound = input
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x[1])
        .max()
        .unwrap();
    let mut grid = grid(input);

    (1usize..)
        .take_while(|_| fall_down(&mut grid, y_bound))
        .last()
        .unwrap()
}

fn exercise_2(input: InputType) -> usize {
    let y_bound = input
        .iter()
        .flat_map(|x| x.iter())
        .map(|x| x[1])
        .max()
        .unwrap();
    let mut grid = grid(input);

    (1usize..)
        .take_while(|_| fall_down2(&mut grid, y_bound))
        .last()
        .unwrap()
}

fn fall_down(grid: &mut DayGrid, y_bound: isize) -> bool {
    let mut sand = Vector2::new([500, 0]);

    loop {
        if sand[1] >= y_bound {
            return false;
        }
        let next_pos = sand + Direction::South;
        let positions = [
            next_pos,
            next_pos + Direction::West,
            next_pos + Direction::East,
        ];

        if let Some(pos) = positions.into_iter().find(|x| grid.get_vec(x).is_none()) {
            sand = pos;
        } else {
            grid.set_vec(&sand, ());
            return true;
        }
    }
}

fn fall_down2(grid: &mut DayGrid, y_bound: isize) -> bool {
    let mut sand = Vector2::new([500, 0]);
    let start = Vector2::new([500, 0]);
    loop {
        if grid.get_vec(&start).is_some() {
            return false;
        }

        let next_pos = sand + Direction::South;
        let positions = [
            next_pos,
            next_pos + Direction::West,
            next_pos + Direction::East,
        ];
        if let Some(pos) = positions
            .into_iter()
            .find(|x| !has_place(grid, &x, y_bound))
        {
            sand = pos;
        } else {
            grid.set_vec(&sand, ());
            return true;
        }
    }
}

fn has_place(grid: &DayGrid, pos: &Vector2, y_bound: isize) -> bool {
    pos[1] >= y_bound + 2 || grid.get_vec(pos).is_some()
}
