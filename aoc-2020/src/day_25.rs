
// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day25.txt"));
    println!("{}", exercise_1(&input));
    // println!("{}", exercise_2(&input));
}

type Input = Vec<usize>;
fn read_input(input: &str) -> Input {}

fn exercise_1(input: &Input) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d18ex1() {
        let input = read_input(include_str!("input/day25test.txt"));
        assert_eq!(2, exercise_1(&input));
        // assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input(include_str!("input/day25test.txt"));
        assert_eq!(12, exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day25.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day25.txt"));
        b.iter(|| exercise_1(&input));
    }
}
