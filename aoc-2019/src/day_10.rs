use crate::test::Bencher;
use itertools::Itertools;
use utils::Grid;

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day10.txt"));
    let (result, pos) = exercise_1(input.clone());
    println!("ex1: {}", result);
    println!(
        "ex2: {}",
        exercise_2(input.clone(), pos.0 as i32, pos.1 as i32)
    );
}

fn gcd(mut a: i32, mut b: i32) -> Option<i32> {
    if a == 0 || b == 0 {
        return None;
    }
    a = a.abs();
    b = b.abs();
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    Some(a)
}

fn gcd_2(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();
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

fn exercise_1(grid: Grid<bool>) -> (u32, (usize, usize)) {
    let mut max_counter = 0;
    let mut max_node = (0, 0);
    let w = grid.width as i32;
    let h = grid.height as i32;

    let asteroids = grid
        .to_vec()
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c { Some((x as i32, y as i32)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (x, y) in &asteroids {
        let mut counter = 0;

        for (tx, ty) in &asteroids {
            let dx = tx - *x;
            let dy = ty - *y;
            let gcd = gcd_2(dx, dy);
            if gcd > 0 {
                let dx = dx / gcd;
                let dy = dy / gcd;
                if let Some(c) = ray(&grid, (*x as usize, *y as usize), (dx, dy)) {
                    if c == (*tx as usize, *ty as usize) {
                        counter += 1;
                    }
                }
            }
        }

        if counter > max_counter {
            max_counter = counter;
            max_node = (*x as usize, *y as usize);
        }
    }
    (max_counter, max_node)
}

fn exercise_2(grid: Grid<bool>, x: i32, y: i32) -> usize {
    // collect asteroids and their degrees
    let mut asteroids = grid
        .to_vec()
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .map(|(tx, ty)| {
            let dx = tx as i32 - x;
            let dy = ty as i32 - y;

            let rotation = (dy as f64).atan2(dx as f64) + std::f64::consts::PI;

            let degrees = (rotation / (2f64 * std::f64::consts::PI) * 3600000f64) as i32;
            let degrees = (degrees + 2700000) % 3600000;

            ((degrees, dx.abs() + dy.abs()), (tx, ty))
        })
        .collect::<Vec<_>>();
    asteroids.sort_by_key(|a| a.0);

    let mut map = hashbrown::HashMap::new();
    for (key, group) in &asteroids.into_iter().group_by(|((deg, _), _)| *deg) {
        let mut c = group.map(|x| x.1).collect::<Vec<_>>();
        c.reverse();
        map.insert(key, c);
    }
    let mut counter = 0;
    while map.len() > 0 {
        let mut keys = map.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            let stack = map.get_mut(&key).unwrap();
            let f = stack.pop().unwrap();
            counter += 1;
            if stack.len() == 0 {
                map.remove(&key);
            }

            if counter == 200 {
                return f.0 * 100 + f.1;
            }
        }
    }
    0
}

fn ray(grid: &Grid<bool>, pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
    let (mut x, mut y) = (pos.0 as i32, pos.1 as i32);
    let w = grid.width as i32;
    let h = grid.height as i32;

    x += dir.0;
    y += dir.1;

    while x >= 0 && x < w && y >= 0 && y < h {
        if grid.get(x as usize, y as usize) == Some(&true) {
            return Some((x as usize, y as usize));
        }
        x += dir.0;
        y += dir.1;
    }

    None
}

fn read_input<'a>(input: &'a str) -> Grid<bool> {
    Grid::from_vec(
        input
            .lines()
            .map(|x| x.chars().map(|a| a == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

#[test]
fn d10_test() {
    let input = read_input(
        r".#..#
.....
#####
....#
...##",
    );
    assert_eq!(exercise_1(input), (8, (3, 4)));
    let input = read_input(include_str!("input/day10_test_1.txt"));
    assert_eq!(exercise_1(input).0, 33);

    let input = read_input(include_str!("input/day10_test_2.txt"));
    assert_eq!(exercise_1(input).0, 35);

    let input = read_input(include_str!("input/day10_test_3.txt"));
    assert_eq!(exercise_1(input).0, 41);

    let input = read_input(include_str!("input/day10_test_4.txt"));
    assert_eq!(exercise_1(input).0, 210);
    let input = read_input(include_str!("input/day10_test_4.txt"));
    assert_eq!(exercise_2(input, 11, 13), 802);
}

#[bench]
fn d10_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day10.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d10_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day10.txt"));
    let (result, pos) = exercise_1(input.clone());
    b.iter(|| exercise_2(input.clone(), pos.0 as i32, pos.1 as i32));
}

#[bench]
fn d10_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day10.txt")));
}
