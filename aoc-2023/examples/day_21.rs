use aoc_2023::{
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use std::collections::{HashMap, HashSet, VecDeque};

type InputType = StaticGrid<char>;

// 637537341306357 <--
// 637537404423705
pub fn main() {
    let input = parse(include_str!("../input/day_21.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone(), 64));

    // println!("Exercise 2a: {}", exercise_2a(&input));
    println!("Exercise 2: {}", exercise_2(input, 26501365));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    let res = exercise_1(input.clone(), 6);
    println!("Exercise 1: {}", res);
    assert_eq!(res, 16);
    // let res = exercise_1(input.clone(), 60);
    // println!("Exercise 1: {}", res);

    assert_eq!(exercise_2(input.clone(), 6), 16);
    assert_eq!(exercise_2(input.clone(), 10), 50);
    let res = exercise_2(input, 100);
    println!("Exercise 2: {}", res);
    assert_eq!(res, 6536);
}

fn parse<'a>(input: &'a str) -> InputType {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_1(input: InputType, count: usize) -> usize {
    let start_pos: aoc_2023::vector::VectorN<2> = input.iter().find(|s| *s.1 == 'S').unwrap().0;

    calculate(&input, count, start_pos, 0)
}

fn exercise_2(input: InputType, count: usize) -> usize {
    let start_pos: aoc_2023::vector::VectorN<2> = input.iter().find(|s| *s.1 == 'S').unwrap().0;

    help(&input, start_pos, count)
}

fn calculate(input: &InputType, count: usize, start_pos: Vector2, start_steps: usize) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back((start_pos, start_steps));

    let mut visited = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        if steps > count {
            continue;
        }
        if !visited.insert((pos, steps % 2 == 0)) {
            continue;
        }

        for n in pos.neighbours_4() {
            if let Some(c) = input.get_vec(&n) {
                if *c == '.' || *c == 'S' {
                    queue.push_back((n, steps + 1));
                }
            }
        }
    }
    let find_even = (count % 2) == 0;

    visited.iter().filter(|s| s.1 == find_even).count()
}

fn help(input: &InputType, start_pos: Vector2, count: usize) -> usize {
    let tile_width = count / input.width;
    let w = input.width as isize;
    let h = input.height as isize;
    let straight_quadrants = [
        calculate_quadrant(input, Vector2::new([0, h / 2])), //l
        calculate_quadrant(input, Vector2::new([w - 1, h / 2])), //r
        calculate_quadrant(input, Vector2::new([w / 2, 0])), //t
        calculate_quadrant(input, Vector2::new([w / 2, h - 1])), //b
    ];
    let distance_into_straight = (start_pos[0] as usize + count) % input.width;

    let endings_count: usize = straight_quadrants
        .iter()
        .map(|s| count_quadrant(s, distance_into_straight, 0))
        .sum();

    let corner_quadrants = [
        calculate_quadrant(input, Vector2::new([0, 0])), //tl
        calculate_quadrant(input, Vector2::new([w - 1, 0])), //tr
        calculate_quadrant(input, Vector2::new([0, h - 1])), //bl
        calculate_quadrant(input, Vector2::new([w - 1, h - 1])), //br
    ];

    let corner_1_count: usize = corner_quadrants
        .iter()
        .map(|s| tile_width * count_quadrant(s, input.width / 2, 0))
        .sum();

    let corner_2_count: usize = corner_quadrants
        .iter()
        .map(|s| (tile_width - 1) * count_quadrant(s, input.width + input.width / 2, 1))
        .sum();

    let central_quadrant = calculate_quadrant(input, start_pos);

    let odd_contained = (tile_width).pow(2);
    let even_contained = (tile_width - 1).pow(2);

    let odd_count = odd_contained * count_quadrant(&central_quadrant, 90000, 0);
    let event_count = even_contained * count_quadrant(&central_quadrant, 90000, 1);

    endings_count + corner_1_count + corner_2_count + odd_count + event_count
}

fn count_quadrant(quadrant: &HashMap<Vector2, usize>, count: usize, carry: usize) -> usize {
    quadrant
        .iter()
        .filter(|s| *s.1 <= count && s.1 % 2 == carry)
        .count()
}

fn calculate_quadrant(
    input: &InputType,
    start_pos: Vector2,
) -> HashMap<aoc_2023::vector::VectorN<2>, usize> {
    let mut queue = VecDeque::new();

    queue.push_back((start_pos, 0));

    let mut visited = HashMap::new();

    while let Some((pos, steps)) = queue.pop_front() {
        if visited.contains_key(&pos) {
            continue;
        }
        visited.insert(pos, steps);

        for n in pos.neighbours_4() {
            if let Some(c) = input.get_vec(&n) {
                if *c == '.' || *c == 'S' {
                    queue.push_back((n, steps + 1));
                }
            }
        }
    }
    visited
}
