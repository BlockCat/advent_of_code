use aoc_2021::{
    grid::{DynamicGrid, Grid},
    vector::Vector2,
};

pub fn main() {
    let input = parse_input(include_str!("../input/day05.txt"));

    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Vec<HydroLine> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let a = split
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            let b = split
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();

            HydroLine {
                source: Vector2::new([a[0], a[1]]),
                dest: Vector2::new([b[0], b[1]]),
            }
        })
        .collect()
}

fn exercise_1(lines: &[HydroLine]) -> usize {
    let mut grid = DynamicGrid::default();
    lines
        .iter()
        .filter(|x| x.source[0] == x.dest[0] || x.source[1] == x.dest[1])
        .for_each(|x| fill_line(x, &mut grid));

    grid.map.values().filter(|&&v| v > 1).count()
}

fn exercise_2(lines: &[HydroLine]) -> usize {
    let mut grid = DynamicGrid::default();

    for line in lines {
        fill_line(line, &mut grid);
    }

    grid.map.values().filter(|&&v| v > 1).count()
}

fn fill_line(line: &HydroLine, grid: &mut dyn Grid<usize>) {    
    let start = line.source;
    let end = line.dest;
    let dif = (start - end).abs();
    let size = dif[0].max(dif[1]);
    
    let dx = match start[0].cmp(&end[0]) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    };
    let dy = match start[1].cmp(&end[1]) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    };

    let mut pos: Vector2 = start;

    for _ in 0..size {
        *grid.get_insert_vec_mut(&pos, 0) += 1;
        pos += Vector2::new([dx, dy]);
    }
    *grid.get_insert_vec_mut(&pos, 0) += 1;
}

#[derive(Debug, Clone, Copy)]
struct HydroLine {
    source: Vector2,
    dest: Vector2,
}
