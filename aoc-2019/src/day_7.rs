use crate::test::Bencher;
use permutohedron::Heap;
use std::sync::mpsc;
use std::sync::mpsc::Sender;

type Input = i32;
type Output = i32;

#[test]
pub fn run() {
    let input = intcode::parse_program(include_str!("input/day7.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: Vec<Input>) -> Output {
    use intcode::NoOutput;

    let mut data = [0, 1, 2, 3, 4];
    let mut heap = Heap::new(&mut data);

    let mut max_out = std::i32::MIN;

    while let Some(perm) = heap.next_permutation() {
        let a = intcode::run_program_channel(input.clone(), vec![perm[0], 0].into_iter(), NoOutput);
        let b = intcode::run_program_channel(input.clone(), vec![perm[1], a].into_iter(), NoOutput);
        let c = intcode::run_program_channel(input.clone(), vec![perm[2], b].into_iter(), NoOutput);
        let d = intcode::run_program_channel(input.clone(), vec![perm[3], c].into_iter(), NoOutput);
        let e = intcode::run_program_channel(input.clone(), vec![perm[4], d].into_iter(), NoOutput);

        if e > max_out {
            max_out = e;
        }
    }
    max_out
}

fn exercise_2(input: Vec<Input>) -> Output {
    use intcode::run_program_channel;
    let mut data = [5, 6, 7, 8, 9];
    let mut heap = Heap::new(&mut data);

    let mut max_out = std::i32::MIN;

    while let Some(perm) = heap.next_permutation() {
        let a_mem = input.clone();
        let b_mem = input.clone();
        let c_mem = input.clone();
        let d_mem = input.clone();
        let e_mem = input.clone();

        let (ase, are) = mpsc::channel();
        let (bse, bre) = mpsc::channel();
        let (cse, cre) = mpsc::channel();
        let (dse, dre) = mpsc::channel();
        let (ese, ere) = mpsc::channel();

        ase.send(perm[0]).unwrap();
        ase.send(0).unwrap();
        bse.send(perm[1]).unwrap();
        cse.send(perm[2]).unwrap();
        dse.send(perm[3]).unwrap();
        ese.send(perm[4]).unwrap();

        std::thread::spawn(move || run_program_channel(a_mem, are.iter(), bse));
        std::thread::spawn(move || run_program_channel(b_mem, bre.iter(), cse));
        std::thread::spawn(move || run_program_channel(c_mem, cre.iter(), dse));
        std::thread::spawn(move || run_program_channel(d_mem, dre.iter(), ese));
        let e = std::thread::spawn(move || run_program_channel(e_mem, ere.iter(), ase));
        let feed_loop = e.join().unwrap();
        if feed_loop > max_out {
            max_out = feed_loop;
        }
    }
    max_out
}

mod intcode {

    enum ParamMode {
        Position,
        Immediate,
    }

    pub struct NoOutput;
    pub struct PrintOutput;
    pub trait Out<T> {
        fn output(&self, n: T);
    }

    impl<T> Out<T> for NoOutput {
        fn output(&self, _: T) {}
    }

    impl<T> Out<T> for PrintOutput
    where
        T: std::fmt::Display,
    {
        fn output(&self, n: T) {
            println!("{}", n);
        }
    }

    impl<T> Out<T> for std::sync::mpsc::Sender<T> {
        fn output(&self, n: T) {
            self.send(n).unwrap_or(())
        }
    }

    fn to_mode(mode: i32) -> ParamMode {
        match mode {
            0 => ParamMode::Position,
            _ => ParamMode::Immediate,
        }
    }

    fn get_value(mode: ParamMode, mem: &Vec<i32>, i: usize) -> i32 {
        match mode {
            ParamMode::Immediate => mem[i],
            ParamMode::Position => mem[mem[i] as usize],
        }
    }

    pub fn parse_program(input: &str) -> Vec<i32> {
        input
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect()
    }

    pub fn run_program_channel(
        mut slice: Vec<i32>,
        mut receiver: impl Iterator<Item = i32>,
        sender: impl Out<i32>,
    ) -> i32 {
        let mut i = 0;
        let mut latest_output = 0;
        while i < slice.len() {
            let instruction = slice[i];
            let opcode = instruction % 100;
            let mode_1 = to_mode((instruction / 100) % 10);
            let mode_2 = to_mode((instruction / 1_000) % 10);         
            //println!("{} - {:?}", opcode, slice);
            match opcode {
                1 => { // add
                    let a = get_value(mode_1, &slice, i + 1);
                    let b = get_value(mode_2, &slice, i + 2);
                    let index = get_value(ParamMode::Immediate, &slice, i + 3);
                    slice[index as usize] = a + b;
                    i += 4;
                }
                2 => { // mul
                    let a = get_value(mode_1, &slice, i + 1);
                    let b = get_value(mode_2, &slice, i + 2);
                    let index = get_value(ParamMode::Immediate, &slice, i + 3);
                    slice[index as usize] = a * b;
                    i += 4;
                }
                3 => { // input
                    let index = get_value(ParamMode::Immediate, &slice, i + 1);
                    slice[index as usize] = receiver.next().unwrap();
                    i += 2;
                }
                4 => { // output
                    let index = get_value(mode_1, &slice, i + 1);
                    latest_output = index;
                    sender.output(index);
                    i += 2;
                }
                5 => { // jump not 0
                    let tester = get_value(mode_1, &slice, i + 1);
                    let jumper = get_value(mode_2, &slice, i + 2);
                    if tester != 0 {
                        i = jumper as usize;
                    } else {
                        i += 3;
                    }
                }
                6 => { // jump if 0
                    let tester = get_value(mode_1, &slice, i + 1);
                    let jumper = get_value(mode_2, &slice, i + 2);                    
                    if tester == 0 {
                        i = jumper as usize;
                    } else {
                        i += 3;
                    }
                }
                7 => { // a < b
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
                8 => { // a == b
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
}

#[test]
fn d7_test() {
    use intcode::parse_program;
    assert_eq!(
        exercise_1(parse_program(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
        )),
        43210
    );

    assert_eq!(
        exercise_2(parse_program(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
        )),
        139629729
    );
    assert_eq!(
        exercise_2(parse_program(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
        )),
        18216
    );
}

#[test]
fn d7_counter() {
    use intcode::parse_program;

    let input = "3,14, 1101,0,1,0, 4,0, 1001,0,1,0, 1007,0,-9999,1, 1005,1,6, 4,0,99";
    let input = "3,14,1101,0,1,0,4,0,1001,0,1,0,1007,0,-9999,1,1005,1,6,4,0,99";
    
    let input = parse_program(&input.replace(" ", "")[..]);
    intcode::run_program_channel(input, std::iter::once(50), intcode::PrintOutput);
}
#[test]
fn d7_fib() {
    use intcode::parse_program;

    let input = "3,0, 3,1, 4,0, 4,1, 1,0,1,2, 1001,1,0,0, 1001,2,0,1, 4,2, 1106,0,8";
    let input = "3,0,3,1,4,0,4,1,1,0,1,2,1001,1,0,0,1001,2,0,1,4,2,1106,0,8";
    
    
    let input = parse_program(&input.replace(" ", "")[..]);
    intcode::run_program_channel(input, vec!(1,1).into_iter(), intcode::PrintOutput);
}

#[bench]
fn d7_bench_ex1(b: &mut Bencher) {
    let input = intcode::parse_program(include_str!("input/day7.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d7_bench_ex2(b: &mut Bencher) {
    let input = intcode::parse_program(include_str!("input/day7.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d7_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::parse_program(include_str!("input/day7.txt")));
}
