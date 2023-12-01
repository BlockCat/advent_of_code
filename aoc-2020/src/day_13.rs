use itertools::*;
use utils::Vector2;

// #[test]
pub fn run() {
    let input = read_input1(include_str!("input/day13.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input.1));
}

pub fn read_input1(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.lines();

    let minutes = lines.next().and_then(|x| x.parse().ok()).unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            if x == "x" {
                None
            } else {
                Some(x.parse().unwrap())
            }
        })
        .collect();
    (minutes, busses)
}

fn exercise_1((arrival, busses): &(usize, Vec<Option<usize>>)) -> usize {
    let bus = busses
        .iter()
        .filter_map(|x| x.as_ref())
        .map(|&bus| (bus, bus - (arrival % bus)))
        .min_by_key(|(_, wait)| *wait)
        .unwrap();
    bus.0 * bus.1
}

fn exercise_2(input: &Vec<Option<usize>>) -> usize {
    let c = input[0].unwrap();

    // offset, bus
    let mut busses = input
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|x| x.1.map(|a| (x.0, a)))
        .collect::<Vec<_>>();

    busses.sort_by_key(|x| -(x.1 as isize));

    println!("{:?}", busses);

    // let mut step = 100000000000002 ;
    // let mut step = 7380589664952;
    let mut start = 0usize;
    let mut increment = c;

    for &(offset, bus) in &busses {
        let x = (start..)
            .step_by(increment)
            .find(|&x| ((x + offset) % bus == 0))
            .unwrap();

        start = x;
        // increment = increment / gcd(bus, increment) * bus ;
        increment *= bus;

        println!("bus: {}, cycle: {}, step: {}", bus, increment, start);
    }
    start
}

fn gcd2(mut a: isize, mut b: isize) -> (usize, isize, isize) {
    let mut x = 0isize;
    let mut y = 1isize;
    let mut lastx = 1isize;
    let mut lasty = 0isize;
    let mut temp;

    while b != 0 {
        let q = a / b;
        let r = a.rem_euclid(b);

        a = b;
        b = r;

        temp = x;
        x = lastx - q * x;
        lastx = temp;

        temp = y;
        y = lasty - q * y;
        lasty = temp;

        println!("{}, {}", lastx, lasty);
    }
    println!(">>{}, {}", lastx, lasty);
    (a as usize, lastx, lasty)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == 0 || b == 0 {
        return std::cmp::max(a, b);
    }
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    // #[test]
    // fn gcd2_test() {
    //     assert_eq!((6, 132, -535), gcd2(12378, 3054));
    //     let (r, a ,b) = gcd2(-7, 13);

    //     assert_eq!(6, a);
    //     assert_eq!(11, b);
    // }
    #[test]
    fn d13p1_test() {
        // let input = read_input(include_str!("input/day13.txt"));
        assert_eq!(
            295,
            exercise_1(&(
                939,
                vec![
                    Some(7),
                    Some(13),
                    None,
                    None,
                    Some(59),
                    None,
                    Some(31),
                    Some(19)
                ]
            ))
        );
    }

    #[test]
    fn d13p2_test() {
        assert_eq!(
            1068781,
            exercise_2(&vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19)
            ])
        );
        assert_eq!(3417, exercise_2(&vec![Some(17), None, Some(13), Some(19)]));
        // assert_eq!(754018, exercise_2(&vec![67, 7, 59, 61]));
    }

    //     #[bench]
    //     fn d13_bench_parse(b: &mut Bencher) {
    //         b.iter(|| read_input(include_str!("input/day13.txt")));
    //     }
    //     #[bench]
    //     fn d13_bench_ex1(b: &mut Bencher) {
    //         let input = read_input(include_str!("input/day13.txt"));
    //         b.iter(|| exercise_1(&input));
    //     }

    //     #[bench]
    //     fn d13_bench_ex2(b: &mut Bencher) {
    //         let input = read_input(include_str!("input/day13.txt"));
    //         b.iter(|| exercise_2(&input));
    //     }
}
