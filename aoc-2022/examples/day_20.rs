type InputType = Vec<isize>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/test.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> isize {
    line.parse().unwrap()
}

fn next_pos(pos: isize, len: usize) -> usize {
    pos.rem_euclid(len as isize) as usize
}
fn exercise_1(input: InputType) -> isize {
    let mut changed = input.into_iter().enumerate().collect::<Vec<_>>();

    for x in 0..changed.len() {
        let pos = changed.iter().position(|(a, _)| *a == x).unwrap();
        let (_, val) = changed[pos];

        changed.remove(pos);
        changed.insert(next_pos(pos as isize + val, changed.len()), (x, val));
    }

    let npos = changed.iter().position(|(_, a)| *a == 0).unwrap();

    let a = changed.iter().cycle().skip(npos).nth(1000).unwrap();
    let b = changed.iter().cycle().skip(npos).nth(2000).unwrap();
    let c = changed.iter().cycle().skip(npos).nth(3000).unwrap();

    a.1 + b.1 + c.1
}

fn exercise_2(input: InputType) -> isize {
    const DECRYPT_KEY: isize = 811589153;
    let mut changed = input
        .into_iter()
        .map(|x| x * DECRYPT_KEY)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..10 {
        for x in 0..changed.len() {
            let pos = changed.iter().position(|(a, _)| *a == x).unwrap();
            let (_, val) = changed[pos];

            changed.remove(pos);
            changed.insert(next_pos(pos as isize + val, changed.len()), (x, val));
        }
    }

    let npos = changed.iter().position(|(_, a)| *a == 0).unwrap();

    let a = changed.iter().cycle().skip(npos).nth(1000).unwrap();
    let b = changed.iter().cycle().skip(npos).nth(2000).unwrap();
    let c = changed.iter().cycle().skip(npos).nth(3000).unwrap();

    a.1 + b.1 + c.1
}
