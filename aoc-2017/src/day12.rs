use std::collections::{ HashSet, VecDeque };
use std::iter::FromIterator;

type Graph = Vec<Vec<usize>>;

fn read_input(input: &str) -> Graph {
    input.lines()
        .map(parse_line)
        .collect::<Vec<Vec<_>>>()
}

fn parse_line(line: &str) -> Vec<usize> {
    line[(line.find("<-> ").unwrap() + 4)..]
        .split(", ")
        //.inspect(|x| println!("{:?}", x))
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>()
}

fn bfs_count_zero(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    queue.push(0usize);

    while let Some(node) = queue.pop() {
        if !visited.contains(&node) {
            visited.insert(node);
            queue.extend(graph[node].iter().cloned());
        }
    }

    visited.len()
}

fn count_groups(graph: &Graph) -> usize {
    let mut groups = 0;    
    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    for i in 0..graph.len() {
        if !visited.contains(&i) {
            groups += 1;
            queue.push(i);

            while let Some(node) = queue.pop() {
                if !visited.contains(&node) {
                    visited.insert(node);
                    queue.extend(graph[node].iter().cloned());
                }
            }
        }        
    }


    groups
}

#[test]
fn test_examples() {
    let input = read_input(r"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5");

    assert_eq!(bfs_count_zero(&input), 6);
    assert_eq!(count_groups(&input), 2);
}

#[test]
fn run12() {
    let input = read_input(include_str!("input/day12.txt"));

    println!("To 0: {}", bfs_count_zero(&input));
    println!("Groups: {}", count_groups(&input));
}