use hashbrown::HashSet;
use hashbrown::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::ops::Add;

type Location = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    North, East, South, West
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, other: Direction) -> Location {
        let (x, y) = self;
        match other {
            Direction::North => (x, y.saturating_sub(1)),
            Direction::South => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::West => (x.saturating_sub(1), y)
        }
    }
}

pub fn execute_exercises() {
    let (mapping, min_y, max_y) = parse_input(include_str!("../input/day17_in.txt"));
    println!("Amount: {}", exercise_1(mapping.clone(), (min_y, max_y)));
}

fn parse_input(input: &str) -> (HashSet<Location>, usize, usize) {
    let mut map = HashSet::new();
    let (mut min_y, mut max_y) = (std::usize::MAX, std::usize::MIN);

    for line in input.lines() {
        let mut line = line.split(", ");
        let a = line.next().unwrap();
        let b = line.next().unwrap();

        let first: usize = a[2..].parse().unwrap();
        let range: Vec<_> = b[2..].split("..").map(|s| s.parse().unwrap()).collect();        

        if &a[0..1] == "x" {
            min_y = std::cmp::min(min_y, range[0]);
            max_y = std::cmp::max(max_y, range[1]);
            for i in range[0]..=range[1] {
                map.insert((first, i));
            }
        } else {
            min_y = std::cmp::min(min_y, first);
            max_y = std::cmp::max(max_y, first);
            for i in range[0]..=range[1] {
                map.insert((i, first));
            }
        }
    }
    (map, min_y, max_y)
}

//what about this then
// store points that can go down (can hit water)
// loop{
// take point 
// Go down till hit
// scan left and right to see if we can go down further, 
// if yes: store point and set water flowing. end
// if no: set static and go one up

// end once
//}

// probably A*
// Whenever we get to a point, we want to find the shortest distance to out of y_bound.
#[derive(Debug)]
enum Water {
    Flowing, Static
}
fn search_direction(prev:Location, mut location: Location, direction: Direction, mapping: &HashSet<Location>, queue: &mut VecDeque<(Location, Location)>, precedence: &mut HashMap<Location, Location>, water: &mut HashMap<Location, Water>) -> (bool, Vec<Location>) {
    let mut waters = Vec::new();

    while !mapping.contains(&(location + direction)) {
        location = location + direction;
        if let Some(Water::Static) = water.get(&location) {
            return (false, waters);
        }
        waters.push(location);
        //check if we should go down here
        let south = location + Direction::South;

        match water.get(&south) {
            Some(Water::Flowing) => return (true, waters),
            Some(Water::Static) => {}
            None => {
                if !mapping.contains(&south) {
                    
                    if !water.contains_key(&location) {
                        queue.push_back((location, prev));
                        precedence.insert(location, prev);
                    }
                    return (true, waters);
                }        
            }
        }
    }
    return (false, waters);
}

fn exercise_1(mapping: HashSet<Location>, bounds: (usize, usize)) -> usize {
    helper(mapping, bounds, false).0
}

fn exercise_2(mapping: HashSet<Location>, bounds: (usize, usize)) -> usize {
    helper(mapping, bounds, true).2
}

fn helper(mapping: HashSet<Location>, (min_y, max_y): (usize, usize), extra_filter: bool) -> (usize, usize, usize) {
    let mut water_mapping = HashMap::new();
    let mut queue = VecDeque::new();
    let mut precedence = HashMap::new();
    queue.push_front(((500, 0), (0, 0)));

    'a: while !queue.is_empty() {
        let (mut pos, prev) = queue.pop_front().unwrap();
        // Get position and search down        
        let mut search = pos;
        while !mapping.contains(&(search + Direction::South)) {            
            search = search + Direction::South;  

            if search.1 > max_y {                
                continue 'a;
            } else {
                water_mapping.insert(search, Water::Flowing);
            }
        }
        // Found bottom so expand
        // Search left.        
        while {            
            let (left_hole, waters_l) = search_direction(pos, search, Direction::West, &mapping, &mut queue, &mut precedence, &mut water_mapping);
            let (right_hole, waters_r) = search_direction(pos, search, Direction::East, &mapping, &mut queue, &mut precedence, &mut water_mapping);

            if left_hole || right_hole {
                water_mapping.extend(waters_l.into_iter().map(|loc| (loc, Water::Flowing)));
                water_mapping.extend(waters_r.into_iter().map(|loc| (loc, Water::Flowing)));
            } else {
                water_mapping.extend(waters_l.into_iter().map(|loc| (loc, Water::Static)));
                water_mapping.extend(waters_r.into_iter().map(|loc| (loc, Water::Static)));
            }            

            if !left_hole && !right_hole {
                if let Some(e) = water_mapping.get_mut(&search) {
                    *e = Water::Static;
                } else {
                    water_mapping.insert(search, Water::Static);                    
                }
                search = search + Direction::North; 
                                  

                if search.1 < pos.1 {
                    // Yo in this situation ehm, take the one where this guy is coming from.                    

                    if pos.1 == prev.1 { 
                        // Get the previous one of the previous one.
                        pos = precedence[&prev];
                        search.0 = pos.0;                        
                    } else {
                        pos = prev;
                        search.0 = prev.0;
                    }



                    while prev.1 == search.1 {
                        
                        pos = precedence[&pos];
                        search.0 = pos.0;
                    }
                    true
                } else {
                    true
                }
            } else {
                false
            }
            
        }{}        
        //pretty_print(&mapping, &water_mapping, (min_y, max_y));

    }

    //pretty_print(&mapping, &water_mapping, (min_y, max_y));
    
    let flowing = water_mapping.iter()        
        .filter(|((_, y), state)| {
        *y >= min_y && *y <= max_y &&
        if let Water::Flowing = *state {
            true
        } else {
            false
        }
    }).count();

    let staticf = water_mapping.iter()        
        .filter(|((_, y), state)| {
        *y >= min_y && *y <= max_y &&
        if let Water::Static = *state {
            true
        } else {
            false
        }
    }).count();

    (flowing + staticf, flowing, staticf)
}

fn pretty_print(mapping: &HashSet<Location>, water: &HashMap<Location, Water>, (min_y, max_y): (usize, usize)) {
    let min_x = mapping.iter().map(|s| s.0).min().unwrap();
    let max_x = mapping.iter().map(|s| s.0).max().unwrap();

    //let max_y = water.iter().map(|(k, v)| k.1).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x-1..=max_x+1 {
            let pos = (x, y);
            if mapping.contains(&pos) {
                print!("#");
            } else {
                match water.get(&pos) {
                    Some(Water::Flowing) => print!("|"),
                    Some(Water::Static)  => print!("~"),
                    None                 => print!(".")
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day17_ex1_s1() {
        let input = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let (mapping, min_y, max_y) = parse_input(input);
        assert_eq!(exercise_1(mapping, (min_y, max_y)), 57);
    }

    #[test]
    fn day17_ex1_s2() {
        let input = r"x=495, y=3..7
y=7, x=495..501
x=501, y=3..7
x=506, y=1..2
x=498, y=9..13
x=505, y=9..13
y=13, x=498..505
x=501, y=10..11
x=503, y=10..11
y=11, x=501..503
";
        let (mapping, min_y, max_y) = parse_input(input);
        //exercise_1(mapping, (min_y, max_y));
        assert_eq!(exercise_1(mapping, (min_y, max_y)), 85);
    }

    #[test]
    fn day17_ex1_s3() {
        let input = r"x=495, y=3..7
y=7, x=495..501
x=501, y=3..7
x=506, y=1..2
x=498, y=9..13
x=505, y=9..13
y=13, x=498..505
x=500, y=10..11
x=502, y=10..11
y=11, x=500..502
";
        let (mapping, min_y, max_y) = parse_input(input);
        // exercise_1(mapping, (min_y, max_y));
        assert_eq!(exercise_1(mapping, (min_y, max_y)), 85);
    }

    #[test]
    fn day17_ex1_s4() {
        let input = r"x=495, y=3..7
y=7, x=495..501
x=501, y=3..7
x=506, y=1..2
x=496, y=9..13
x=505, y=9..13
y=13, x=496..505
x=499, y=10..11
x=501, y=10..11
y=11, x=499..501
";
        let (mapping, min_y, max_y) = parse_input(input);
        //exercise_1(mapping, (min_y, max_y));
        assert_eq!(exercise_1(mapping, (min_y, max_y)) ,95);
    }

    #[test]
    fn day17_ex1_s5() {
        let (mapping, min_y, max_y) = parse_input(include_str!("../input/day17_in.txt"));
        assert_eq!(exercise_1(mapping.clone(), (min_y, max_y)), 28246);
        //exercise_1(mapping.clone(), (min_y, max_y)); //28246
    }

        #[test]
    fn day17_ex2_s1() {
        let input = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let (mapping, min_y, max_y) = parse_input(input);
        assert_eq!(exercise_2(mapping, (min_y, max_y)), 29);
    }

    #[test]
    fn day17_ex2_s5() {
        let (mapping, min_y, max_y) = parse_input(include_str!("../input/day17_in.txt"));
        assert_eq!(exercise_2(mapping.clone(), (min_y, max_y)), 23107);        
    }

    #[bench]
    fn day17_bench_read(b: &mut Bencher) {
        b.iter(|| parse_input(include_str!("../input/day17_in.txt")));
    }

    #[bench]
    fn day17_bench_ex1(b: &mut Bencher) {
        let (mapping, min_y, max_y) = parse_input(include_str!("../input/day17_in.txt"));
        b.iter(|| exercise_1(mapping.clone(), (min_y, max_y)));        
    }
    #[bench]
    fn day17_bench_ex2(b: &mut Bencher) {
        let (mapping, min_y, max_y) = parse_input(include_str!("../input/day17_in.txt"));
        b.iter(|| exercise_2(mapping.clone(), (min_y, max_y)));        
    }


}