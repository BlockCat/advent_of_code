pub fn main() {
    let numbers: Vec<isize> = include_str!("../input/day01.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    println!("{}", exercise::<2>(&numbers));
    println!("{}", exercise::<4>(&numbers));
}

// a + (b + c) > (b + c) + d => a > d
fn exercise<const N: usize>(count: &[isize]) -> usize {
    count
        .windows(N)
        .filter(|window| window[N - 1] > window[0])
        .count()
}
