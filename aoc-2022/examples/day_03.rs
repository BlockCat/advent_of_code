pub fn main() {
    println!("Exercise 1: {}", exercise_1(input()));
    println!("Exercise 2: {}", exercise_2(input()));
}
fn input() -> Vec<(u64, u64)> {
    include_str!("../input/day_03.txt")
        .lines()
        .map(parse_line)
        .collect()
}
fn parse_line(line: &str) -> (u64, u64) {
    let (a, b) = line.split_at(line.len() / 2);
    (str_to_set(a), str_to_set(b))
}
fn str_to_set(part: &str) -> u64 {
    part.chars()
        .map(priority)
        .fold(0u64, |acc, x| acc | (1 << x))
}
fn priority(cha: char) -> u8 {
    let start = cha as u8 - b'A';
    start % 32 + (1 - start / 32) * 26 + 1
}
fn exercise_1(input: Vec<(u64, u64)>) -> u32 {
    input
        .into_iter()
        .map(|(a, b)| (a & b).trailing_zeros())
        .sum()
}
fn exercise_2(input: Vec<(u64, u64)>) -> u32 {
    input
        .chunks(3)
        .map(|x| (x[0].0 | x[0].1) & (x[1].0 | x[1].1) & (x[2].0 | x[2].1))
        .map(|group| group.trailing_zeros())
        .sum()
}
