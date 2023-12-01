use crate::test::Bencher;

#[test]
pub fn run() {
    let mut input = read_input(include_str!("input/day2.txt"));
    input[1] = 12;
    input[2] = 2;
    println!("{}", exercise_1(input.clone()));
    println!("{}", exercise_2(input.clone()));
}

pub fn exercise_1(slice: Vec<i32>) -> i32 {
    run_program(12, 2, slice)
}

pub fn exercise_2(slice: Vec<i32>) -> i32 {
    let target = 19690720i32;
    let offset = run_program(0, 0, slice.clone());
    let diff = run_program(1, 0, slice.clone()) - offset;
    let noun = (target - offset) / diff;

    let r = run_program(noun, 0, slice.clone());
    let verb = (target - r) / (run_program(noun, 1, slice.clone()) - r);

    assert_eq!(run_program(noun, verb, slice.clone()), target);
    noun * 100 + verb
}

fn run_program(noun: i32, verb: i32, mut slice: Vec<i32>) -> i32 {
    slice[1] = noun;
    slice[2] = verb;
    for i in (0..slice.len()).step_by(4) {
        if slice[i] == 99 {
            break;
        }
        let index = slice[i + 3] as usize;
        let a = slice[slice[i + 1] as usize];
        let b = slice[slice[i + 2] as usize];

        match slice[i] {
            1 => slice[index] = a + b,
            2 => slice[index] = a * b,            
            _ => panic!(),
        }
    }

    slice[0]
}
pub fn read_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn d2_test() {
    assert_eq!(run_program(0, 0, read_input("1,0,0,0,99")), 2);
    assert_eq!(run_program(1, 1, read_input("1,1,1,4,99,5,6,0,99")), 30);
    assert_eq!(exercise_1(read_input(include_str!("input/day2.txt"))), 5098658);
    assert_eq!(exercise_2(read_input(include_str!("input/day2.txt"))), 5064);
}

#[bench]
fn d2_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day2.txt")));
}
#[bench]
fn d2_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day2.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d2_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day2.txt"));
    b.iter(|| exercise_2(input.clone()));
}
