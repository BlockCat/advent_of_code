use hashbrown::{HashMap, HashSet};
use std::{
    collections::{BinaryHeap, VecDeque},
    iter::FromIterator,
};
// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day21.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

type Input = Vec<(Vec<String>, Vec<String>)>;
fn read_input(input: &str) -> Input {
    input.lines().map(read_line).collect()
}

fn read_line(line: &str) -> (Vec<String>, Vec<String>) {
    let mut it = line.split(" (contains "); //)
    let foods = it
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect();
    let allergens = it
        .next()
        .unwrap()
        .replace(")", "")
        .split(", ")
        .map(|x| x.to_string())
        .collect();

    (foods, allergens)
}

#[derive(Debug)]
struct FlowGraph<'a> {
    nodes: Vec<&'a str>,
    edges: Vec<HashMap<usize, bool>>,
    residu: Vec<HashMap<usize, bool>>,
}

fn fuckit(input: &Input) -> (HashSet<&str>, HashMap<&str, &str>) {
    let mut pot_mapping: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (ingredients, allergens) in input {
        for allergen in allergens {
            let m = pot_mapping
                .entry(allergen)
                .or_insert_with(|| ingredients.iter().map(|x| &x[..]).collect());

            *m = m
                .intersection(&ingredients.iter().map(|x| &x[..]).collect())
                .map(|x| *x)
                .collect::<HashSet<&str>>();
        }
    }

    let mut mapping = HashMap::new();

    while pot_mapping.iter().any(|x| x.1.len() > 0) {
        let mut toremove = Vec::new();
        for x in pot_mapping.iter().filter(|x| x.1.len() == 1) {
            mapping.insert(*x.0, *x.1.iter().next().unwrap());
            toremove.push(*x.1.iter().next().unwrap());
        }

        for r in toremove {
            for (_, x) in pot_mapping.iter_mut() {
                x.remove(r);
            }
        }

        for (ingredients, allergens) in input {
            for allergen in allergens {
                let m = pot_mapping
                    .entry(allergen)
                    .or_insert_with(|| ingredients.iter().map(|x| &x[..]).collect());

                *m = m
                    .intersection(&ingredients.iter().map(|x| &x[..]).collect())
                    .map(|x| *x)
                    .collect::<HashSet<&str>>();
            }
        }
    }

    println!("{:?}", mapping);

    let not = ingredients(input)
        .difference(&mapping.values().map(|x| *x).collect::<HashSet<&str>>())
        .cloned()
        .collect();

    (not, mapping)
}

fn ingredients(input: &Input) -> HashSet<&str> {
    input
        .iter()
        .flat_map(|(x, _)| x.iter())
        .map(|x| &x[..])
        .collect()
}

fn exercise_1(input: &Input) -> usize {
    let (without, mapping) = fuckit(input);

    println!("{:?}", without);
    println!("{:?}", mapping);
    let mut sum = 0;
    for (ingredients, _) in input {
        for ing in ingredients.iter() {
            if without.contains(&ing[..]) {
                sum += 1;
            }
        }
    }
    sum
}

fn exercise_2(input: &Input) -> String {
    let (without, mapping) = fuckit(input);

    let mut mapping = mapping.into_iter().collect::<Vec<_>>();
    mapping.sort();
    println!("{:?}", mapping);

    mapping
        .into_iter()
        .map(|x| x.1)
        .collect::<Vec<_>>()
        .connect(",")
        .to_string()
}

// fn exercise_1(input: &Input) -> usize {
//     let (mut graph, allergens, ingredients) = build_graph(input);

//     println!("{:?}", graph.nodes);
//     println!("{:?}", graph.edges);
//     println!(
//         "{}, {}, {}",
//         graph.nodes.len(),
//         graph.edges.len(),
//         graph.residu.len()
//     );
//     while find_flow(&mut graph) {
//         // println!("{:?}", graph);
//     }

//     let mut without = HashSet::new();

//     for i in (1 + allergens.len())..graph.nodes.len() - 1 {
//         let node = graph.nodes[i];
//         let mm = graph.residu[i].iter().filter(|x| *x.1).collect::<Vec<_>>();
//         if mm.len() == 0 {
//             without.insert(node);
//         } else if mm.len() == 1 {
//         } else {
//             unreachable!()
//         }
//     }

//     let mut sum = 0;
//     for (ingredients, _) in input {
//         for ing in ingredients.iter() {
//             if without.contains(&ing[..]) {
//                 sum += 1;
//             }
//         }
//     }

//     //

//     for i in 0..graph.nodes.len() {
//         let edges = graph.edges[i]
//             .iter()
//             .filter(|x| *x.1)
//             .map(|x| graph.nodes[*x.0])
//             .collect::<Vec<_>>();

//         println!("{}: edges {:?}", graph.nodes[i], edges);
//         let edges = graph.residu[i]
//             .iter()
//             .filter(|x| *x.1)
//             .map(|x| graph.nodes[*x.0])
//             .collect::<Vec<_>>();
//         println!("{}: residu {:?}", graph.nodes[i], edges);
//     }

//     let mut without = Vec::from_iter(without);
//     without.sort();
//     println!("{:?}", without);

//     sum
// }

fn build_graph(input: &Input) -> (FlowGraph, Vec<&str>, Vec<&str>) {
    let mut allergens_indredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();
    let mut all_allergens: HashSet<&str> = HashSet::new();

    for (ingredients, allergens) in input {
        for all in allergens {
            all_allergens.insert(all);
            for ing in ingredients {
                all_ingredients.insert(ing);
                allergens_indredients
                    .entry(all)
                    .or_insert_with(|| HashSet::new())
                    .insert(ing);
            }
        }
    }

    let mut all_ingredients = Vec::from_iter(all_ingredients);
    let mut all_allergens = Vec::from_iter(all_allergens);

    all_allergens.sort();
    all_ingredients.sort();

    let mut nodes = Vec::with_capacity(2 + all_allergens.len() + all_ingredients.len());
    let mut edges = Vec::new();
    let mut residu = Vec::new();

    nodes.push("start");
    edges.push(HashMap::new());
    residu.push(HashMap::new());

    for allergen in &all_allergens {
        edges[0].insert(nodes.len(), false);
        edges.push(HashMap::new());
        nodes.push(*allergen);

        let mut m = HashMap::new();
        m.insert(0, false);
        residu.push(m);
    }

    let mut index_map = HashMap::new();
    let mut finalResidu = HashMap::new();
    for ingredient in &all_ingredients {
        let index = nodes.len();
        index_map.insert(*ingredient, index);
        let mut m = HashMap::new();
        m.insert(1 + all_allergens.len() + all_ingredients.len(), false);
        edges.push(m);
        nodes.push(*ingredient);
        residu.push(HashMap::new());

        finalResidu.insert(index, false);
    }

    nodes.push("end");
    edges.push(Default::default());

    for (index, allergen) in all_allergens.iter().enumerate() {
        for ingredient in &allergens_indredients[allergen] {
            let ingredient_index = index_map[*ingredient];
            edges[1 + index].insert(ingredient_index, false);
            residu[ingredient_index].insert(1 + index, false);
        }
    }

    residu.push(finalResidu);

    (
        FlowGraph {
            nodes,
            edges,
            residu,
        },
        all_allergens,
        all_ingredients,
    )
}

fn find_flow(graph: &mut FlowGraph) -> bool {
    let mut stack = VecDeque::new();
    stack.push_front((0usize, vec![0]));

    // let mut visited = HashSet::new();

    while let Some((index, path)) = stack.pop_back() {
        if index == graph.nodes.len() - 1 {
            // we found it.
            // mutate it.

            for i in 0..(path.len() - 1) {
                let current = &path[i];
                let next = &path[i + 1];

                if let Some(x) = graph.edges[*current].get_mut(next) {
                    assert!(!*x);
                    *x = true;
                    *graph.residu[*next].get_mut(current).unwrap() = true;
                } else if let Some(x) = graph.residu[*current].get_mut(next) {
                    assert!(*x);
                    *x = false;
                    *graph.edges[*next].get_mut(current).unwrap() = false;
                } else {
                    unreachable!()
                }
            }

            return true;
        }

        let mut edges = Vec::from_iter(graph.edges[index].iter());
        let mut residu = Vec::from_iter(graph.residu[index].iter());
        edges.sort();
        residu.sort();

        for (n, used) in edges {
            if !*used && !path.contains(n) {
                let mut np = path.clone();
                np.push(*n);
                stack.push_back((*n, np));
            }
        }
        for (n, used) in residu {
            if *used && !path.contains(n) {
                let mut np = path.clone();
                np.push(*n);
                stack.push_back((*n, np));
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d18ex1() {
        let input = read_input(include_str!("input/day21test.txt"));
        assert_eq!(2, exercise_1(&input));
        // assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input(include_str!("input/day21test.txt"));
        assert_eq!(12, exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day21.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day21.txt"));
        b.iter(|| exercise_1(&input));
    }
}
