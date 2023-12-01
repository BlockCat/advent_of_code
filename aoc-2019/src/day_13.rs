use crate::test::Bencher;
use std::sync::mpsc;
use utils::intcode;

#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day13.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(mut program: intcode::IntProgram) -> usize {
    
    program.nth(1);
    program.step_by(3).filter(|x| *x == 2).count()
}
fn exercise_2(mut program: intcode::IntProgram) -> usize {
    program.memory[0] = 2;
    
    let mut score = 0;
    let mut x_bat = 0;
    let mut x_ball; 
    loop {
        match (program.next(), program.next(), program.next()) {
            (Some(x), _, Some(3)) => x_bat = x,
            (Some(x), _, Some(4)) => {
                x_ball = x;
                program.input((x_ball - x_bat).signum());
            },
            (Some(-1), Some(0), Some(c)) => score = c,
            (None, _, _) => return score as usize,
            _ => {}
        }
    }
}

#[test]
fn d13_test() {
    let input = intcode::IntProgram::parse(include_str!("input/day13.txt"));
    assert_eq!(exercise_1(input.clone()), 193);
    assert_eq!(exercise_2(input), 10547);
}

#[bench]
fn d13_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day13.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d13_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day13.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d13_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day13.txt")));
}
