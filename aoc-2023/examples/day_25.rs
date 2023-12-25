use std::collections::{HashMap, HashSet};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

type InputType = HashMap<String, Vec<String>>;
pub fn main() {
    let input = parse(include_str!("../input/day_25.txt"));

    println!("Exercise 1: {}", exercise_1(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    println!("Exercise 1: {}", exercise_1(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let (name, ls) = line.split_once(": ").unwrap();

    (
        name.to_string(),
        ls.split_whitespace().map(|s| s.to_string()).collect(),
    )
}

fn exercise_1(input: InputType) -> usize {
    let map: HashMap<String, HashSet<String>> = create_map(input.clone());

    (0..)
        .par_bridge()
        .find_map_any(|_| do_contraction(map.clone()))
        .unwrap()
}

fn do_contraction(map: HashMap<String, HashSet<String>>) -> Option<usize> {
    let mut map = map
        .into_iter()
        .map(|(a, b)| (a, b.into_iter().collect::<Vec<_>>()))
        .collect::<HashMap<_, _>>();

    let mut grouping = HashMap::new();
    let mut rnd = thread_rng();
    while map.len() > 2 {
        let p = map.keys().choose(&mut rnd).unwrap();

        let e = map[p].choose(&mut rnd).unwrap();

        let p = p.clone();
        let e = e.clone();

        // collapse p, e
        let mut neighbours_p = map.remove(&p).unwrap();
        let mut neighbours_e = map.remove(&e).unwrap();

        neighbours_e.retain(|s| s != &p);
        neighbours_p.retain(|s| s != &e);

        let new_node = format!("{}-{}", p, e);

        grouping.insert(new_node.clone(), (p.clone(), e.clone()));

        for n in &neighbours_e {
            for ele in map.get_mut(n).unwrap() {
                if *ele == e {
                    *ele = new_node.to_string();
                }
            }
        }
        for n in &neighbours_p {
            for ele in map.get_mut(n).unwrap() {
                if *ele == p {
                    *ele = new_node.to_string();
                }
            }
        }

        neighbours_p.extend(neighbours_e);

        map.insert(new_node, neighbours_p);
    }

    // println!("AAAAA: {:?}", map);

    let mut i = map.into_iter();

    let a = i.next().unwrap();
    let b = i.next().unwrap();

    if a.1.len() == 3 {
        let a = a.0.split("-").count();
        let b = b.0.split("-").count();

        return Some(a * b);
    } else {
        return None;
    }
}

fn create_map(input: InputType) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for (key, values) in input {
        for v in &values {
            map.entry(v.clone()).or_default().insert(key.clone());
        }
        map.entry(key.clone()).or_default().extend(values);
    }

    map
}
