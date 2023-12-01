pub fn main() {
    let numbers = include_str!("../input/day02.txt")
        .lines()
        .map(|x| to_direction(x))
        .collect::<Vec<_>>();

    let result = numbers
        .iter()
        .cloned()
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
        .map(|(horizontal, depth)| horizontal * depth)
        .unwrap();

    println!("ex1: {:?}", result);

    let (hor, depth, _) = numbers
        .iter()
        .fold((0, 0, 0), |(hor, depth, aim), (forward, db)| {
            (hor + forward, depth + aim * forward, aim + db)
        });

    println!("ex2: {}", hor * depth);
}

// (x, depth)
fn to_direction(dir: &str) -> (isize, isize) {
    let mut split = dir.split(' ');
    let dir = split.next();
    let len = split.next().unwrap().parse().unwrap();
    match dir {
        Some("forward") => (len, 0),
        Some("down") => (0, len),
        Some("up") => (0, -len),
        _ => unreachable!(),
    }
}
