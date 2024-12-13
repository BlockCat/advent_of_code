use aoc_2024::grouped_lines;
use aoc_2024::stopwatch;
use num_traits::Euclid;
type Input = Vec<ButtonSet>;

pub fn main() {
    let numbers = input(include_str!("../input/day_13.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    grouped_lines(input)
        .map(|mut x| {
            let ba = x.next().unwrap();
            let bb = x.next().unwrap();
            let prize = x.next().unwrap();

            ButtonSet {
                button_a: parse_line(ba),
                button_b: parse_line(bb),
                prize: parse_line(prize),
            }
        })
        .collect()
}

fn parse_line(b: &str) -> (usize, usize) {
    let offsets = &b[b.find(':').unwrap() + 4..];
    let offset_komma = offsets.find(',').unwrap();

    let x = &offsets[..offset_komma];
    let y = &offsets[offset_komma + 4..];

    let x = x.parse().unwrap();
    let y = y.parse().unwrap();

    (x, y)
}

fn exercise_1(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|x| lowest_cost(x, 0))
        .map(|(a, b)| a * 3 + b)
        .sum()
}
fn exercise_2(input: &Input) -> usize {
    input
        .iter()
        .filter_map(|x| lowest_cost(x, 10000000000000))
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn lowest_cost(set: &ButtonSet, offset: usize) -> Option<(usize, usize)> {
    let prize_x = offset + set.prize.0;
    let prize_y = offset + set.prize.1;

    let a1 = prize_x as isize;
    let a2 = prize_y as isize;
    let b1 = set.button_a.0 as isize;
    let b2 = set.button_a.1 as isize;
    let c1 = set.button_b.0 as isize;
    let c2 = set.button_b.1 as isize;

    // [a1] = [b1, c1] * [x1]
    // [a2] = [b2, c2] * [x2]
    // \vec{a} = M * \vec{x}
    // \vec{x} = M^-1 * \vec{a}

    // [x1] = [c2, -c1]           * [a1]
    // [x2] = [-b2, b1] / inv_det * [a2]

    let inv_determinant = b1 * c2 - b2 * c1;

    let (a, ra) = (a1 * c2 - a2 * c1).div_rem_euclid(&inv_determinant);
    let (b, rb) = (a2 * b1 - a1 * b2).div_rem_euclid(&inv_determinant);

    if ra == 0 && rb == 0 {
        Some((a as usize, b as usize))
    } else {
        None
    }
}

#[derive(Debug)]
struct ButtonSet {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}
