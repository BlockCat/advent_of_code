use std::collections::VecDeque;

fn algorithm1(step_size: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(0);    
    for i in 1..=2017 {
        queue.rotate_left(step_size % i);
        queue.push_back(i);
    }  
    queue[0]
}


fn algorithm2(step_size: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(0);    
    for i in 1..=50000000 {
        queue.rotate_left(step_size % i);
        queue.push_back(i);
    }  
    let pos = queue.iter().position(|x| x == &0).unwrap();

    queue[(pos + 1) % queue.len()]
}

#[test]
fn test_examples() {
    assert_eq!(algorithm1(3), 638);    
}

#[test]
fn run17() {
    let input = 343;
    println!("{}", algorithm1(input));
    println!("{}", algorithm2(input));
}