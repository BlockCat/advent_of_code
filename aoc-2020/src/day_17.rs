use hashbrown::HashSet;
use utils::VectorN;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day17.txt"));
    println!("{}", exercise_1(&input, 160));
    // let input = read_input_hack(include_str!("input/day17.txt"));
    println!("{}", exercise_2(&input, 160));
}

type Vector = VectorN<4>;

fn read_input(input: &str) -> HashSet<Vector> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Vector {
                        value: [x as isize, y as isize, 0, 0],
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn exercise_1(input: &HashSet<Vector>, iterations: usize) -> usize {
    let offsets: Vec<Vector> = (-1..=1)
        .flat_map(|x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).map(move |z| Vector {
                    value: [x, y, z, 0],
                })
            })
        })
        .filter(|x| x.value != [0, 0, 0, 0])
        .collect::<Vec<_>>();

    exercise_a(input, iterations, offsets)
}

fn exercise_2(input: &HashSet<Vector>, iterations: usize) -> usize {
    let offsets: Vec<Vector> = (-1..=1)
        .flat_map(|x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).flat_map(move |z| {
                    (-1..=1).map(move |w| Vector {
                        value: [x, y, z, w],
                    })
                })
            })
        })
        .filter(|x| x.value != [0, 0, 0, 0])
        .collect::<Vec<_>>();

    exercise_a(input, iterations, offsets)
}

fn exercise_b<const N: usize>(input: &HashSet<VectorN<N>>, iterations: usize) -> usize {
    let mut old_set = input.clone();
    for _ in 0..iterations {
        let cands = candidates(&old_set, &offsets);
        old_set = cands
            .into_iter()
            .filter(|v| {
                let count = offsets
                    .iter()
                    .filter_map(|w| old_set.get(&(w.clone() + v.clone())))
                    .count();

                match (old_set.contains(v), count) {
                    (true, 2) => true,
                    (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                }
            })
            .collect();
    }

    old_set.len()
}

fn exercise_a(input: &HashSet<Vector>, iterations: usize, offsets: Vec<Vector>) -> usize {
    let mut old_set = input.clone();
    for _ in 0..iterations {
        let cands = candidates(&old_set, &offsets);
        old_set = cands
            .into_iter()
            .filter(|v| {
                let count = offsets
                    .iter()
                    .filter_map(|w| old_set.get(&(w.clone() + v.clone())))
                    .count();

                match (old_set.contains(v), count) {
                    (true, 2) => true,
                    (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                }
            })
            .collect();
    }

    old_set.len()
}

fn candidates<const N: usize>(
    input: &HashSet<VectorN<N>>,
    offsets: &Vec<VectorN<N>>,
) -> HashSet<VectorN<N>> {
    input
        .iter()
        .flat_map(|v| offsets.iter().map(move |w| v.clone() + w.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d17ex1() {
        let input = read_input(include_str!("input/day17test.txt"));
        assert_eq!(112, exercise_1(&input, 6))
    }

    #[test]
    fn d17ex2() {
        let input = read_input(include_str!("input/day17test.txt"));
        assert_eq!(848, exercise_2(&input, 6));
    }

    #[bench]
    fn d17_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day17.txt"));
        b.iter(|| exercise_1(&input, 6));
    }

    #[bench]
    fn d17_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day17.txt"));
        b.iter(|| exercise_2(&input, 6));
    }
}
