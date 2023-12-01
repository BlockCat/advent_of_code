use crate::test::Bencher;
use hashbrown::HashMap;

use utils::intcode;
use utils::{Direction, Vector2};

#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day19.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input)); // 3840993, 3810985 too high
}

fn exercise_1(program: intcode::IntProgram) -> usize {
    let mut counter = 0;

    for y in 0..50i64 {
        for x in 0..50 {
            let mut program = program.clone();
            program.input(x);
            program.input(y);
            if let Some(1) = program.next() {
                counter += 1;
            }
        }
    }
    counter
}

/*
fn print_grid(grid: &HashMap<Vector2, bool>) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    println!("---");
    for y in min_y..=max_y {
        let line = (min_x..=max_x)
            .map(|x| match grid.get(&utils::Vector2(x, y)) {
                Some(true) => '#',
                _ => '.',
            })
            .collect::<String>();
        println!("{}", line);
    }
    println!("---");
}*/

fn exercise_2(mut program: intcode::IntProgram) -> i64 {
    let mut xl = 0;
    let mut xr = 0;
    let dist = 10000;
    for x in 0..dist {
        let mut program = program.clone();
        program.input(x);
        program.input(dist);

        if let Some(1) = program.next() {
            xl = x;
            break;
        }
    }

    for x in xl..dist {
        let mut program = program.clone();
        program.input(x);
        program.input(dist);

        if let Some(0) = program.next() {
            xr = x - 1;
            break;
        }
    }

    let a = xl as f64 / dist as f64;
    let b = xr as f64 / dist as f64;
    //    let y2 = (xl*100 + 100*dist) as f64 / (xr - xl) as f64;
    let y = (b * 100f64 + 100f64) / (b - a);

    //  let x2 = ((xl*100 + 100*dist) * xl) as f64 / (dist * (xr - xl)) as f64;

    let x = a * y;
    let y = y - 100f64;

    let y2 = y.floor() as i64 - 7;
    let x2 = x.ceil() as i64 - 3;

    x2 * 10000 + y2
}
#[test]
fn d19_test() {
    let input = intcode::IntProgram::parse(include_str!("input/day19.txt"));
    assert_eq!(exercise_1(input.clone()), 166);
    assert_eq!(exercise_2(input), 3790981);
}

#[bench]
fn d19_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day19.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d19_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day19.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d19_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day19.txt")));
}

//R
