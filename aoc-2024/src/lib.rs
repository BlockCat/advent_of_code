use std::str::Lines;

pub mod bitset;
pub mod direction;
pub mod grid;
pub mod vector;

pub fn stopwatch<F>(action: F) -> Result<std::time::Duration, std::time::SystemTimeError>
where
    F: FnOnce(),
{
    use std::time;
    let start = time::SystemTime::now();
    action();
    let end = time::SystemTime::now();

    end.duration_since(start)
}

pub fn grouped_lines<'a>(r: &'a str) -> impl Iterator<Item = Lines<'a>> {
    r.split("\n\n").map(|group| group.lines())
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
