pub fn main() {
    let numbers = include_str!("../input/day03.txt")
        .lines()
        .map(decode_binary)
        .collect::<Vec<_>>();

    println!("ex1: {}", exercise_1(&numbers));
    println!("ex2: {}", exercise_2(&numbers));
}

fn decode_binary(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| match c {
            '1' => 1,
            '0' => 0,
            _ => unreachable!(),
        })
        .collect()
}

fn exercise_1(numbers: &[Vec<u8>]) -> usize {
    let commons = vec![0usize; numbers[0].len()];

    let m = numbers
        .iter()
        .fold(commons, |acc, x| {
            acc.into_iter()
                .zip(x)
                .map(|x| (x.0 + *x.1 as usize))
                .collect()
        })
        .into_iter()
        .map(|f| {
            (
                u32::from(f > numbers.len() / 2),
                u32::from(f <= numbers.len() / 2),
            )
        })
        .collect::<Vec<_>>();

    let a: String = m.iter().map(|a| a.0.to_string()).collect();
    let b: String = m.iter().map(|a| a.1.to_string()).collect();

    let a = usize::from_str_radix(&a, 2).unwrap();
    let b = usize::from_str_radix(&b, 2).unwrap();

    a * b
}

fn exercise_2(numbers: &[Vec<u8>]) -> usize {
    let a = oxy(numbers.iter().collect::<Vec<_>>());
    let b = co2(numbers.iter().collect::<Vec<_>>());

    println!("{} * {}", a, b);

    a * b
}

fn oxy(mut numbers: Vec<&Vec<u8>>) -> usize {
    let mut counter = 0;
    while numbers.len() > 1 {
        let commons = count_most(&numbers, counter);
        let search = if commons.0 > commons.1 { 0u8 } else { 1u8 };
        numbers = numbers
            .into_iter()
            .filter(|&a| a[counter] == search)
            .collect();
        counter += 1;
    }

    assert!(numbers.len() == 1);

    let a: String = numbers[0].iter().map(|x| x.to_string()).collect();

    usize::from_str_radix(&a, 2).unwrap()
}

fn co2(mut numbers: Vec<&Vec<u8>>) -> usize {
    let mut counter = 0;
    while numbers.len() > 1 {
        let commons = count_most(&numbers, counter);
        let search = if commons.0 > commons.1 { 1u8 } else { 0u8 };
        numbers = numbers
            .into_iter()
            .filter(|&a| a[counter] == search)
            .collect();
        counter += 1;
    }

    assert!(numbers.len() == 1);

    let a: String = numbers[0].iter().map(|x| x.to_string()).collect();

    usize::from_str_radix(&a, 2).unwrap()
}

fn count_most(numbers: &[&Vec<u8>], index: usize) -> (usize, usize) {
    numbers
        .iter()
        .fold((0usize, 0usize), |(a, b), x| match x[index] {
            0 => (a + 1, b),
            1 => (a, b + 1),
            _ => unreachable!(),
        })
}
