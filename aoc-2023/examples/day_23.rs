use aoc_2023::{
    direction::Direction,
    grid::{Grid, StaticGrid},
    vector::Vector2,
};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type InputType = StaticGrid<char>;

// 1754
// 6034 too low
pub fn main() {
    let input = parse(include_str!("../input/day_23.txt"));

    println!("Exercise 1: {}", exercise_2(input.clone(), true));
    println!("Exercise 2: {}", exercise_2(input, false));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    println!("Exercise 1: {}", exercise_2(input.clone(), true));
    println!("Exercise 2: {}", exercise_2(input, false));
    // assert_eq!(62, exercise_1(input.clone()));
    // assert_eq!(952408144115, exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    StaticGrid::from_vec(input.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn exercise_2(input: InputType, directional: bool) -> usize {
    let graph = create_graph(&input, directional);

    let mut interesting_points = if directional {
        graph
            .iter()
            .enumerate()
            .filter(|s| s.1.len() >= 2)
            .map(|s| s.0)
            .collect::<Vec<_>>()
    } else {
        graph
            .iter()
            .enumerate()
            .filter(|s| s.1.len() > 2)
            .map(|s| s.0)
            .collect::<Vec<_>>()
    };
    interesting_points.push(0);
    interesting_points.push(graph.len() - 1);

    interesting_points.sort();

    let interesting_graph = interesting_points
        .iter()
        .map(|s| find_path(&graph, *s, &interesting_points))
        .collect::<Vec<_>>();

    find_longest_path_2(&interesting_graph, 0, interesting_points.len() - 1)
}
fn create_graph(input: &InputType, directional: bool) -> Vec<HashSet<usize>> {
    let nodes = input
        .iter()
        .filter(|s| s.1 != &'#')
        .map(|s| s.0)
        .collect::<Vec<_>>();

    let node_map = nodes
        .iter()
        .enumerate()
        .map(|(i, s)| (*s, i))
        .collect::<HashMap<_, _>>();

    let edges = nodes
        .iter()
        .map(|s| {
            if directional {
                let mut set = HashSet::new();
                let upper_neighbour = *s + Direction::North;
                let lower_neighbour = *s + Direction::South;
                let left_neighbour = *s + Direction::West;
                let right_neighbour = *s + Direction::East;

                if let Some(c) = input.get_vec(&upper_neighbour) {
                    if *c != '#' && *c != 'v' {
                        set.insert(node_map[&upper_neighbour]);
                    }
                }
                if let Some(c) = input.get_vec(&lower_neighbour) {
                    if *c != '#' && *c != '^' {
                        set.insert(node_map[&lower_neighbour]);
                    }
                }
                if let Some(c) = input.get_vec(&left_neighbour) {
                    if *c != '#' && *c != '>' {
                        set.insert(node_map[&left_neighbour]);
                    }
                }
                if let Some(c) = input.get_vec(&right_neighbour) {
                    if *c != '#' && *c != '<' {
                        set.insert(node_map[&right_neighbour]);
                    }
                }

                set
            } else {
                s.neighbours_4()
                    .iter()
                    .filter(|s| match input.get_vec(*s) {
                        Some('#') => false,
                        None => false,
                        _ => true,
                    })
                    .map(|ss| node_map[ss])
                    .collect::<HashSet<_>>()
            }
        })
        .collect::<Vec<_>>();

    edges
}

fn find_path(graph: &Vec<HashSet<usize>>, start_point: usize, points: &[usize]) -> HashSet<IEdge> {
    let mut queue = VecDeque::new();
    queue.push_back((start_point, 0));

    let mut visited = HashSet::new();

    let mut neighbours = HashSet::new();

    while let Some((point, steps)) = queue.pop_front() {
        if !visited.insert(point) {
            continue;
        }
        if point != start_point {
            if let Some(i) = points.iter().position(|a| a == &point) {
                neighbours.insert(IEdge { target: i, steps });
                continue;
            }
        }

        for neighbor in &graph[point] {
            queue.push_back((*neighbor, steps + 1));
        }
    }

    neighbours
}

fn find_longest_path_2(graph: &Vec<HashSet<IEdge>>, start: usize, end: usize) -> usize {
    let mut heap: BinaryHeap<Entry2> = BinaryHeap::new();

    heap.push(Entry2::new(start, 0, HashSet::new()));

    let mut max = 0;

    while let Some(mut entry) = heap.pop() {
        if entry.pos == end {
            if entry.steps > max {
                println!("{}", entry.steps);
                max = entry.steps;
            }
            continue;
        }

        if !entry.keys.insert(entry.pos) {
            continue;
        }

        let pos = entry.pos;
        let steps = entry.steps;

        for edge in &graph[pos] {
            heap.push(Entry2::new(
                edge.target,
                steps + edge.steps,
                entry.keys.clone(),
            ));
        }
    }
    max
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Entry {
    pos: Vector2,
    steps: usize,
    keys: HashSet<Vector2>,
    destination: Vector2,
}

impl Entry {
    fn heur(&self) -> isize {
        // let x = Vector2::manhattan(&self.pos, &self.destination) * 10;
        // self.steps as isize - x as isize
        self.steps as isize
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heur().cmp(&other.heur())
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]

struct Entry2 {
    pos: usize,
    steps: usize,
    keys: HashSet<usize>,
}

impl Entry2 {
    fn new(pos: usize, steps: usize, keys: HashSet<usize>) -> Self {
        Self { pos, steps, keys }
    }
}

impl Ord for Entry2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for Entry2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct IEdge {
    target: usize,
    steps: usize,
}
