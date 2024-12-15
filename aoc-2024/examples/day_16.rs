use aoc_2024::stopwatch;

type Input = Vec<u32>;

pub fn main() {
    let numbers = input(include_str!("../input/day_16.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        // println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> usize {

}

fn exercise_1(input: &Input) -> usize {
    unimplemented!()
}
fn exercise_2(input: &Input) -> usize {
    unimplemented!()    
}