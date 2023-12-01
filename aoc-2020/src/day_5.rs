pub fn run() {
    let input = include_str!("input/day5.txt").trim();
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input, 7, 3));
}

fn exercise_1(input: &str) -> usize {
    input.lines().map(|line| {
            let mut it = line.chars();
            let row = get_number(7, &mut it, 'B');
            let col = get_number(3, &mut it, 'R');
            row * 8 + col
        })
        .fold(0, |max, x| max.max(x)) // this is faster than using .max().unwrap()?
}

fn get_number(length: usize, it: &mut impl Iterator<Item = char>, back: char) -> usize {
    it.take(length)
        .fold(0, |acc, x| (acc * 2) + (x == back) as usize)
}

fn exercise_2(input: &str, row_length: usize, col_length: usize) -> usize {
    let (min, max, sum) = input
        .lines()        
        .map(|line| {
            let mut it = line.chars();
            let row = get_number(row_length, &mut it, 'B');
            let col = get_number(col_length, &mut it, 'R');
            row * 8 + col
        })
        .fold((std::usize::MAX, 0usize, 0usize), |(min, max, sum), x| {
            (min.min(x), max.max(x), sum + x)
        });

    let expected_sum = (max + min) * (max - min + 1) / 2;

    // println!("Max seat: {}, free seat: {}", max, expected_sum - sum);
    return expected_sum - sum;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d5p1_test() {
        // let input = include_str!("input/day4.txt");
        // assert_eq!(254, exercise_1(&input));
        assert_eq!(44, get_number(7, &mut "FBFBBFFRLR".chars(), 'B'));
        assert_eq!(5, get_number(3, &mut "RLR".chars(), 'R'));
    }

    // #[test]
    // fn d5p2_test() {
    //     let input = include_str!("input/day4.txt");
    //     assert_eq!(184, exercise_2(&input));
    // }

    #[bench]
    fn d5_bench_ex1(b: &mut Bencher) {
        let input = include_str!("input/day5.txt");
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d5_bench_ex2(b: &mut Bencher) {
        let input = include_str!("input/day5.txt");
        b.iter(|| exercise_2(&input, 7, 3));
    }

    // #[bench]
    // fn d5_bench_ex1bigboi(b: &mut Bencher) {
    //     let input = include_str!("input/day4bigboi.txt");
    //     b.iter(|| exercise_1(&input));
    // }

    #[bench]
    fn d5_bench_ex2bigboi(b: &mut Bencher) {
        let input = include_str!("input/day5bigboi.txt");
        b.iter(|| exercise_2(&input, 18, 7));
    }
}
