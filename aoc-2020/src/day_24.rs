use hashbrown::{HashMap, HashSet};
use utils::Vector3;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day24.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input, 100));
}

enum Directions {
    EAST,
    WEST,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
}

const DIRECTIONS: [Directions; 6] = [
    Directions::EAST,
    Directions::WEST,
    Directions::NORTHEAST,
    Directions::NORTHWEST,
    Directions::SOUTHEAST,
    Directions::SOUTHWEST,
];

impl Directions {
    fn value(&self) -> Vector3 {
        match self {
            Directions::EAST => Vector3::new([1, -1, 0]),
            Directions::WEST => Vector3::new([-1, 1, 0]),
            Directions::NORTHEAST => Vector3::new([0, -1, 1]),
            Directions::NORTHWEST => Vector3::new([-1, 0, 1]),
            Directions::SOUTHEAST => Vector3::new([1, 0, -1]),
            Directions::SOUTHWEST => Vector3::new([0, 1, -1]),
        }
    }
}

type Input = Vec<Vec<Directions>>;
fn read_input(input: &str) -> Input {
    input.lines().map(read_line).collect()
}

fn read_line(input: &str) -> Vec<Directions> {
    let mut v = Vec::with_capacity(input.len() * 2 / 3);

    let mut it = input.chars();

    while let Some(c) = it.next() {
        v.push(match c {
            'e' => Directions::EAST,
            'w' => Directions::WEST,
            's' => match it.next() {
                Some('e') => Directions::SOUTHEAST,
                Some('w') => Directions::SOUTHWEST,
                _ => unreachable!(),
            },
            'n' => match it.next() {
                Some('e') => Directions::NORTHEAST,
                Some('w') => Directions::NORTHWEST,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        });
    }

    v
}

fn exercise_1(input: &Input) -> usize {
    create_set(input).len()
}

fn exercise_2(input: &Input, len: usize) -> usize {
    let mut set = create_set(input);

    for _ in 0..len {
        let mut new_set = HashSet::new();

        for candidate in find_candidates(&set) {
            let x = DIRECTIONS
                .iter()
                .map(|x| candidate + x.value())
                .filter(|x| set.contains(x))
                .count();
            if set.contains(&candidate) {
                if !(x == 0 || x > 2) {
                    new_set.insert(candidate);
                }
            } else {
                if x == 2 {
                    new_set.insert(candidate);
                }
            }
        }

        set = new_set;
    }
    set.len()
}

fn create_set(input: &Input) -> HashSet<Vector3> {
    let mut set = HashSet::new();
    for path in input {
        let c = find_tile(Vector3::new([0, 0, 0]), path);
        if set.contains(&c) {
            set.remove(&c);
        } else {
            set.insert(c);
        }
    }
    set
}

fn find_tile(start: Vector3, path: &Vec<Directions>) -> Vector3 {
    path.iter().fold(start, |acc, x| acc + x.value())
}

fn find_candidates(set: &HashSet<Vector3>) -> HashSet<Vector3> {
    set.iter()
        .flat_map(|x| DIRECTIONS.iter().map(move |d| *x + d.value()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d18ex1() {
        let input = read_input(include_str!("input/day24test.txt"));
        assert_eq!(10, exercise_1(&input));
        // assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input(include_str!("input/day24test.txt"));
        assert_eq!(2208, exercise_2(&input, 100));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day24.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day24.txt"));
        b.iter(|| exercise_1(&input));
    }
}
