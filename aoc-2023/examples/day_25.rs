type InputType = Vec<Vec<char>>;
pub fn main() {
    let input = parse(include_str!("../input/day_25.txt"));

    println!("Exercise 1: {}", exercise_1(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    println!("Exercise 1: {}", exercise_1(input));
    // assert_eq!(62, exercise_1(input.clone()));
    // assert_eq!(952408144115, exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: InputType) -> usize {
    0
}
