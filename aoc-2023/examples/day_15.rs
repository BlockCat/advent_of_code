type InputType = Vec<usize>;

pub fn main() {
    let input = parse(include_str!("../input/day_15.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

// #[test]
// fn test() {
//     let input = parse(include_str!("../input/test.txt"));

//     println!("Exercise 1: {}", exercise_1(input.clone()));
//     println!("Exercise 2: {}", exercise_2(input));

// }

fn parse<'a>(input: &'a str) -> InputType {
    // let grid: Vec<Vec<Tile>> = input.lines().map(|line| parse_line(line)).collect();
    // StaticGrid::from_vec(grid)
}

fn parse_line(line: &str) -> Vec<usize> {
    // line.chars().map(|c| Tile::from(c)).collect()
}

fn exercise_1(input: InputType) -> usize {
    0
}

fn exercise_2(input: InputType) -> usize {
    0
}
