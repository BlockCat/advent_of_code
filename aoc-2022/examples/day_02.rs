const WIN_TABLE: [usize; 9] = [3, 6, 0, 0, 3, 6, 6, 0, 3];
const ROUND_TABLE: [usize; 9] = [3, 1, 2, 1, 2, 3, 2, 3, 1];

pub fn main() {
    println!("Exercise 1: {}", exercise_1(input()));
    println!("Exercise 2: {}", exercise_2(input()));
}

fn input() -> impl Iterator<Item = (u8, u8)> {
    include_bytes!("../input/day_02.txt")
        .chunks(4)
        .map(|x| (x[0], x[2]))
}

fn exercise_1(input: impl Iterator<Item = (u8, u8)>) -> usize {
    input
        .map(|(a, b)| {
            let a = a - b'A';
            let b = b - b'X';
            1 + b as usize + WIN_TABLE[(a * 3 + b) as usize]
        })
        .sum()
}

fn exercise_2(input: impl Iterator<Item = (u8, u8)>) -> usize {
    input
        // .par_iter()
        .map(|(da, db)| {
            let a = da as u8 - b'A';
            let b = db as u8 - b'X';
            3 * b as usize + ROUND_TABLE[(a * 3 + b) as usize]
        })
        .sum()
}
