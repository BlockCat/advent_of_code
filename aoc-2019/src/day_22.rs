use crate::test::Bencher;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Instruction {
    DealStack,
    DealIncrement(i128),
    Cut(i128),
}

//#[test]
pub fn run() {
    let input = read_input(include_str!("input/day22.txt"));
    println!("ex1: {}", exercise_1(&input.clone(), 10007, 2019));
    println!("ex2: {}", exercise_2(input));
}

fn read_input(input: &str) -> Vec<Instruction> {
    input.lines().map(read_line).collect()
}

fn read_line(line: &str) -> Instruction {
    let line = line.trim();
    if line.starts_with("cut") {
        Instruction::Cut(line[4..].parse().expect(&line[4..]))
    } else if line.starts_with("deal into new stack") {
        Instruction::DealStack
    } else if line.starts_with("deal with increment") {
        Instruction::DealIncrement(line[20..].parse().expect(&line[20..]))
    } else {
        panic!("Invalid line: {}", line)
    }
}

fn exercise_1(input: &Vec<Instruction>, size: i128, track: i128) -> i128 {
    let mut position = track;
    for instruction in input {
        match instruction {
            Instruction::DealStack => {
                position = (size - 1) - position;
            }
            Instruction::Cut(c) => {
                position = (position - c).rem_euclid(size);
            }
            Instruction::DealIncrement(increment) => {
                position = (position * increment) % size;
            }
        }
    }
    position
}

fn modpow(mut b: i128, mut p: i128, m: i128) -> i128 {
    let mut a = 1;
    while p > 0 {
        if p % 2 == 0 {
            b *= b;
            b %= m;
            p /= 2;
        } else {
            a *= b;
            a %= m;
            p -= 1;
        }
    }
    a
}

fn exercise_2(input: Vec<Instruction>) -> i128 {
    let size = 119315717514047;
    let repeats = 101741582076661usize;
    let mut b = 1;
    let mut a = 0;
    for instruction in input.iter().rev() {
        match instruction {
            Instruction::DealStack => {
                b = (size - b) % size;
                a = (size - a + size - 1) % size;
            }
            Instruction::Cut(cut) => {
                a = (a + cut.rem_euclid(size)) % size;
            }
            Instruction::DealIncrement(increment) => {
                let x = modpow(*increment, size - 2, size);
                b = (b * x) % size;
                a *= x;
            }
        };
    }
    let m1 = modpow(b - 1, size - 2, size);
    let r = (b - 1) * 2020 % size;
    let r = (r + a) % size;
    let r = r * modpow(b, 101741582076661, size) % size;
    let r = (r - a) % size;
    (r * m1) % size
}

#[test]
fn d22_test() {
    println!("start 1");
    let input = r"deal with increment 7
    deal into new stack
    deal into new stack";
    let output = &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
    let input = read_input(input);
    for (i, x) in output.iter().enumerate() {
        assert_eq!(exercise_1(&input, 10, *x), i as i128);        
    }
    println!("start 2");
    let input = r"cut 6
    deal with increment 7
    deal into new stack";
    let output = &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
    let input = read_input(input);

    for (i, x) in output.iter().enumerate() {
        assert_eq!(exercise_1(&input, 10, *x), i as i128);
    }

    println!("start 3");
    let input = r"deal with increment 7
    deal with increment 9
    cut -2";
    let output = &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
    let input = read_input(input);
    for (i, x) in output.iter().enumerate() {
        assert_eq!(exercise_1(&input, 10, *x), i as i128);
    }

    println!("start 4");

    let input = r"deal into new stack
    cut -2
    deal with increment 7
    cut 8
    cut -4
    deal with increment 7
    cut 3
    deal with increment 9
    deal with increment 3
    cut -1";
    let input = read_input(input);
    let output = &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
    for (i, x) in output.iter().enumerate() {
        assert_eq!(exercise_1(&input, 10, *x), i as i128);
    }

    println!("start 5");
    let input = read_input(include_str!("input/day22.txt"));
    assert_eq!(exercise_1(&input, 10007, 2019), 7171);
}

#[bench]
fn d22_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day22.txt"));
    b.iter(|| exercise_1(&input, 10007, 2019));
}

#[bench]
fn d22_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day22.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d22_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day22.txt")));
}
