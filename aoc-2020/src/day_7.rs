use hashbrown::{HashMap, HashSet};
use std::{collections::VecDeque, iter::FromIterator};
use utils::Grid;

pub fn run() {
    let input = read_input(include_str!("input/day7test.txt").trim());
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

fn read_input(input: &str) -> (HashMap<String, usize>, Grid<Option<u32>>) {
    let rules = input.lines().map(parse_line_fast).collect::<Vec<_>>();

    let mut map = HashMap::new();

    for (id, rule) in rules.iter().enumerate() {
        if !map.contains_key(&rule.0) {
            map.insert(rule.0.clone(), id);
        }
    }

    let mut grid: Grid<Option<u32>> = Grid::new(map.len(), map.len());

    for (bag, contains) in rules {
        let id = map[&bag];
        for contain in contains {
            // println!("{} has {} {}", bag, contain.0, contain.1);
            grid.set(id, map[&contain.1], Some(contain.0));
        }
    }

    (map, grid)
}

fn parse_line_fast(line: &str) -> (String, Vec<(u32, String)>) {
    let mut it = line.split(' ');
    let bag = it.by_ref().take(2).collect::<String>();
    it.next().unwrap();
    it.next().unwrap();

    let mut bags = Vec::new();

    while let Some(num) = it.next() {
        if num == "no" {
            return (bag, bags);
        }
        let number = num.parse().unwrap();
        let contains = it.by_ref().take(2).collect::<String>();
        it.next().unwrap();
        bags.push((number, contains));
    }

    (bag, bags)
}

fn exercise_1((map, grid): &(HashMap<String, usize>, Grid<Option<u32>>)) -> usize {
    let mut visited = HashSet::<usize>::new();

    let shiny_gold_id = map["shinygold"];
    visited.insert(shiny_gold_id);

    let mut stack = VecDeque::new();
    stack.push_back(shiny_gold_id);

    while let Some(id) = stack.pop_back() {
        for y in 0..grid.height {
            if let Some(Some(_)) = grid.get(y, id) {
                if !visited.contains(&y) {
                    stack.push_front(y);
                    visited.insert(y);
                }
            }
        }
    }

    visited.len() - 1
}

fn exercise_2((map, grid): &(HashMap<String, usize>, Grid<Option<u32>>)) -> usize {
    let mut visited = HashSet::<usize>::new();

    let shiny_gold_id = map["shinygold"];
    visited.insert(shiny_gold_id);

    let mut stack = VecDeque::new();
    stack.push_back((1usize, shiny_gold_id));

    let mut sum = 0;
    while let Some((amount, id)) = stack.pop_back() {
        for y in 0..grid.height {
            if let Some(Some(n)) = grid.get(id, y) {
                stack.push_front((*n as usize * amount, y));
                sum += *n as usize * amount;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d7p1_test() {
        let input = read_input(include_str!("input/day7test.txt"));
        assert_eq!(4, exercise_1(&input));
    }

    #[test]
    fn d7p2_test() {
        let input = read_input(include_str!("input/day7test.txt"));
        assert_eq!(32, exercise_2(&input));
    }

    #[bench]
    fn d7_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day7.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d7_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day7.txt"));
        b.iter(|| exercise_2(&input));
    }    
}
