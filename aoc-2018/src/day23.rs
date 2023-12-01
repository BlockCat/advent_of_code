type Location = (i32, i32, i32);

#[derive(Clone)]
struct Nanobot {
    location: Location,
    radius: usize
}

pub fn execute_exercises() {
    let bots = parse_input(include_str!("../input/day23_in.txt"));
    println!("Bots in range of max: {}", exercise_1(bots.clone())); // 4677 too low
    println!("Shortest distance to most overlap: {}", exercise_2(bots.clone()));
}

fn parse_input(input: &str) -> Vec<Nanobot> {
    input.lines()
        .map(|line| {
            let mut split = line.split(' ');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            let z = split.next().unwrap().parse().unwrap();
            let r = split.next().unwrap().parse().unwrap();

            Nanobot {
                location: (x, y, z),
                radius: r
            }
        }).collect()
}

fn distance((x1, y1, z1): Location, (x2, y2, z2): Location) -> i32 {
    (x1 - x2).abs()
    + (y1 - y2).abs()
    + (z1 - z2).abs()
}

fn exercise_1(input: Vec<Nanobot>) -> usize {
    let radius_bot = input.iter().max_by_key(|s| s.radius).unwrap();

    input.iter()
        .filter(|b| {
            distance(b.location, radius_bot.location) as usize <= radius_bot.radius            
        }).count()
}

fn exercise_2(input: Vec<Nanobot>) -> i32 {
    
    // The region of a bot is like a cube, but weirdly oriented.
    // Map it to a inf plane with normal (1, 1, 1)
    // sweep   
    
    let mut events = input.iter().flat_map(|s| {
        let x = s.location.0 + s.location.1 + s.location.2;
        vec![(x - s.radius as i32, 1), (x + s.radius as i32 + 1, -1)].into_iter()
    }).collect::<Vec<_>>();
    events.sort();

    let t = events.into_iter().fold((0, 0, std::i32::MAX), |(current, max_start, range), (loc, x)| {
        if current + x == max_start {
            (current + x, max_start, std::cmp::min(range, loc.abs()))
        } else if current + x > max_start {
            (current + x, current + x, loc.abs())
        } else {
            (current + x, max_start, range)
        }
    });

    t.2    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;


    #[test]
    fn day23_ex1_s1() {
        let input = r"0 0 0 4
1 0 0 1
4 0 0 3
0 2 0 1
0 5 0 3
0 0 3 1
1 1 1 1
1 1 2 1
1 3 1 1";
        let input = parse_input(input);
        let result = exercise_1(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn day23_ex1_s2() {
        let input = parse_input(include_str!("../input/day23_in.txt"));        
        let result = exercise_1(input);
        assert_eq!(result, 219);
    }

    #[test]
    fn day23_ex2_s1() {
        let input = r"10 12 12 2
12 14 12 2
16 12 12 4
14 14 14 6
50 50 50 200
10 10 10 5";
        let input = parse_input(input);
        let result = exercise_2(input);
        assert_eq!(result, 36);
    }

    #[test]
    fn day23_ex2_s2() {
        let input = parse_input(include_str!("../input/day23_in.txt"));        
        let result = exercise_2(input);
        assert_eq!(result, 83779034);
    }

    #[bench]
    fn day23_bench_ex1(b: &mut Bencher) {
        let input = parse_input(include_str!("../input/day23_in.txt"));        
        b.iter(|| exercise_1(input.clone()));
    }

    #[bench]
    fn day23_bench_ex2(b: &mut Bencher) {
        let input = parse_input(include_str!("../input/day23_in.txt"));        
        b.iter(|| exercise_2(input.clone()));
    }
}