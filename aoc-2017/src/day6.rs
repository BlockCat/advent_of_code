use std::collections::HashMap;

fn read_input(input: &str) -> Vec<u32> {
    input.split_whitespace().map(|l| l.parse().unwrap()).collect::<Vec<_>>()
}

fn algorithm1(mut memory: Vec<u32>) -> (usize, usize) {
    let mut set = HashMap::<Vec<u32>, usize>::new();
    let memory_size = memory.len();
    set.insert(memory.clone(), 0);
    for i in 1.. {
        // Find highest
        let (index, max) = memory.iter().enumerate().map(|(a, b)| (a, *b)).max_by_key(|&(a, b)| (b, std::cmp::Reverse(a))).unwrap();
        //println!("{:?} -> m:{}, i:{}", memory, max, index);
        memory[index] = 0;
        for j in 1..=(max as usize) {
            memory[(index + j) % memory_size] += 1;
        }

        if let Some(v) = set.insert(memory.clone(), i) {
            return (i, i - v);
        }
    }
        
    unreachable!()    
}

#[test]
fn test_example() {
    let memory = vec!(0, 2, 7, 0);
    assert_eq!(algorithm1(memory), (5, 4));
}

#[test]
fn run_6() {
    let memory = read_input(include_str!("input/day6.txt"));
    println!("{:?}", algorithm1(memory));
}