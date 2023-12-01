type InputType = impl Iterator<Item = (u8, u8)>;

pub fn main() {
    println!("Exercise 1: {}", exercise_1(input()));
    println!("Exercise 2: {}", exercise_2(input()));
}

fn input() -> InputType {
    include_bytes!("../input/day_02.txt")
        .chunks(4)
        .map(|x| (x[0], x[2]))
}

fn exercise_1(input: InputType) -> usize {
    0
}

fn exercise_2(input: InputType) -> usize {
    0
}
