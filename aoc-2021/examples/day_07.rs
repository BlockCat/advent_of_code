type Input = Vec<isize>;
pub fn main() {
    let input = parse_input(include_str!("../input/day07.txt"));
    println!("Ex1: {}", exercise_1(&input));
    println!("Ex2: {}", exercise_2(&input));
}

fn parse_input(input: &str) -> Input {
    let mut input: Input = input.split(',').map(|x| x.parse().unwrap()).collect();
    input.sort();
    input
}

fn exercise_1(input: &Input) -> usize {
    cost(input, input[input.len() / 2])
}

fn cost(input: &Input, pos: isize) -> usize {
    input.iter().map(|tar| (tar - pos).abs()).sum::<isize>() as usize
}

// f(x) = ?
// f'(x) = \sum_{a_i} (\abs{a_i - x} + 0.5)
//       = 0.5n - nx + \sum_{a_i} a_i
// =>
// 0.5n - nx + \sum_{a_i} a_i === 0
// 0.5n + \sum_{a_i} a_i === nx
// 0.5 + (\sum_{a_i} a_i)/n === x
// => x = \avg{A} + 0.5
fn exercise_2(input: &Input) -> usize {
    let sum: isize = input.iter().sum();
    let avg = (sum + 1) / input.len() as isize;
    cost2(input, avg)
}
fn cost2(input: &Input, pos: isize) -> usize {
    input
        .iter()
        .map(|tar| (tar - pos).abs())
        .map(|pos| (pos + 1) * pos)
        .sum::<isize>() as usize
        / 2
}
