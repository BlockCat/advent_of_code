mod pathfinding {
    use super::*;
    use std::collections::VecDeque;    
    use std::ops::Add;
    use hashbrown::{HashMap, HashSet};
    

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

    pub fn find_path(map: &Vec<Vec<bool>>, (s_x, s_y): Location, targets: &[Location], entities: &[Location]) -> Vec<Location> {        
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut queue: BinaryHeap<Reverse<_>> = BinaryHeap::new();
        queue.push(Reverse((0, s_y, s_x)));

        let mut visited: HashMap<Location, Option<Direction>> = HashMap::new();
        visited.insert((s_x, s_y), None);
        
        let targets: HashSet<Location> = targets.iter().cloned().collect();
        let entities: HashSet<Location> = entities.iter().cloned().collect();

        let mut max_size = 1000;
        let mut destos = Vec::new();

        while !queue.is_empty() {            
            let Reverse((size, y, x)) = queue.pop().unwrap();       

            if size > max_size {
                continue;
            }

            for direction in vec![Direction::North, Direction::West, Direction::East, Direction::South] {
                let next_pos = (x, y) + direction;
                let is_target = targets.contains(&next_pos);
                let is_too_long = size + 1 > max_size;
                let is_visited = visited.contains_key(&next_pos);
                let is_wall = !map[next_pos.1][next_pos.0]; // Remember that wall map is transposed
                let is_other = entities.contains(&next_pos);
                
                if is_target && !is_too_long {
                    if !is_visited {
                        visited.insert(next_pos, Some(direction));
                    }
                    destos.push(next_pos);
                    max_size = size + 1;                    
                    continue;
                }             

                if is_wall & !is_visited & !is_other && !is_too_long {                    
                    visited.insert(next_pos, Some(direction));                    
                    queue.push(Reverse((size + 1, next_pos.1, next_pos.0)));
                }                
            }
        }

        match destos.iter().min_by_key(|&(x, y)| (y, x)) {
            Some(dest) => rebuild_path(*dest, visited),
            None => vec!((s_x, s_y))
        }
    }

    fn rebuild_path(mut target: Location, visited: HashMap<Location, Option<Direction>>) -> Vec<Location> {
        let mut path = VecDeque::new();

        loop {
            path.push_front(target);            

            match visited[&target] {
                Some(Direction::North) => target = target + Direction::South,
                Some(Direction::East) => target = target + Direction::West,
                Some(Direction::South) => target = target + Direction::North,
                Some(Direction::West) => target = target + Direction::East,
                _ => return path.into()
            }
        }        
    }
}

type Health = i32;
type Location = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Elf, Goblin
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entity {
    entity_type: Type,
    health: Health,
    location: Location,
}

pub fn execute_exercises() {
    let (a, b) = parse_input(include_str!("../input/day15_in.txt"));
    println!("Checksum: {}", exercise_1(a, b));

    let (a, b) = parse_input(include_str!("../input/day15_in.txt"));    
    println!("Checksum: {:?}", exercise_2(a, b));
}


fn parse_input(input: &str) -> (Vec<Vec<bool>>, Vec<Entity>) {
    let map = input.lines().map(|l| {
        l.chars().map(|c| c == '#').collect()
    }).collect();

    let entities = input.lines().enumerate().map(|(y, l)| {
        l.chars().enumerate().filter_map(move |(x, c)| {
            match c {
                'G' => Some(Entity {
                    entity_type: Type::Goblin,
                    health: 200,
                    location: (x, y)
                }),
                'E' => Some(Entity {
                    entity_type: Type::Elf,
                    health: 200,
                    location: (x, y)
                }),
                _ => None
            }
        })
    }).flatten().collect();
    (map, entities)
}

fn find_path(map: &Vec<Vec<bool>>, entities: &Vec<Entity>, entity: &Entity) -> Vec<Location> {
    let targets: Vec<_> = entities.iter().filter(|e| { e.entity_type != entity.entity_type && e.health > 0}).map(|e| e.location).collect();
    let objects: Vec<_> = entities.iter().filter(|e| *e != entity && e.health > 0).map(|e| e.location).collect();
    pathfinding::find_path(&map, entity.location, &targets, &objects)
}

fn get_enemy_mut<'a>(entity: &Entity, entities: &'a mut Vec<Entity>) -> &'a mut Entity {
    entities.iter_mut()
        .filter(|e| {
            e.entity_type != entity.entity_type //find enemy entities
            && e.health > 0 // that are alive
            && manhattan(e.location, entity.location) == 1 //and one away
        }).min_by_key(|e| (e.health, e.location.1, e.location.0)).unwrap()
}

fn exercise_1(map: Vec<Vec<bool>>, mut entities: Vec<Entity>) -> usize {
    let mut goblins = entities.iter().filter(|e| e.entity_type == Type::Goblin).count();
    let mut elves = entities.iter().filter(|e| e.entity_type == Type::Elf).count();

    for i in 0.. {                
        entities.sort_by_key(|e| { (e.location.1, e.location.0) });
        
        for entity_id in 0..entities.len() {
            let entity = &entities[entity_id];
            if entity.health <= 0 {
                continue;
            }
            
            let path = find_path(&map, &entities, entity);

            if path.len() > 2 {
                entities[entity_id].location = path[1];
            }
            let entity = entities[entity_id].clone();

            // Check enemies
            
            if path.len() == 2 || path.len() == 3 {                
                let oth = get_enemy_mut(&entity, &mut entities);
                oth.health -= 3;

                if oth.health <= 0 {
                    match oth.entity_type {
                        Type::Elf => elves -= 1,
                        Type::Goblin => goblins -= 1,
                    }
                }           

                if elves == 0 || goblins == 0 {
                    let sum: usize = entities.iter()
                        .filter(|e| e.entity_type == entity.entity_type && e.health >= 0)
                        .map(|e| e.health as usize)
                        .sum();

                    let round = if entity_id == entities.iter().rposition(|e| e.health > 0).unwrap() {
                        i + 1
                    } else {
                        i
                    };
                    return sum * round;
                }
            }
        }
    }

    unreachable!()
}

fn exercise_2(map: Vec<Vec<bool>>, entities: Vec<Entity>) -> (usize, i32) {
    let mut lower_bound = 3i32; //exclusive
    let mut upper_bound = 200i32; //inclusive
    
    loop {
        let next_attack = (lower_bound + upper_bound) / 2;        

        if lower_bound == next_attack || upper_bound == next_attack {            
            return (exercise_2_help(&map, entities.clone(), upper_bound).unwrap(), upper_bound);
        }

        if let Some(_) = exercise_2_help(&map, entities.clone(), next_attack) {                 
            upper_bound = next_attack;
        } else {            
            lower_bound = next_attack;
        }        
    }
}

fn exercise_2_help(map: &Vec<Vec<bool>>, mut entities: Vec<Entity>, elf_attack: i32) -> Option<usize> {
    let mut goblins = entities.iter().filter(|e| e.entity_type == Type::Goblin).count();    
    for i in 0.. {                
        //Sort by y and then by x
        entities.sort_by_key(|e| { (e.location.1, e.location.0) });

        for entity_id in 0..entities.len() {             
            if entities[entity_id].health <= 0 { //If this entity is dead, continue
                continue;
            }

            let path = find_path(&map, &entities, &entities[entity_id]);
            if path.len() > 2 {
                entities[entity_id].location = path[1];
            }

            // Check enemies
            let entity = entities[entity_id].clone();
            if path.len() == 2 || path.len() == 3 {                
                let oth = get_enemy_mut(&entity, &mut entities);

                oth.health -= match entity.entity_type {
                    Type::Elf => elf_attack,
                    Type::Goblin => 3
                };

                if oth.health <= 0 {
                    match oth.entity_type {
                        Type::Elf => return None,
                        Type::Goblin => goblins -= 1,
                    }
                }           

                if goblins == 0 {
                    let sum: usize = entities.iter()
                        .filter(|e| e.entity_type == Type::Elf && e.health > 0)
                        .map(|e| e.health as usize)
                        .sum();

                    let round = if entity_id == entities.into_iter().rposition(|e| e.health > 0).unwrap() {
                        i + 1
                    } else {
                        i
                    };

                    return Some(sum * round);
                }
            }
        }

        entities = entities.into_iter().filter(|e| e.health > 0).collect();
    }

    unreachable!()
}

fn manhattan((x, y): Location, (a, b): Location) -> i32 {
    let x = (x as i32 - a as i32).abs();
    let y = (y as i32 - b as i32).abs();
    x + y
}

fn pretty_print(map: &Vec<Vec<bool>>, entities: &Vec<Entity>) {
    use hashbrown::HashMap;
    let m: HashMap<Location, &Entity> = entities.iter().filter(|e| e.health > 0).map(|e| (e.location, e)).collect();

    /*for (i, e) in entities.iter().enumerate() {
        println!("{:?} entity: {:?}, h:{}", e.entity_type, e.location, e.health);
    }*/

    for y in 0..map.len() {
        let col = &map[y];
        let mut cha = Vec::new();
        for x in 0..col.len() {
            if m.contains_key(&(x, y)) {
                cha.push(m[&(x, y)]);
                match m[&(x, y)].entity_type {
                    Type::Elf => print!("E"),
                    Type::Goblin => print!("G"),                    
                }
            } else {
                if col[x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }

        for ch in cha {
            print!("  {:?}: {} ", ch.entity_type, ch.health);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day15_ex1_s1() {
        let input = r"#######   
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
####### ";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_1(map, entities), 27730);
    }

    
    #[test]
    fn day15_ex1_s2() {
        let input = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_1(map, entities), 36334);
    }

    #[test]
    fn day15_ex1_s3() {
        let input = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_1(map, entities), 39514);
    }


    #[test]
    fn day15_ex1_s4() {
        let input = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_1(map, entities), 27755);
    }

    #[test]
    fn day15_ex1_s5() {
        let input = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_1(map, entities), 28944);
    }

    #[test]
    fn day15_ex1_s6() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_1(map, entities), 18740);
    }

    #[test]
    fn day15_ex1_s7() {        
        let (a, b) = parse_input(include_str!("../input/day15_in.txt"));
        assert_eq!(exercise_1(a, b), 181952);
    }

    #[test]
    fn day15_ex2_s1() {
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_2(map, entities), (4988, 15));
    }

    #[test]
    fn day15_ex2_s2() {
        let input = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_2(map, entities), (31284, 4));
    }

    #[test]
    fn day15_ex2_s3() {
        let input = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_2(map, entities), (3478, 15));
    }

    #[test]
    fn day15_ex2_s4() {
        let input = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_2(map, entities), (6474, 12));
    }

    #[test]
    fn day15_ex2_s5() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        let (map, entities) = parse_input(input);
        assert_eq!(exercise_2(map, entities), (1140, 34));
    }

    #[test]
    fn day15_ex2_s6() {        
        let (map, entities) = parse_input(include_str!("../input/day15_in.txt"));        
        exercise_2_help(&map, entities, 25);
    }

    #[bench]
    fn day15_bench_ex1(b: &mut Bencher) {
         let (map, entities) = parse_input(include_str!("../input/day15_in.txt"));
        b.iter(move || exercise_1(map.clone(), entities.clone()));
    }

    #[bench]
    fn day15_bench_ex2(b: &mut Bencher) {
        let (map, entities) = parse_input(include_str!("../input/day15_in.txt"));
        b.iter(move || exercise_2(map.clone(), entities.clone()));
    }
    

}