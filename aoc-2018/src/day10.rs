use std::cmp;

pub fn execute_exercises() {
    exercise_1(read_input());
}

fn read_input() -> Vec<(i32, i32, i32, i32)> {
    parse_input(include_str!("../input/day10_in.txt"))
}

fn parse_input(input: &'static str) -> Vec<(i32, i32, i32, i32)> {
    input.lines().map(|l| {
        let c: Vec<i32> = l.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        c.iter();
        (c[0], c[1], c[2], c[3])
    }).collect()
}

fn exercise_1(mut input: Vec<(i32, i32, i32, i32)>) {
    let mut prev_h = std::i32::MAX;
    for counter in 1.. {
        let (up, down) = input.iter_mut()            
            .fold((std::i32::MAX, std::i32::MIN), |(min, max), (x, y, dx, dy)| {
                *x += *dx;
                *y += *dy;
                (cmp::min(min, *y), cmp::max(max, *y))
            });

        if down-up < 10 {
            print(&input);
            println!("seconds: {}", counter);
        }

        if (down - up) >= prev_h {
            break;
        } else {
            prev_h = down - up;
        }
    }    
}

fn print(input: &[(i32, i32, i32, i32)]) {
    use hashbrown::HashSet;
    let (mut up, mut down) = (std::i32::MAX, std::i32::MIN);
    let (mut left, mut right) = (std::i32::MAX, std::i32::MIN);
    
    let c: HashSet<(i32, i32)> = input.iter()
        .map(|(x, y, _, _)| {
            left = cmp::min(left, *x);
            right = cmp::max(right, *x);            
            up = cmp::min(up, *y);
            down = cmp::max(down, *y);  

            (*x, *y)
        }).collect();

    for y in up-4..=down+4 {
        for x in left-4..=right+4 {
            if c.contains(&(x, y)) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }
        
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d10_ex1_s1() {
        let input = r"9 1 0 2
7 0 -1 0
3 -2 -1 1
6 10 -2 -1
2 -4 2 2
-6 10 2 -2
1 8 1 -1
1 7 1 0
-3 11 1 -2
7 6 -1 -1
-2 3 1 0
-4 3 2 0
10 -3 -1 1
5 11 1 -2
4 7 0 -1
8 -2 0 1
15 0 -2 0
1 6 1 0
8 9 0 -1
3 3 -1 1
0 5 0 -1
-2 2 2 0
5 -2 1 2
1 4 2 1
-2 7 2 -2
3 6 -1 -1
5 0 1 0
-6 0 2 0
5 9 1 -2
14 7 -2 0
-3 6 2 -1";

        exercise_1(parse_input(input));
    }

    #[bench]
    fn d10_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(read_input()));
    }
}