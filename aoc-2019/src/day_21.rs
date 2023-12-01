use crate::test::Bencher;
use utils::intcode;

#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day21.txt"));
    println!("ex1: {:?}", exercise_1(input.clone()));
    println!("ex2: {:?}", exercise_2(input));
}

fn exercise_1(mut program: intcode::IntProgram) -> Option<i64> {
    provide_array(&mut program, "NOT A J"); // !A => J
    provide_array(&mut program, "NOT C T");
    provide_array(&mut program, "AND D T");
    provide_array(&mut program, "OR T J"); // !C & D => J
    provide_array(&mut program, "WALK");
    let output = program.collect::<Vec<_>>();
    let last_value = output.last().unwrap();

    if *last_value <= 128 {
        print_error(output);
        None
    } else {
        Some(*last_value)
    }
}
fn exercise_2(mut program: intcode::IntProgram) -> Option<i64> {
    provide_array(&mut program, "NOT A J"); // !A => J
    provide_array(&mut program, "OR B T");
    provide_array(&mut program, "AND C T");
    provide_array(&mut program, "NOT T T");
    provide_array(&mut program, "AND D T");
    provide_array(&mut program, "AND H T");
    provide_array(&mut program, "OR T J"); // !(B & C) & D & H => J
    provide_array(&mut program, "RUN\n");

    let output = program.collect::<Vec<_>>();
    let last_value = output.last().unwrap();

    if *last_value <= 128 {
       print_error(output);
       None
    } else {
        Some(*last_value)
    }
}

fn print_error(output: Vec<i64>) {
    println!(
        "{}",
        output.iter().map(|x| *x as u8 as char).collect::<String>()
    );
}

fn provide_array(program: &mut intcode::IntProgram, array: &str) {
    for c in array.chars() {
        program.input(c as i64);
    }
    program.input('\n' as i64);
}

#[test]
fn d21_test() {
    let input = intcode::IntProgram::parse(include_str!("input/day21.txt"));
    assert_eq!(exercise_1(input.clone()), Some(19355227));
    assert_eq!(exercise_2(input.clone()), Some(1143802926));
}

#[bench]
fn d21_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day21.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d21_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day21.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d21_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day21.txt")));
}
