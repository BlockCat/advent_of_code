use crate::test::Bencher;

#[test]
pub fn run() {
    let input = intcode::parse_program(include_str!("input/day9.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: intcode::Memory) -> i64 {
    intcode::run_program_channel(input, std::iter::once(1), &mut intcode::NoOutput)
}

fn exercise_2(input: intcode::Memory) -> i64 {
    intcode::run_program_channel(input, std::iter::once(2), &mut intcode::NoOutput)
}

#[test]
fn d7_test() {
    use intcode::*;
    let input = parse_program("104,1125899906842624,99");
    assert_eq!(
        run_program_channel(input, std::iter::empty(), &mut intcode::NoOutput),
        1125899906842624
    );

    let input = parse_program("1102,34915192,34915192,7,4,7,99,0");
    assert_eq!(
        run_program_channel(input, std::iter::empty(), &mut intcode::NoOutput),
        34915192 * 34915192
    );

    let input = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut buf = Vec::new();
    run_program_channel(input, std::iter::empty(), &mut buf); 
    assert_eq!(buf, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

    let input = parse_program(include_str!("input/day9.txt"));
    assert_eq!(exercise_1(input.clone()), 3512778005);
    assert_eq!(exercise_2(input.clone()), 35920);
}

#[bench]
fn d9_bench_ex1(b: &mut Bencher) {
    let input = intcode::parse_program(include_str!("input/day9.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d9_bench_ex2(b: &mut Bencher) {
    let input = intcode::parse_program(include_str!("input/day9.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d9_bench_parse(b: &mut Bencher) {
    b.iter(|| intcode::parse_program(include_str!("input/day9.txt")));
}

mod intcode {

    #[derive(PartialEq, Eq, Debug)]
    enum ParamMode {
        Position,
        Immediate,
        Relative,
    }

    pub struct NoOutput;
    pub struct PrintOutput;
    pub trait Out<T> {
        fn output(&mut self, n: T);
    }

    impl<T> Out<T> for NoOutput {
        fn output(&mut self, _: T) {}
    }

    impl<T> Out<T> for PrintOutput
    where
        T: std::fmt::Display,
    {
        fn output(&mut self, n: T) {
            println!(">{}", n);
        }
    }

    impl<T> Out<T> for std::sync::mpsc::Sender<T> {
        fn output(&mut self, n: T) {
            self.send(n).unwrap_or(())
        }
    }

    impl<T> Out<T> for Vec<T> {
        fn output(&mut self, n: T) {
            self.push(n);
        }
    }

    #[derive(Clone)]
    pub struct Memory {
        input: Vec<i64>,
        overmem: hashbrown::HashMap<usize, i64>,
    }

    impl std::ops::Index<usize> for Memory {
        type Output = i64;
        fn index(&self, index: usize) -> &Self::Output {
            if index < self.input.len() {
                &self.input[index]
            } else {
                self.overmem.get(&index).unwrap_or(&0)
            }
        }
    }

    impl std::ops::IndexMut<usize> for Memory {
        fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
            if index < self.input.len() {
                &mut self.input[index]
            } else {
                self.overmem.entry(index).or_insert(0)
            }
        }
    }

    fn to_mode(mode: i64) -> ParamMode {
        match mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => unreachable!(),
        }
    }

    fn get_value(mode: ParamMode, mem: &Memory, relative_position: i64, i: usize) -> i64 {
        match mode {
            ParamMode::Immediate => mem[i],
            ParamMode::Position => mem[mem[i] as usize],
            ParamMode::Relative => mem[(mem[i] + relative_position) as usize],
        }
    }

    fn get_index(mode: ParamMode, mem: &Memory, relative_position: i64, i: usize) -> i64 {
        match mode {
            ParamMode::Position => mem[i],
            ParamMode::Immediate => mem[i],
            ParamMode::Relative => mem[i] + relative_position,
            _ => panic!(),
        }
    }

    pub fn parse_program(input: &str) -> Memory {
        Memory {
            overmem: hashbrown::HashMap::new(),
            input: input
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
        }
    }

    pub fn run_program_channel(
        mut slice: Memory,
        mut receiver: impl Iterator<Item = i64>,
        sender: &mut impl Out<i64>,
    ) -> i64 {
        let mut i = 0;
        let mut latest_output = -1;
        let mut relative_position = 0i64;
        loop {
            let instruction = slice[i];
            let opcode = instruction % 100;
            let mode_1 = to_mode((instruction / 100) % 10);
            let mode_2 = to_mode((instruction / 1_000) % 10);
            let mode_3 = to_mode((instruction / 10_000) % 10);

            //println!("{} - {:?}", opcode, slice);
            match opcode {
                1 => {
                    // add
                    let a = get_value(mode_1, &slice, relative_position, i + 1);
                    let b = get_value(mode_2, &slice, relative_position, i + 2);
                    let index = get_index(mode_3, &slice, relative_position, i + 3);
                    slice[index as usize] = a + b;
                    i += 4;
                }
                2 => {
                    // mul
                    let a = get_value(mode_1, &slice, relative_position, i + 1);
                    let b = get_value(mode_2, &slice, relative_position, i + 2);
                    let index = get_index(mode_3, &slice, relative_position, i + 3);
                    slice[index as usize] = a * b;
                    i += 4;
                }
                3 => {
                    // input
                    let index = get_index(mode_1, &slice, relative_position, i + 1);
                    slice[index as usize] = receiver.next().unwrap();
                    i += 2;
                }
                4 => {
                    // output
                    let value = get_value(mode_1, &slice, relative_position, i + 1);
                    latest_output = value;
                    sender.output(value);
                    i += 2;
                }
                5 => {
                    // jump not 0
                    let tester = get_value(mode_1, &slice, relative_position, i + 1);
                    let jumper = get_value(mode_2, &slice, relative_position, i + 2);
                    if tester != 0 {
                        i = jumper as usize;
                    } else {
                        i += 3;
                    }
                }
                6 => {
                    // jump if 0
                    let tester = get_value(mode_1, &slice, relative_position, i + 1);
                    let jumper = get_value(mode_2, &slice, relative_position, i + 2);
                    if tester == 0 {
                        i = jumper as usize;
                    } else {
                        i += 3;
                    }
                }
                7 => {
                    // a < b
                    let a = get_value(mode_1, &slice, relative_position, i + 1);
                    let b = get_value(mode_2, &slice, relative_position, i + 2);
                    let index = get_index(mode_3, &slice, relative_position, i + 3);
                    if a < b {
                        slice[index as usize] = 1;
                    } else {
                        slice[index as usize] = 0;
                    }
                    i += 4;
                }
                8 => {
                    // a == b
                    let a = get_value(mode_1, &slice, relative_position, i + 1);
                    let b = get_value(mode_2, &slice, relative_position, i + 2);
                    let index = get_index(mode_3, &slice, relative_position, i + 3);
                    if a == b {
                        slice[index as usize] = 1;
                    } else {
                        slice[index as usize] = 0;
                    }
                    i += 4;
                }
                9 => {
                    let a = get_value(mode_1, &slice, relative_position, i + 1);
                    relative_position += a;
                    i += 2;
                }
                99 => break,
                _ => panic!("Unexpected opcode: {}", opcode),
            }
        }

        latest_output
    }
}
