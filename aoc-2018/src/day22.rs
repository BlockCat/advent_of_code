use std::ops::Add;
use std::ops::Index;

pub fn execute_exercises() {
    println!("Risk index: {}", exercise_1(4845, (6, 770))); // 4677 too low
    println!("Shortest minutes: {}", exercise_2(4845, (6, 770), (1000, 1000)));
}

type Location = (usize, usize);

#[derive(Clone)]
enum Soil {
    Rocky, Wet, Narrow
}

impl Soil {
    fn does_support(&self, tool: &Tool) -> bool {
        match self {
            Soil::Rocky => tool == &Tool::Gear || tool == &Tool::Torch,
            Soil::Wet => tool == &Tool::Gear || tool == &Tool::Neither,
            Soil::Narrow => tool == &Tool::Torch || tool == &Tool::Neither
        }
    }


}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
enum Tool {
    Gear = 0, Torch = 1, Neither = 2
}

impl From<Soil> for u64 {

    fn from(soil: Soil) -> u64 {
        match soil {
            Soil::Rocky => 0, 
            Soil::Wet => 1,
            Soil::Narrow => 2, 
        }
    }
}

impl std::fmt::Display for Soil {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        let written = match self {
            Soil::Rocky => '.', 
            Soil::Wet => '=',
            Soil::Narrow => '|', 
        };
        write!(f, "{}", written)
    }
}

fn exercise_1(depth: u32, location: Location) -> u64 {
    let mut map = vec!(vec!((Soil::Rocky, 0); location.0 + 1); location.1 + 1);

    map[0][0] = (match depth % 3 {
            0 => Soil::Rocky,
            1 => Soil::Wet,
            2 => Soil::Narrow,
            _ => unreachable!()
        } ,depth);

    let map = fill_x(depth, map);
    let map = fill_y(depth, map);
    let map = fill_all(location, depth, map);

    //pretty_print(&map);

    map.into_iter().flat_map(|s| s.into_iter()).map(|(s, e)| u64::from(s)).sum()
}

fn exercise_2(depth: u32, target: Location, (width, height): (usize, usize)) -> u32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    //let width =  (target.0 + 1) * 2;
    //let height = (target.1 + 1) * 2;

    let mut map =       vec!(vec!((Soil::Rocky, 0); width); height);
    let mut visited =   vec!(vec!([std::u32::MAX - 7; 3]; width); height);

    map[0][0] = (match depth % 3 {
            0 => Soil::Rocky,
            1 => Soil::Wet,
            2 => Soil::Narrow,
            _ => unreachable!()
        } ,depth);

    let map = fill_x(depth, map);
    let map = fill_y(depth, map);
    let map = fill_all(target, depth, map);

    let mut heap: BinaryHeap<(Reverse<u32>, Location, Tool)> = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0), Tool::Torch));    

    while !heap.is_empty() {
        let (Reverse(minutes), location, tool) = heap.pop().unwrap();

        if location == target && tool == Tool::Torch {            
            return minutes;
        } else if location == target {
            heap.push((Reverse(minutes + 7), location, Tool::Torch));
            continue;
        }        
    
        // If my current location has a shorter distance to this point with my tool....
        let shortest_minutes = visited[location.1][location.0][tool.clone() as usize];
        if shortest_minutes <= minutes {
            continue;
        }

        // If my current location has a shorter distance to this point with another tool (but change to my tool)
        let shortest_minutes = visited[location.1][location.0].iter().min().unwrap();
        if shortest_minutes + 7 <= minutes {
            continue;
        }       

        visited[location.1][location.0][tool.clone() as usize] = minutes;
        
        // Check north
        let possible_directions = vec![Direction::N, Direction::E, Direction::S, Direction::W].into_iter().filter(|d| {
            match d {
                Direction::N => location.1 > 0,
                Direction::E => location.0 < map[0].len() - 1,
                Direction::S => location.1 < map.len() - 1,                
                Direction::W => location.0 > 0,                
            }
        });

        for dir in possible_directions {            
            let next_loc = location + dir;

            // Check if I can go to the next location with my current tool
            // This might need to change though
            if map[next_loc.1][next_loc.0].0.does_support(&tool) {                
                heap.push((Reverse(minutes + 1), next_loc, tool.clone()));
            } else {                
                let possible_tool = match (&map[location.1][location.0].0, &map[next_loc.1][next_loc.0].0) {
                    (Soil::Rocky, Soil::Wet) => Tool::Gear,
                    (Soil::Rocky, Soil::Narrow) => Tool::Torch,
                    (Soil::Wet, Soil::Narrow) => Tool::Neither,
                    (Soil::Wet, Soil::Rocky) => Tool::Gear,
                    (Soil::Narrow, Soil::Rocky) => Tool::Torch,
                    (Soil::Narrow, Soil::Wet) => Tool::Neither,
                    _ => unreachable!()
                };
                heap.push((Reverse(minutes + 8), next_loc, possible_tool));
            }
        }
    }

    unreachable!()
}

fn fill_x(depth: u32, mut map: Vec<Vec<(Soil, u32)>>) -> Vec<Vec<(Soil, u32)>> {

    let mut erosion_level = depth;
    
    for x in 1..map[0].len() {
        erosion_level += 16807; 
        while erosion_level > 20183 {
            erosion_level -= 20183;
        }             
        let soil = match erosion_level % 3 {
            0 => Soil::Rocky,
            1 => Soil::Wet,
            2 => Soil::Narrow,
            _ => unreachable!()
        };

        map[0][x] = (soil, erosion_level);
    }

    map
}

fn fill_y(depth: u32, mut map: Vec<Vec<(Soil, u32)>>) -> Vec<Vec<(Soil, u32)>> {

    let mut erosion_level = depth;
    
    for y in 1..map.len() {
        erosion_level += 48271;   
        while erosion_level > 20183 {
            erosion_level -= 20183;
        }        
        let soil = match erosion_level % 3 {
            0 => Soil::Rocky,
            1 => Soil::Wet,
            2 => Soil::Narrow,
            _ => unreachable!()
        };

        map[y][0] = (soil, erosion_level);
    }

    map
}

fn fill_all((width, height): Location, depth: u32, mut map: Vec<Vec<(Soil, u32)>>) -> Vec<Vec<(Soil, u32)>> {

    for y in 1..map.len() {
        for x in 1..map[0].len() {

            if x == width && y == height {
                map[height][width] = (match depth % 3 {
                    0 => Soil::Rocky,
                    1 => Soil::Wet,
                    2 => Soil::Narrow,
                    _ => unreachable!()
                }, depth);
                continue;
            }          
            let geologic_index= map[y][x-1].1 as u128 * map[y-1][x].1 as u128;
            let erosion_level = (geologic_index + depth as u128) % 20183;            
            
            let soil = match erosion_level % 3 {
                0 => Soil::Rocky,
                1 => Soil::Wet,
                2 => Soil::Narrow,
                _ => unreachable!()
            };

            map[y][x] = (soil, erosion_level as u32);
        }
    }

    

    map
}

fn pretty_print(map: &Vec<Vec<(Soil, u32)>>) {

    for s in map {
        for (soil, _) in s {
            print!("{}", soil);
        }

        println!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;


    #[test]
    fn day22_ex1_s1() {
        let result = exercise_1(510, (10, 10));

        assert_eq!(result, 114);
    }

    #[test]
    fn day22_ex1_s2() {
        let result = exercise_1(4845, (6, 770));
        assert_eq!(result, 5400);
    }

    #[test]
    fn day22_ex2_s1() {
        let result = exercise_2(510, (10, 10), (20, 20));
        assert_eq!(result, 45);
    }

    #[test]
    fn day22_ex2_s2() {
        let result = exercise_2(4845, (6, 770), (1000, 1000));
        assert_eq!(result, 1048);
    }

    #[bench]
    fn day22_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(4845, (6, 770)));
    }

    #[bench]
    fn day22_bench_ex2(b: &mut Bencher) {
        b.iter(|| exercise_2(4845, (6, 770), (1000, 1000)));
    }

}