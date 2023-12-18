use aoc_2023::{direction::Direction, vector::Vector2};

type InputType = Vec<Entry>;

pub fn main() {
    let input = parse(include_str!("../input/day_18.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    assert_eq!(62, exercise_1(input.clone()));
    assert_eq!(952408144115, exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Entry {
    let mut l = line.split_whitespace();
    let direction = match l.next().unwrap() {
        "L" => Direction::West,
        "U" => Direction::North,
        "D" => Direction::South,
        "R" => Direction::East,
        _ => unreachable!(),
    };
    let number = l.next().unwrap().parse().unwrap();

    let colour = l.next().unwrap().to_string();

    Entry {
        direction,
        number,
        colour,
    }
}

fn exercise_1(input: InputType) -> usize {
    let mut pos = Vector2::new([0, 0]);

    let mut events = Vec::new();

    for en in input {
        let old = pos;
        for _ in 0..en.number {
            pos = pos + en.direction;
        }
        match en.direction {
            Direction::East => events.push((old, pos)),
            Direction::West => events.push((pos, old)),
            _ => {}
        }
    }
    events.sort_by_key(|a| (a.0[1], a.0[0]));

    let mut ranges: Vec<(isize, isize)> = Vec::new();
    let mut counter = 0;
    let mut prev_y = events[0].0[1];

    for event in events {
        let y = event.0[1];
        let sx = event.0[0];
        let ex = event.1[0];

        if y != prev_y {
            counter += ranges.iter().map(|(a, b)| (b - a + 1)).sum::<isize>() * (y - prev_y);
            prev_y = y;
        }

        let start_range = ranges.iter().position(|(a, b)| (*a..=*b).contains(&sx));
        let end_range = ranges.iter().position(|(a, b)| (*a..=*b).contains(&ex));
        match (start_range, end_range) {
            (Some(p1), Some(p2)) if p1 == p2 => {
                let a = ranges.remove(p1);
                let left_range = (a.0, sx);
                let right_range = (ex, a.1);
                if left_range.1 - left_range.0 > 0 {
                    ranges.push(left_range);
                    counter -= left_range.1 - left_range.0 + 1;
                }
                if right_range.1 - right_range.0 > 0 {
                    ranges.push(right_range);
                    counter -= right_range.1 - right_range.0 + 1;
                }

                counter += a.1 - a.0 + 1;
            }
            (Some(p1), Some(p2)) => {
                let a = ranges.remove(p2.max(p1));
                let b = ranges.remove(p2.min(p1));

                ranges.push((a.0.min(b.0), a.1.max(b.1)));
            }
            (Some(p1), None) => {
                let a = ranges.remove(p1);
                ranges.push((a.0.min(sx), a.1.max(ex)));
            }
            (None, Some(p2)) => {
                let a = ranges.remove(p2);
                ranges.push((a.0.min(sx), a.1.max(ex)));
            }
            (None, None) => {
                ranges.push((sx, ex));
            }
        }
    }

    counter as usize
}

fn exercise_2(input: InputType) -> usize {
    let input = input
        .into_iter()
        .map(|s| {
            let hexa = &s.colour[2..8];
            assert!(hexa.len() == 6);

            let number = usize::from_str_radix(&hexa[0..5], 16).unwrap();

            let direction = match &hexa[5..6] {
                "0" => Direction::East,
                "1" => Direction::South,
                "2" => Direction::West,
                "3" => Direction::North,
                _ => unreachable!(),
            };

            Entry {
                direction,
                number,
                colour: s.colour.clone(),
            }
        })
        .collect();

    exercise_1(input)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    direction: Direction,
    number: usize,
    colour: String,
}
