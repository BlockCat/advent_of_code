// #[test]
pub fn run() {
    let jump = 3;
    let input = read_input(include_str!("input/day10.txt"), jump);
    println!("{:?}", input);
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input, jump));
}

pub fn read_input(input: &str, jump: usize) -> Vec<usize> {
    let mut lines = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    lines.push(0);
    lines.sort();
    lines.push(lines.last().unwrap() + jump);
    lines
}

fn exercise_1(slice: &Vec<usize>) -> usize {
    let (one, three) =
        slice
            .iter()
            .zip(slice.iter().skip(1))
            .fold((0, 0), |(one, three), (&a, &b)| {
                if b == a + 1 {
                    (one + 1, three)
                } else if b == a + 3 {
                    (one, three + 1)
                } else {
                    (one, three)
                }
            });

    println!("{} of 1 and {} of 3", one, three + 1);
    (one) * (three)
}

fn exercise_2(slice: &Vec<usize>, jump: usize) -> usize {
    let mut map = vec![0usize; slice.len()];

    map[0] = 1;
    map[1] = 1;

    for i in 2..slice.len() {
        let start_index = i.saturating_sub(jump);
        map[i] = (start_index..i)            
            .filter(|x| slice[i] - slice[*x] <= jump)
            .map(|x| map[x])
            .sum();
    }

    *map.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;
    #[test]
    fn d10p1_test() {
        let input = read_input(include_str!("input/day10.txt"), 3);
        assert_eq!(2170, exercise_1(&input));
    }

    #[test]
    fn d10p2_test() {
        let input = read_input(include_str!("input/day10.txt"), 3);
        assert_eq!(24803586664192, exercise_2(&input, 3));
    }

    #[bench]
    fn d10_bench_parse(b: &mut Bencher) {
        b.iter(|| read_input(include_str!("input/day1.txt"), 3));
    }
    #[bench]
    fn d10_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day10.txt"), 3);
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d10_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day10.txt"), 3);
        b.iter(|| exercise_2(&input, 3));
    }

    // #[bench]
    // fn d1_bench_ex1bigboi(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1bigboi.txt"));
    //     b.iter(|| exercise_1(&input, 99920044));
    // }

    // #[bench]
    // fn d1_bench_ex2bigboi(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1bigboi.txt"));
    //     b.iter(|| exercise_2(&input, 99920044));
    // }

    // #[bench]
    // fn d1_bench_parsebigboi(b: &mut Bencher) {
    //     b.iter(|| read_input(include_str!("input/day1bigboi.txt")));
    // }

    // #[bench]
    // fn d1_bench_ex2(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1.txt"));
    //     b.iter(|| exercise_2(&input, 2020));
    // }

    // #[bench]
    // fn d1_bench_ex2b(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day1.txt"));
    //     b.iter(|| exercise_2b(&input, 2020));
    // }
}
