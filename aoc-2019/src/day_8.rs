use crate::test::Bencher;

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day8.txt"));
    println!("ex1: {}", exercise_1(input.clone(), (25, 6)));
    println!("ex2: {}", exercise_2(input, (25, 6)));
}

fn exercise_1(input: Vec<u8>, (width, height): (usize, usize)) -> u32 {
    let m = input
        .chunks(width * height)
        .map(|chunk| {
            chunk.iter().fold([0u32; 3], |mut acc, x| {
                acc[*x as usize] += 1;
                acc
            })
        })
        .min()
        .unwrap();

    m[1] * m[2]
}

fn exercise_2(input: Vec<u8>, (width, height): (usize, usize)) -> i32 {
    let mut output = vec![2u8; width * height];
    for chunk in input.chunks(width * height) {
        for (index, pixel) in chunk.iter().enumerate() {
            if output[index] == 2 {
                output[index] = *pixel;
            }
        }
    }

    let mut index = 0;
    for _ in 0..height {
        for _ in 0..width {            
            let p = output[index];
            match p {
                0 => print!(" "),
                1 => print!("■"),
                _ => print!(" "),
            }
            index += 1;
        }
        println!();
    }

    0
}

fn read_input<'a>(input: &'a str) -> Vec<u8> {
    input.chars().map(|x| x as u8 - '0' as u8).collect()
}

#[test]
fn d8_test() {    
}

#[bench]
fn d8_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day8.txt"));
    b.iter(|| exercise_1(input.clone(), (25, 6)));
}

#[bench]
fn d8_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day8.txt"));
    b.iter(|| exercise_2(input.clone(), (25, 6)));
}

#[bench]
fn d8_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day8.txt")));
}
