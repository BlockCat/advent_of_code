use aoc_2024::stopwatch;
use rayon::iter::{ParallelBridge, ParallelIterator};

type Input = Vec<Vec<char>>;

pub fn main() {
    let input = include_str!("../input/bigboi4.txt");

    let l = stopwatch(|| {
        let input = parse(input);

        let a1 = exercise_1(&input);
        println!("Ex1: {}", a1);
        let a2 = exercise_2(&input);
        println!("Ex2: {}", a2);
    });

    println!("Time: {:?}", l);
}

fn parse(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn exercise_1(input: &Input) -> usize {
    (0..input.len())
        .par_bridge()
        .map(|i| {
            (0..input.len())
                .map(|j| count_from(input, i, j))
                .sum::<usize>()
        })
        .sum()
}

fn exercise_2(input: &Input) -> usize {
    (1..input.len() - 1)
        .par_bridge()
        .map(|i| (1..input.len() - 1).filter(|j| has_x(input, i, *j)).count())
        .sum()
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn count_from(input: &Input, x: usize, y: usize) -> usize {
    DIRS.into_iter()
        .filter(|(i, j)| {
            let x2 = x as isize + i * 3;
            let y2 = y as isize + j * 3;
            let len = input.len() as isize;
            x2 >= 0
                && y2 >= 0
                && x2 < len
                && y2 < len
                && (0..4)
                    .map(|index| {
                        let x = x as isize + i * index;
                        let y = y as isize + j * index;
                        input[x as usize][y as usize]
                    })
                    .eq(['X', 'M', 'A', 'S'])
        })
        .count()
}

fn has_x(input: &Input, x: usize, y: usize) -> bool {
    if input[x][y] != 'A' {
        return false;
    }

    let diag_1 = [input[x - 1][y - 1], input[x + 1][y + 1]];
    let diag_2 = [input[x - 1][y + 1], input[x + 1][y - 1]];

    let bool_1 = diag_1 == ['M', 'S'] || diag_1 == ['S', 'M'];
    let bool_2 = diag_2 == ['M', 'S'] || diag_2 == ['S', 'M'];

    bool_1 && bool_2
}
