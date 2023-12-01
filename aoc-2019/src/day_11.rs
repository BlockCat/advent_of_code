use crate::test::Bencher;
use std::sync::mpsc;
use utils::intcode;

#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day11.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: intcode::IntProgram) -> usize {
    let mut grid = hashbrown::HashMap::new();
    run_program(input, &mut grid)
}
fn exercise_2(input: intcode::IntProgram) -> usize {
    let mut grid = hashbrown::HashMap::new();
    grid.insert(utils::Vector2(0, 0), 1);
    run_program(input, &mut grid);

    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| match grid.get(&utils::Vector2(x, y)) {
                Some(1) => 'X',
                _ => ' ',
            })
            .collect::<String>();
        println!("{}", line);
    }

    0
}

fn run_program(
    mut program: intcode::IntProgram,
    grid: &mut hashbrown::HashMap<utils::Vector2, i64>,
) -> usize {
    
    let mut drawn = hashbrown::HashSet::new();
    let mut current_pos = utils::Vector2(0, 0);
    let mut current_dir = utils::Direction::North;
    

    loop {
        let current_grid = grid.entry(current_pos).or_insert(0i64);
        program.input(*current_grid);
        let (next_colour, next_rotation) = match (program.next(), program.next()) {
            (Some(nc), Some(nr)) => (nc, nr),
            _ => break,
        };

        *current_grid = next_colour;
        drawn.insert(current_pos);
        current_dir = match next_rotation {
            0 => current_dir.left(),
            1 => current_dir.right(),
            r => panic!("Invalid rotation: {}", r),
        };
        current_pos += current_dir;
    }

    drawn.len()
}

#[test]
fn d11_test() {
    let input = intcode::IntProgram::parse(include_str!("input/day11.txt"));
    assert_eq!(exercise_1(input), 2056);
}

#[bench]
fn d11_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day11.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d11_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day11.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d11_bench_parse(b: &mut Bencher) {
    b.iter(||intcode::IntProgram::parse(include_str!("input/day11.txt")));
}