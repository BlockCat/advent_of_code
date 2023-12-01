use rayon::prelude::*;

type InputType = Vec<u32>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1::<4>(numbers.clone()));
    println!("Exercise 2: {}", exercise_1::<14>(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_06.txt")
        .chars()
        .map(|c| (1 << (c as u8) - b'a'))
        .collect()
}

fn exercise_1<const N: u32>(input: InputType) -> usize {
    let a = input
        .par_windows(N as usize)        
        .position_first(|x| x.iter().fold(0u32, |acc, x| acc | x).count_ones() == N)
        .unwrap();
    a + N as usize
}
