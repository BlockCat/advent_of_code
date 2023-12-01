use std::ops::Add;
use hashbrown::HashMap;
use hashbrown::HashSet;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy, Debug)]
enum Direction {
    NORTH, EAST, SOUTH, WEST
}

// (y, x)
type Position = (usize, usize);
type TrackMap = [[char; 152 ]; 152];

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, other: Direction) -> Position {
        let (y, x) = self;
        match other {
            Direction::NORTH => (y - 1, x),
            Direction::SOUTH => (y + 1, x),
            Direction::EAST => (y, x + 1),
            Direction::WEST => (y, x - 1)
        }
    }
}


impl Direction {
    fn next_direction(&self, ch: char) -> Self {
        match (ch, self) {
            ('/', &Direction::NORTH) => Direction::EAST,
            ('/', &Direction::EAST) => Direction::NORTH,
            ('/', &Direction::SOUTH) => Direction::WEST,
            ('/', &Direction::WEST) => Direction::SOUTH,
            ('\\', &Direction::NORTH) => Direction::WEST,
            ('\\', &Direction::EAST) => Direction::SOUTH,
            ('\\', &Direction::SOUTH) => Direction::EAST,
            ('\\', &Direction::WEST) => Direction::NORTH,            
            _ => *self
        }
    }

    fn left(&self) -> Self {
        match self {
            &Direction::NORTH => Direction::WEST,
            &Direction::EAST => Direction::NORTH,
            &Direction::SOUTH => Direction::EAST,
            &Direction::WEST => Direction::SOUTH,
        }
    }

    fn right(&self) -> Self {
        match self {
            &Direction::NORTH => Direction::EAST,
            &Direction::EAST => Direction::SOUTH,
            &Direction::SOUTH => Direction::WEST,
            &Direction::WEST => Direction::NORTH,
        }
    }
}

pub fn execute_exercises() {  
    let (a, b) = read_input();
    println!("Collision at: {:?}", exercise_1(a, b));
    let (a, b) = read_input();
    println!("Final cart at: {:?}", exercise_2(a, b))
}

fn read_input() -> (TrackMap, Vec<(Position, Direction, u32)>) {
    parse_input(include_str!("../input/day13_in.txt"))
}

fn parse_input(input: &str) -> (TrackMap, Vec<(Position, Direction, u32)>) {
    let mut map = [[' '; 152]; 152];
    let mut carts = Vec::with_capacity(20);
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {            
            match character {
                '>' => {carts.push(((y, x), Direction::EAST, 0));}
                'v' => {carts.push(((y, x), Direction::SOUTH, 0));}
                '<' => {carts.push(((y, x), Direction::WEST, 0));}
                '^' => {carts.push(((y, x), Direction::NORTH, 0));}
                '/' => {map[y][x] = character;},
                '\\' => {map[y][x] = character;},
                '+' => {map[y][x] = character;},
                _=> {}
            }
        }
    }

    (map, carts)
}

// Remember coords are in (y, x) for sorting (matrix coords)
fn exercise_1(map: TrackMap, mut carts: Vec<(Position, Direction, u32)>) -> Position {
    
    loop {
        carts.sort();
        let mut set = HashSet::with_capacity(carts.len());
        for (pos, direction, state) in carts.iter_mut() {
            // Move cart            
            *pos = *pos + *direction;
            
            if !set.insert(*pos) {
                let (y, x) = *pos;
                return (x, y);
            }            

            // Next direction
            let character = map[pos.0][pos.1];
            let result = match (character, *state) {
                ('+', 0) => (direction.left(), 1),
                ('+', 1) => (*direction, 2),
                ('+', 2) => (direction.right(), 0),
                (_, _) => (direction.next_direction(character), *state)
            };
            *direction = result.0;
            *state = result.1;
        }
    }
}

// Remember coords are in (y, x) for sorting (matrix coords)
fn exercise_2(map: TrackMap, mut carts: Vec<(Position, Direction, u32)>) -> Position {

    loop {
        // We can probably skip a whole lot of hashmap insertions by checking the min distance between two carts.
        carts.sort();
        let mut visited: HashMap<Position, Vec<(Position, Direction, u32)>> = HashMap::with_capacity(20);
        for (pos, direction, state) in carts.iter_mut() {
            
            if visited.contains_key(pos) {
                visited.get_mut(pos).unwrap().push((*pos, *direction, *state));     
                continue;
            }
            // Move cart
            *pos = *pos + *direction;

            // Next direction
            
            let character = map[pos.0][pos.1];
            let result = match (character, *state) {
                ('+', 0) => (direction.left(), 1),
                ('+', 1) => (*direction, 2),
                ('+', 2) => (direction.right(), 0),
                (_, _) => (direction.next_direction(character), *state)
            };
            *direction = result.0;
            *state = result.1;

            visited.entry(*pos).or_insert(vec!()).push((*pos, *direction, *state));
        }

        // Remove all carts on a collision place        
        carts = visited.values().filter(|v| {
            v.len() <= 1
        }).flatten().cloned().collect();

        if carts.len() == 1 {            
            let (y, x) = carts[0].0;
            return (x, y);
        }     
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day13_ex1_s1() {
        let input = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
        let (a, b) = parse_input(input);        
        let result = exercise_1(a, b);

        assert_eq!(result, (7, 3));
    }

    #[test]
    fn day13_ex1_s2() {
        let (a, b) = read_input();
        assert_eq!(exercise_1(a, b), (32,99));
    }

    #[test]
    fn day13_ex2_s1() {
        let input = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
        let (a, b) = parse_input(input);        
        let result = exercise_2(a, b);

        assert_eq!(result, (6, 4));
    }    

    #[test]
    fn day13_ex2_s2() {
        let (a, b) = read_input();
        assert_eq!(exercise_2(a, b), (56, 31));
    }

    #[bench]
    fn day13_bench_read(b: &mut Bencher) {
        b.iter(|| {
            let (a, b) = read_input();
        })
    }

    #[bench]
    fn day13_bench_ex1(b: &mut Bencher) {
        let (map, carts) = read_input();
        b.iter(|| {            
            exercise_1(map.clone(), carts.clone());
        });
    }

    #[bench]
    fn day13_bench_ex2(b: &mut Bencher) {
        let (map, carts) = read_input();
        b.iter(|| {            
            exercise_2(map.clone(), carts.clone());
        });
    }
}