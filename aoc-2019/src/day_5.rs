use crate::test::Bencher;

type Input = i32;
type Output = i32;

enum ParamMode {
    Position,
    Immediate,
}

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day5.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: Vec<Input>) -> Output {
    run_program(input, std::iter::once(1))
}

fn exercise_2(input: Vec<Input>) -> Output {
    run_program(input, std::iter::once(5))
}

fn read_input(input: &str) -> Vec<Input> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn to_mode(mode: i32) -> ParamMode {
    match mode {
        0 => ParamMode::Position,
        _ => ParamMode::Immediate
    }
}

fn get_value(mode: ParamMode, mem: &Vec<i32>, i: usize) -> i32 {
    match mode {
        ParamMode::Immediate => mem[i],
        ParamMode::Position => mem[mem[i] as usize],
    }
}

fn run_program(mut slice: Vec<Input>, mut inputs: impl Iterator<Item = i32>) -> i32 {
    let mut i = 0;
    let mut latest_output = 0;
    while i < slice.len() {
        let instruction = slice[i];
        let opcode = instruction % 100;
        let mode_1 = to_mode((instruction / 100) % 10);
        let mode_2 = to_mode((instruction / 1_000) % 10);
        let mode_3 = to_mode((instruction / 10_000) % 10);
        //println!("{}", instruction);
        match opcode {
            1 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                slice[index as usize] = a + b;
                i += 4;
            }
            2 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                slice[index as usize] = a * b;
                i += 4;
            }
            3 => {
                let index = get_value(ParamMode::Immediate, &slice, i + 1);
                slice[index as usize] = inputs.next().unwrap();
                i += 2;
            }
            4 => {
                let index = get_value(mode_1, &slice, i + 1);                                                              
                latest_output = index;
                i += 2;
            }
            5 => {
                let tester = get_value(mode_1, &slice, i + 1);
                let jumper = get_value(mode_2, &slice, i + 2);
                if tester != 0 {
                    i = jumper as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let tester = get_value(mode_1, &slice, i + 1);
                let jumper = get_value(mode_2, &slice, i + 2);
                if tester == 0 {
                    i = jumper as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                if a < b {
                    slice[index as usize] = 1;
                } else {
                    slice[index as usize] = 0;
                }
                i += 4;
            }
            8 => {
                let a = get_value(mode_1, &slice, i + 1);
                let b = get_value(mode_2, &slice, i + 2);
                let index = get_value(ParamMode::Immediate, &slice, i + 3);
                if a == b {
                    slice[index as usize] = 1;
                } else {
                    slice[index as usize] = 0;
                }
                i += 4;
            }
            99 => break,
            _ => panic!("Unexpected opcode: {}", opcode),
        }
    }

    latest_output
}

#[test]
fn d5_test() {
    assert_eq!(
        0,
        run_program(read_input("3,9,8,9,10,9,4,9,99,-1,8"), std::iter::once(9))
    );
    assert_eq!(
        1,
        run_program(read_input("3,9,8,9,10,9,4,9,99,-1,8"), std::iter::once(8))
    );
    assert_eq!(
        0,
        run_program(read_input("3,9,7,9,10,9,4,9,99,-1,8"), std::iter::once(9))
    );
    assert_eq!(
        1,
        run_program(read_input("3,9,7,9,10,9,4,9,99,-1,8"), std::iter::once(7))
    );
    assert_eq!(999, run_program(read_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), std::iter::once(7)));
    assert_eq!(1000, run_program(read_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), std::iter::once(8)));
    assert_eq!(1001, run_program(read_input("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"), std::iter::once(9)));

    let input = read_input(include_str!("input/day5.txt"));
    assert_eq!(9219874, exercise_1(input.clone()));
    assert_eq!(5893654, exercise_2(input));
}

#[bench]
fn d5_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day5.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d5_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day5.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d5_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day5.txt")));
}
