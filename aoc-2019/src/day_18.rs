use crate::test::Bencher;
use hashbrown::HashMap;
use hashbrown::HashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use utils::Grid;
use utils::{Direction, Vector2};

#[derive(Clone, Eq, PartialEq, Debug)]
enum Cell {
    Entrance,
    Wall,
    Empty,
    Key(char),
    Door(char),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];
pub fn run() {
    let input = read_input(include_str!("input/day18.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn read_input(input: &str) -> Grid<Cell> {
    Grid::from_vec(input.lines().map(read_line).collect())
}

fn read_line(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '#' => Cell::Wall,
            '@' => Cell::Entrance,
            '.' => Cell::Empty,
            c if c >= 'a' && c <= 'z' => Cell::Key(c),
            c if c >= 'A' && c <= 'Z' => Cell::Door(c),
            c => panic!("Invalid code: {}", c),
        })
        .collect()
}

fn create_comparer(grid: &Grid<Cell>) -> u32 {
    let mut key_comparer = 0u32;

    for c in &grid.grid {
        if let Cell::Key(x) = c {
            key_comparer |= 1 << (*x as u32 - 'a' as u32);
        }
    }

    !key_comparer
}
fn exercise_1(grid: Grid<Cell>) -> usize {    
    exercise_1a(grid, 0)
}

fn exercise_1a(grid: Grid<Cell>, find_key: u32) -> usize {
    let mut entrance = None;
    let mut visisted: HashSet<(u32, Vector2)> = HashSet::new();
    let mut queue = VecDeque::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(Cell::Entrance) = grid.get(x, y) {
                entrance = Some(Vector2(x as isize, y as isize));
            }
        }
    }

    if entrance == None {
        grid.print();
        panic!("No entrance found!");
    }

    let key_comparer = create_comparer(&grid) | 0;

    //queue.push_front((key_comparer, 0, vec!(), entrance.unwrap()));
    queue.push_front((key_comparer, 0, entrance.unwrap()));

    'nodes: while let Some((mut keys, steps, pos)) = queue.pop_back() {
        if keys == !0 {
            return steps - 1;
        }
        if visisted.contains(&(keys, pos)) {
            continue;
        }

        visisted.insert((keys, pos));

        match grid.get(pos.0 as usize, pos.1 as usize) {
            Some(Cell::Key(c)) => {
                keys |= 1 << (*c as u32 - 'a' as u32);
            }
            Some(Cell::Door(c)) => {
                if (keys & (1 << (*c as u32 - 'A' as u32))) == 0 {
                    continue 'nodes;
                }
            }
            _ => {}
        }

        for dir in &DIRECTIONS {
            let next_pos = pos + *dir;

            match grid.get(next_pos.0 as usize, next_pos.1 as usize) {
                Some(Cell::Empty) | Some(Cell::Entrance) => {
                    queue.push_front((keys, steps + 1, next_pos));
                }
                Some(Cell::Key(_)) => {
                    queue.push_front((keys, steps + 1, next_pos));
                }
                Some(Cell::Door(_)) => {
                    queue.push_front((keys, steps + 1, next_pos));
                }
                Some(Cell::Wall) => {}
                None => panic!("Grid not found: {:?}", next_pos),
            }
        }
    }

    unreachable!()
}

fn adjust_grid(grid: &mut Grid<Cell>) -> [Vector2; 4] {
    let mut entrance = None;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(Cell::Entrance) = grid.get(x, y) {
                entrance = Some(Vector2(x as isize, y as isize));
            }
        }
    }
    if entrance == None {
        grid.print();
        panic!("No entrance found!");
    }

    let entrance = entrance.unwrap();
    let x = entrance.0 as usize;
    let y = entrance.1 as usize;
    grid.set(x - 1, y - 1, Cell::Entrance);
    grid.set(x, y - 1, Cell::Wall);
    grid.set(x + 1, y - 1, Cell::Entrance);
    grid.set(x - 1, y, Cell::Wall);
    grid.set(x, y, Cell::Wall);
    grid.set(x + 1, y, Cell::Wall);
    grid.set(x - 1, y + 1, Cell::Entrance);
    grid.set(x, y + 1, Cell::Wall);
    grid.set(x + 1, y + 1, Cell::Entrance);
    let x = x as isize;
    let y = y as isize;
    [
        Vector2(x - 1, y - 1),
        Vector2(x - 1, y + 1),
        Vector2(x + 1, y - 1),
        Vector2(x + 1, y + 1),
    ]
}

fn find_path(
    grid: &Grid<Cell>,
    start: Vector2,
    dest: Vector2,
    max_keys: u32,
) -> (u32, Option<usize>) {
    let mut stack = BinaryHeap::new();

    stack.push((Reverse(Vector2::manhattan(&start, &dest)), 0, start, 0));

    let mut visited = HashSet::new();

    while let Some((_, steps, position, mut keys)) = stack.pop() {
        if position == dest {
            return (keys, Some(steps));
        }

        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);

        if let Some(Cell::Door(c)) = grid.get(position.0 as usize, position.1 as usize) {
            if max_keys & (1 << (*c as u32 - 'A' as u32)) > 0 {
                keys |= 1 << (*c as u32 - 'A' as u32);
            }
        }

        for dir in &DIRECTIONS {
            let next_pos = position + *dir;

            match grid.get(next_pos.0 as usize, next_pos.1 as usize) {
                Some(Cell::Empty) | Some(Cell::Entrance) | Some(Cell::Key(_))
                | Some(Cell::Door(_)) => {
                    stack.push((
                        Reverse(Vector2::manhattan(&start, &dest)),
                        steps + 1,
                        next_pos,
                        keys,
                    ));
                }
                Some(Cell::Wall) => {}
                None => panic!("Grid not found: {:?}", next_pos),
            }
        }
    }

    (max_keys, None)
}

fn exercise_2(mut grid: Grid<Cell>) -> usize {
    let entrances = adjust_grid(&mut grid);

    let mut key_locations = HashMap::new();
    let mut vector_locations = HashMap::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if let Some(Cell::Key(c)) = grid.get(x, y) {
                key_locations.insert(*c, Vector2(x as isize, y as isize));
                vector_locations.insert(Vector2(x as isize, y as isize), *c);
            }
        }
    }

    let mut maxmap: HashMap<(Vector2, Vector2), Vec<(u32, Option<usize>)>> = HashMap::new();
    let mut visitable = HashMap::new();
    let kl = key_locations.values().collect::<Vec<_>>();

    for i in 0..kl.len() {
        let a = kl[i];
        maxmap.insert((a.clone(), a.clone()), vec![(!0, None)]);
        for j in i + 1..kl.len() {
            let b = kl[j];
            let path = find_path(&grid, a.clone(), b.clone(), !0);
            if path.0 != !0 || path.1.is_some() {
                //println!("{:?}, {:?} -- k:{:b}, x: {:?}", a, b, path.0, path.1);
                visitable
                    .entry(a)
                    .or_insert(Vec::new())
                    .push((vector_locations[b], b.clone()));
                visitable
                    .entry(b)
                    .or_insert(Vec::new())
                    .push((vector_locations[a], a.clone()));
                maxmap
                    .entry((a.clone(), b.clone()))
                    .or_insert(Vec::new())
                    .push(path);
                maxmap
                    .entry((b.clone(), a.clone()))
                    .or_insert(Vec::new())
                    .push(path);
            }
        }
    }
    for entrance in &entrances {
        for i in 0..kl.len() {
            let b = kl[i];
            let path = find_path(&grid, entrance.clone(), b.clone(), !0);
            visitable
                .entry(entrance)
                .or_insert(Vec::new())
                .push((vector_locations[b], b.clone()));

            maxmap
                .entry((entrance.clone(), b.clone()))
                .or_insert(Vec::new())
                .push(path);
        }
    }
    let key_comparer = create_comparer(&grid);

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut visited_2: HashMap<[Vector2; 4], (usize, u32)> = HashMap::new();

    queue.push((Reverse(0), key_comparer, entrances));

    while let Some((steps, keys, positions)) = queue.pop() {
        let total_steps = steps.0;

        if visited.contains(&(keys, positions)) {
            continue;
        }
        if let Some((s2, k2)) = visited_2.get(&positions) {
            if steps.0 >= *s2 && (keys & k2) == keys {
                continue;
            }
        }

        if keys == !0 {
            return total_steps;
        }

        visited.insert((keys, positions));
        visited_2.insert(positions, (steps.0, keys));

        for (i, pos) in positions.iter().enumerate() {
            if !visitable.contains_key(pos) {
                continue;
            }

            for (k, dst) in &visitable[pos] {
                if keys & (1 << (*k as u32 - 'a' as u32)) == 1 {
                    continue;
                }
                if !maxmap.contains_key(&(pos.clone(), dst.clone())) {
                    continue;
                }
                if maxmap[&(pos.clone(), dst.clone())][0].1 == None {
                    continue;
                }

                let steps = maxmap[&(pos.clone(), dst.clone())]
                    .iter()
                    .filter_map(|x| match x {
                        (k, Some(s)) if (k | keys) == keys => Some(s),
                        _ => None,
                    })
                    .min();

                let steps: Option<usize> = steps.cloned(); 

                if let Some(steps) = steps {
                    let mut ppp = positions.clone();
                    ppp[i] = dst.clone();
                    let keys = keys | (1 << (*k as u32 - 'a' as u32));
                    queue.push((Reverse(total_steps + steps), keys, ppp));
                }
            }
        }
    }

    unreachable!()
}

#[test]
fn d18_test_2() {
    let input = read_input(
        r"#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
    );
    assert_eq!(exercise_2(input), 8);
    println!("1");

    let input = read_input(
        r"###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############",
    );
    assert_eq!(exercise_2(input), 24);
    println!("2");

    let input = read_input(
        r"#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############",
    );
    assert_eq!(exercise_2(input), 32);
    println!("3");

    let input = read_input(
        r"#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
    );
    assert_eq!(exercise_2(input), 72);
    println!("4");
}
#[test]
fn d18_test_1() {
    let input = read_input("#########\n#b.A.@.a#\n#########");
    assert_eq!(exercise_1(input), 8);

    let input = read_input("########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################");
    assert_eq!(exercise_1(input), 86);

    let input = read_input(
        r"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
    );
    assert_eq!(exercise_1(input), 132);

    let input = read_input(
        r"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
    );
    assert_eq!(exercise_1(input), 136);

    let input = read_input(
        r"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
    );
    assert_eq!(exercise_1(input), 81);

}

#[bench]
fn d18_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day18.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d18_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day18.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d18_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day18.txt")));
}
