use crate::test::Bencher;

use utils::intcode;

type Input = i64;
type Output = i64;

//#[test]
pub fn run() {
    let input = intcode::IntProgram::parse(include_str!("input/day23.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: intcode::IntProgram) -> Output {
    use intcode::IntProgramResult;
    let mut programs = Vec::new();
    for i in 0..50 {
        let mut program = input.clone();
        program.input(i);
        programs.push(program);
    }

    loop {
        for i in 0..50 {
            let next_intcode = programs[i].next();
            if next_intcode.is_none() {
                continue;
            }
            let next_intcode = next_intcode.unwrap();
            if let IntProgramResult::Value(address) = &next_intcode {
                match (programs[i].next().unwrap(), programs[i].next().unwrap()) {
                    (IntProgramResult::Value(x), IntProgramResult::Value(y)) => {
                        if *address == 255 {
                            return y;
                        } else {
                            programs[*address as usize].input(x);
                            programs[*address as usize].input(y);
                        }
                    }
                    _ => panic!(),
                }
            } else if let intcode::IntProgramResult::Stalled = &next_intcode {
                programs[i].input(-1);
            }
        }
    }

    0
}
fn exercise_2(input: intcode::IntProgram) -> Output {
    use intcode::IntProgramResult;
    let mut programs = Vec::new();

    for i in 0..50 {
        let mut program = input.clone();
        program.input(i);
        programs.push(program);
    }

    let mut nat: Option<(i64, i64)> = None;
    let mut prev_y_send: Option<i64> = None;
    let mut counter = 1;
    loop {
        let mut idle = true;
        for i in 0..50 {
            let next_intcode = programs[i].next().unwrap();

            if let IntProgramResult::Value(address) = &next_intcode {
                idle = false;                
                match (programs[i].next().unwrap(), programs[i].next().unwrap()) {
                    (IntProgramResult::Value(x), IntProgramResult::Value(y)) => {
                        if *address == 255 {
                            nat = Some((x, y));
                        } else {
                            programs[*address as usize].input(x);
                            programs[*address as usize].input(y);
                        }
                    }
                    _ => panic!(),
                }
            } else {
                programs[i].input(-1);
            }
        }

        if idle && programs.iter().all(|x| x.input_stack[0] == -1) {            
            if counter == 0 {                
                let (x, y) = nat.unwrap();
                programs[0].input(x);
                programs[0].input(y);
                counter = 1;

                if let Some(prev_y) = prev_y_send {
                    if prev_y == y {
                        return y;
                    }
                }
                prev_y_send = Some(y);
            } else {
                counter -= 1;
            }
        } else {
            counter = 1;
        }
    }

    0
}
#[test]
fn d22_test() {}

#[bench]
fn d22_bench_ex1(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day23.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d22_bench_ex2(b: &mut Bencher) {
    let input = intcode::IntProgram::parse(include_str!("input/day23.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d22_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::IntProgram::parse(include_str!("input/day23.txt")));
}
