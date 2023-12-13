type InputType = Vec<Vec<Vec<bool>>>;

pub fn main() {
    let input = parse(include_str!("../input/day_13.txt"));
    println!("{:?}", input.len());
    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    let mut list = vec![];

    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        list.push(
            lines
                .by_ref()
                .take_while(|s| !s.is_empty())
                .map(|s| (parse_line(s)))
                .collect::<Vec<_>>(),
        );
    }
    list
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn exercise_1(input: InputType) -> usize {
    input
        .iter()
        .map(|s| match find_reflection(s, find_reflection_in_vec) {
            Line::Horizontal(i) => i * 100,
            Line::Vertical(i) => i,
        })
        .sum()
}

fn exercise_2(input: InputType) -> usize {
    input
        .iter()
        .map(
            |s| match find_reflection(s, find_reflection_in_vec_with_smudge) {
                Line::Horizontal(i) => i * 100,
                Line::Vertical(i) => i,
            },
        )
        .sum()
}

fn find_reflection<F>(input: &Vec<Vec<bool>>, reflection_finder: F) -> Line
where
    F: Fn(&Vec<Vec<bool>>) -> Option<usize>,
{
    if let Some(i) = reflection_finder(input) {
        return Line::Horizontal(i);
    }
    let height = input.len();
    let width = input[0].len();
    let mut transposed = vec![vec![false; height]; width];
    for x in 0..width {
        for y in 0..height {
            transposed[x][y] = input[y][x];
        }
    }

    if let Some(i) = reflection_finder(&transposed) {
        return Line::Vertical(i);
    }
    panic!("No reflection found");
}

fn find_reflection_in_vec(input: &Vec<Vec<bool>>) -> Option<usize> {
    let height = input.len();
    'outer: for index in 1..height {
        let dist = (height - index).min(index);
        for offset in 0..dist {
            if input[index - offset - 1] != input[index + offset] {
                continue 'outer;
            }
        }

        return Some(index);
    }

    None
}

fn find_reflection_in_vec_with_smudge(input: &Vec<Vec<bool>>) -> Option<usize> {
    let height = input.len();
    let width = input[0].len();
    'outer: for index in 1..height {
        let dist = (height - index).min(index);
        let mut counts = 0;
        for offset in 0..dist {
            for k in 0..width {
                if input[index - offset - 1][k] != input[index + offset][k] {
                    counts += 1;
                    if counts > 1 {
                        continue 'outer;
                    }
                }
            }
        }

        if counts == 1 {
            return Some(index);
        }
    }

    None
}

enum Line {
    Horizontal(usize),
    Vertical(usize),
}
