use std::ops::Add;
use hashbrown::HashMap;
use std::collections::VecDeque;

pub fn execute_exercises() {
    let samples = parse_input(include_str!("../input/day20_in.txt"));
    //println!("{:?}", samples);
    let result = exercise_1(samples);
    println!("Distance: {}", result.0);
    println!("more than 1000: {}", result.1);
    
}

type Location = (isize, isize);

#[derive(Debug, Clone)]
enum Direction {
    N, E, S, W
}

impl Direction {
    fn turn_around(&self) -> Direction {
        match &self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

impl From<&Direction> for u8 {
    fn from(f: &Direction) -> u8 {
        match f {
            Direction::N => 1,
            Direction::E => 2,
            Direction::S => 4,
            Direction::W => 8,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            'W' => Direction::W,
            _ => panic!("Invalid character")
        }
    }
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Location {
        let (x, y) = self;
        match other {
            Direction::N => (x, y - 1),
            Direction::S => (x, y + 1),
            Direction::E => (x + 1, y),
            Direction::W => (x - 1 , y)
        }
    }
}

fn parse_input(input: &str) -> HashMap<Location, u8> {
    let mut stack = VecDeque::new();
    let mut loc = (0isize, 0isize);
    let mut map = HashMap::new();
    stack.push_front(loc);

    for character in input.chars() {
        match character {
            'N'| 'E' | 'S' | 'W' => {
                let direction = Direction::from(character);
                let flag = u8::from(&direction);
                // uncomment for visualization
                //let other_flag = u8::from(&direction.turn_around());
                *map.entry(loc).or_insert(0u8) |= flag;
                loc = loc + direction;
                //*map.entry(loc).or_insert(0u8) |= other_flag;
            }
            '(' => {
                stack.push_front(loc);
            }
            '|' => {
                loc = *stack.front().unwrap();
            }
            ')' => {
                loc = stack.pop_front().unwrap();
            }
            _ => unreachable!()
        }
    }

    map
}

fn exercise_1(input: HashMap<Location, u8>) -> (usize, usize) {
    
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    queue.push_front((0, (0, 0)));

    while !queue.is_empty() {
        let (dist, loc) = queue.pop_front().unwrap();

        let odist = distances.entry(loc).or_insert(std::usize::MAX);

        if dist < *odist {
            *odist = dist;        

            if let Some(directions) = input.get(&loc) {
                for direction in vec![Direction::N, Direction::E, Direction::S, Direction::W] {                
                    if *directions & u8::from(&direction) > 0 {
                        queue.push_back((dist + 1, loc + direction));
                    }
                }
            }
        }
    }

    (distances.values().cloned().max().unwrap(), 
    distances.values().cloned().filter(|&v| v >= 1000).count())
}

fn pretty_print(map: &HashMap<Location, u8>) {
    let minx = map.iter().map(|(l, _)| l.0).min().unwrap() as isize;
    let maxx = map.iter().map(|(l, _)| l.0).max().unwrap() as isize;
    let miny = map.iter().map(|(l, _)| l.1).min().unwrap() as isize;
    let maxy = map.iter().map(|(l, _)| l.1).max().unwrap() as isize;

    let s: String = (miny..=maxy).map(|y| {
        (minx..=maxx).map(move |x| {
            if (x, y) == (0, 0) {
                'x'
            } else {
                match map.get(&(x, y)) {
                    Some(0) => ' ', //0000
                    Some(1) => '╵', //0001
                    Some(2) => '╶',//0010
                    Some(3) => '└',//0011
                    Some(4) => '╷',//0100
                    Some(5) => '│',//0101
                    Some(6) => '┌',//0110
                    Some(7) => '├',//0111
                    Some(8) => '╴',//1000
                    Some(9) => '┘',//1001
                    Some(10) => '─', //1010
                    Some(11) => '┴', //1011
                    Some(12) => '┐', //1100
                    Some(13) => '┤', //1101
                    Some(14) => '┬', //1110
                    Some(15) => '┼',
                    None => ' ',
                    _ => {
                        println!("error: {:?}", map.get(&(x, y)));
                        unreachable!()
                    }
                }
            }
        }).chain(vec!('\n'))
    }).flatten().collect();

    println!("{}", s);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day20_ex1_s1() {
       let input = r"ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN";
        let input = parse_input(input);        
        let result = exercise_1(input);
        assert_eq!(result, (18, 0));
    }

    #[test]
    fn day20_ex1_s2() {
       let input = r"ENWWW(NEEE|SSE(EE|N))";
        let input = parse_input(input);
        pretty_print(&input);
        let result = exercise_1(input);
        assert_eq!(result, (10, 0));
    }

    #[test]
    fn day20_ex1_s3() {
       let input = r"WNE";
        let input = parse_input(input);        
        let result = exercise_1(input);
        assert_eq!(result, (3, 0));
    }

     #[test]
    fn day20_ex1_s4() {
       let input = r"ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))";
        let input = parse_input(input);
        pretty_print(&input);
        let result = exercise_1(input);
        assert_eq!(result, (23, 0));
    }

     #[test]
    fn day20_ex1_s5() {
       let input = r"WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))";
        let input = parse_input(input);
        pretty_print(&input);
        let result = exercise_1(input);
        assert_eq!(result, (31, 0));
    }

    #[test]
    fn day20_ex1_s6() {
        let samples = parse_input(include_str!("../input/day20_in.txt"));
        //pretty_print(&samples);
        let result = exercise_1(samples);
        assert_eq!(result, (4186, 8466));
    }

    #[bench]
    fn day20_bench_ex12(b: &mut Bencher) {
        let samples = parse_input(include_str!("../input/day20_in.txt"));
        //pretty_print(&samples);
        b.iter(move || exercise_1(samples.clone()));
    }

    #[bench]
    fn day20_bench_read(b: &mut Bencher) {                
        b.iter(move || parse_input(include_str!("../input/day20_in.txt")));
    }

}