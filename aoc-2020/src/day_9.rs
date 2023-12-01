// use crate::test::Bencher;
use std::iter::FromIterator;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day9.txt"));
    let e1r = exercise_1(&input, 25);
    println!("{}", e1r);
    println!("{}", exercise_2(&input, e1r));
}

pub fn read_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>()
}

fn exercise_1(slice: &Vec<usize>, preamble: usize) -> usize {
    slice
        .windows(preamble + 1)
        .find(|x| {
            let mut v = Vec::from_iter(x[0..x.len() - 1].iter().cloned());
            v.sort();
            let val = find_1(&v, *x.last().unwrap());
            // println!("{:?}, with {}", v, x.last().unwrap());
            val.is_none()
        })
        .map(|x| x.last().unwrap().clone())
        .unwrap()
}

pub fn find_1(slice: &[usize], target: usize) -> Option<usize> {
    let mut low = 0;
    let mut up = slice.len() - 1;

    while up > low {
        if slice[low] + slice[up] == target {
            return Some(slice[low] * slice[up]);
        } else if slice[low] + slice[up] > target {
            up -= 1;
        } else {
            low += 1;
        }
    }

    return None;
}

fn exercise_2(slice: &Vec<usize>, target: usize) -> usize {
    let mut sum = slice[0] + slice[1];

    let mut start = 0;
    let mut end = 1;

    while sum != target {
        if sum < target {
            end += 1;
            sum += slice[end];
        } else {
            sum -= slice[start];
            start += 1;
        }
    }

    let v = &slice[start..=end];

    println!("{:?}", v);

    let (min, max) = v.iter().fold((std::usize::MAX, 0), |(min, max), &x| {
        (min.min(x), max.max(x))
    });

    min + max
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d9p1_test() {
        let input = read_input(include_str!("input/day9.txt"));
        assert_eq!(69316178, exercise_1(&input, 25));
    }

    #[test]
    fn d9p2_test() {
        let input = read_input(include_str!("input/day9.txt"));
        assert_eq!(9351526, exercise_2(&input, 69316178));
    }

    #[bench]
    fn d9_bench_parse(b: &mut Bencher) {
        b.iter(|| read_input(include_str!("input/day9.txt")));
    }
    #[bench]
    fn d9_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day9.txt"));
        b.iter(|| exercise_1(&input, 25));
    }
    #[bench]
    fn d9_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day9.txt"));
        b.iter(|| exercise_2(&input, 69316178));
    }
}
