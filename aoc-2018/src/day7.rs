use std::collections::BinaryHeap;
use std::iter::FromIterator;
use std::cell::RefCell;

#[derive(Eq, PartialEq)]
struct Node {
    value: char,
    constrained: RefCell<i32>,
    next: Vec<char>
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn execute_exercises() {
    println!("Order: {}", exercise_1(read_input()));
    println!("parallel time: {}", exercise_2(read_input(), 5, 60));
}

fn read_input() -> impl Iterator<Item = (char, char)> {
    parse_input(include_str!("../input/day7_in.txt"))
}

fn parse_input(input: &'static str) -> impl Iterator<Item = (char, char)> {
    input.lines().map(|l| {
        let a = l[5..6].chars().next().unwrap();
        let b = l[36..37].chars().next().unwrap();

        (a, b)
    })
}


fn create_precendence_graph(input: impl Iterator<Item = (char, char)>) -> Vec<Node> {
    let mut precendence_graph: Vec<Node> = (b'A'..=b'Z').map(|a| Node {
        value: a as char,
        constrained: RefCell::new(0),
        next: Vec::with_capacity(20)
    }).collect();

    let offset = b'A';    
    for (prec, todo) in input.map(|(a, b)| (a as u8, b as u8)) {  
        precendence_graph[(prec - offset) as usize].next.push(todo as char);
        *precendence_graph[(todo - offset) as usize].constrained.get_mut() += 1; 
    }

    precendence_graph
}


fn exercise_1(input: impl Iterator<Item = (char, char)>) -> String {

    let precendence_graph = create_precendence_graph(input);

    let mut seen = Vec::with_capacity(20);
    let mut heap = BinaryHeap::from_iter(precendence_graph.iter().filter(|node| *node.constrained.borrow() == 0 && !node.next.is_empty()));
    let offset = b'A' as usize;

    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        seen.push(node.value);
        heap.extend(node.next.iter()
            .map(|n| &precendence_graph[*n as usize - offset])
            .inspect(|n| *n.constrained.borrow_mut() -= 1)
            .filter(|n| *n.constrained.borrow() == 0));            
    }

    seen.iter().collect::<String>()
}

fn exercise_2(input: impl Iterator<Item = (char, char)>, workers: usize, seconds_per_step: i32) -> i32 {
    let precendence_graph = create_precendence_graph(input);
    let offset = b'A' as usize;

    let mut working: Vec<bool> = (0..workers).map(|_| false).collect();
    let mut heap = BinaryHeap::from_iter(precendence_graph.iter().filter(|node| *node.constrained.borrow() == 0 && !node.next.is_empty()));
    let mut time = 0;    
    let mut event_heap = BinaryHeap::new();
    
    for (worker, ref mut working) in working.iter_mut().enumerate().take(heap.len()) { // Take initial jobs        
            let node = heap.pop().unwrap();
            let time = seconds_per_step + i32::from(node.value as u8 - b'A' + 1);
            event_heap.push((-time, worker, node));
            **working = true;        
    }

    while !event_heap.is_empty() {
        let (t, worker, node) = event_heap.pop().unwrap();
        time = -t; //We don't have min heaps        
        working[worker] = false;
        
        heap.extend(node.next.iter()
            .map(|n| &precendence_graph[*n as usize - offset])
            .inspect(|n| *n.constrained.borrow_mut() -= 1)
            .filter(|n| *n.constrained.borrow() == 0));

        for (free_worker, ref mut working) in working.iter_mut().enumerate().filter(|(_, is_working)| !**is_working) {
            if !heap.is_empty() {
                let node = heap.pop().unwrap();
                let time = time + seconds_per_step + i32::from(node.value as u8 - b'A' + 1);
                event_heap.push((-time, free_worker, node));
                **working = true;
            }
        }        
    }

    time
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d7_ex1_s1() {
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(exercise_1(parse_input(input)), "CABDFE");
    }

    #[test]
    fn d7_ex1_s2() {
        assert_eq!(exercise_1(read_input()), "GKCNPTVHIRYDUJMSXFBQLOAEWZ");
    }

    #[test]
    fn d7_ex2_s1() {
        let input = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        assert_eq!(exercise_2(parse_input(input), 2, 0), 15);
    }

    #[test]
    fn d7_ex2_s2() {
        assert_eq!(exercise_2(read_input(), 4, 60), 1265);
    }

    #[bench]
    fn d7_bench_prec(b: &mut Bencher) {
        b.iter(|| create_precendence_graph(read_input()));
    }


    #[bench]
    fn d7_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d7_bench_ex2(b: &mut Bencher) {
        b.iter(|| exercise_2(read_input(), 5, 60));
    }

}