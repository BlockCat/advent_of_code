use crate::test::Bencher;

const BASE: [i32; 4] = [0, 1, 0, -1];

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day16.txt")); // 53494818 too high
    println!(
        "ex1: {}",
        exercise_1(input.clone())
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>()
    );
    println!(
        "ex2: {}",
        exercise_2(input.clone())
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<String>()
    );
}

fn exercise_1(mut input: Vec<i32>) -> Vec<i32> {
    for _ in 0..100 {
        for pos in 0..input.len() / 2 {
            let input = &mut input[pos..];            
            input[0] = (input
                .iter()
                .enumerate()
                .map(|(index, c)| (BASE[get_index(pos, index + pos, 1)] * *c))
                .sum::<i32>())
                .abs();
        }
        for pos in (input.len() / 2..input.len() - 1).rev() {            
            input[pos] += input[pos+1];            
        }
        for x in &mut input {
            *x %= 10;
        }
    }
    input
}

fn exercise_2(input: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    for x in &input[0..7] {
        sum *= 10;
        sum += x;
    }
    let sum = sum as usize;
    let input_len = input.len() * 10_000;

    assert!(sum >= (input_len / 2) && sum <= input_len);

    let mut input = std::iter::repeat(input)
        .take(10_000)
        .flatten()
        .skip(sum)
        .collect::<Vec<_>>();
    for _ in 0..100 {
        for i in (0..input.len() - 1).rev() {
            input[i] += input[i + 1];
        }
        for x in input.iter_mut() {
            *x %= 10;
        }
    }

    input[0..8].iter().cloned().collect::<Vec<_>>()
}

fn read_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|x| (x as u8 - '0' as u8) as i32)
        .collect()
}

fn get_index(position: usize, index: usize, offset: usize) -> usize {
    ((index + offset) / (position + 1)) & 0b11
}

#[test]
fn d16_test() {
    assert_eq!((-13i32 % 10).abs(), 3);
    assert_eq!(
        (0..6).map(|x| get_index(0, x, 1)).collect::<Vec<_>>(),
        vec!(1, 2, 3, 0, 1, 2)
    );
    assert_eq!(
        (0..6).map(|x| get_index(1, x, 1)).collect::<Vec<_>>(),
        vec!(0, 1, 1, 2, 2, 3)
    );
    assert_eq!(
        (0..6).map(|x| get_index(2, x, 1)).collect::<Vec<_>>(),
        vec!(0, 0, 1, 1, 1, 2)
    );

    assert_eq!(
        exercise_1(read_input("80871224585914546619083218645595"))
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>(),
        String::from("24176176")
    );
    assert_eq!(
        exercise_1(read_input("19617804207202209144916044189917"))
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>(),
        String::from("73745418")
    );
    assert_eq!(
        exercise_1(read_input("69317163492948606335995924319873"))
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>(),
        String::from("52432133")
    );

    assert_eq!(
        exercise_2(read_input("03036732577212944063491565474664"))
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>(),
        String::from("84462026")
    );
    assert_eq!(
        exercise_2(read_input("02935109699940807407585447034323"))
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>(),
        String::from("78725270")
    );
    assert_eq!(
        exercise_2(read_input("03081770884921959731165446850517"))
            .into_iter()
            .take(8)
            .map(|x| format!("{}", x))
            .collect::<String>(),
        String::from("53553731")
    );
}

#[bench]
fn d16_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day16.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d16_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day16.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d16_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day16.txt")));
}

/*
3*1  + 4*0  + 0*-1 + 4*0  + 0*1  + 4*0  + 3*-1 + 8*0  = 0
 3*0  + 4*1  + 0*1  + 4*0  + 0*0  + 4*-1 + 3*-1 + 8*0  = 3
  3*0  + 4*0  + 0*1  + 4*1  + 0*1  + 4*0  + 3*0  + 8*0  = 4
   3*0  + 4*0  + 0*0  + 4*1  + 0*1  + 4*1  + 3*1  + 8*0  = 1
    3*0  + 4*0  + 0*0  + 4*0  + 0*1  + 4*1  + 3*1  + 8*1  = 5
     3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*1  + 3*1  + 8*1  = 5
      3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*1  + 8*1  = 1
       3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*0  + 8*1  = 8



*/
