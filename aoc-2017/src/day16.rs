use std::iter::FromIterator;

#[derive(Clone)]
enum Action {
    Spin(isize),
    Exchange(usize, usize),
    Partner(char, char)
}

fn read_action(input: &str) -> Action {
    match &input[0..1] {
        "s" => Action::Spin(input[1..].parse::<isize>().unwrap()),
        "x" => {
            let mut it = input[1..].split('/');
            let a = it.next().map(str::parse::<usize>).unwrap().expect("could not parse");
            let b = it.next().map(str::parse::<usize>).unwrap().expect("could not parse");
            Action::Exchange(a, b)
        },
        "p" => {
            let mut it = input[1..].split('/');
            let a = it.next().unwrap().chars().next().unwrap();
            let b = it.next().unwrap().chars().next().unwrap();
            Action::Partner(a, b)
        },
        c => panic!("Action {} does not exist", c)
    }
}

fn algorithm1(input: impl Iterator<Item = Action>, size: &str) -> String {
    let mut offset: u32 = 0;
    let mut vec = std::collections::VecDeque::<char>::from_iter(size.chars());
    for action in input {
        match action {
            Action::Spin(v) => vec.rotate_right(v as usize),
            Action::Exchange(a, b) => {
                let temp = vec[a];
                vec[a] = vec[b];
                vec[b] = temp;
            },
            Action::Partner(a, b) => {
                let a = vec.iter().position(|x| x == &a).unwrap();
                let b = vec.iter().position(|x| x == &b).unwrap();
                let temp = vec[a];
                vec[a] = vec[b];
                vec[b] = temp;
            }
        }
    }
    vec.into_iter().collect::<String>()
}

fn algorithm2(input: impl Iterator<Item = Action>, size: &str) -> String {
    let input = input.collect::<Vec<_>>();
    let mut visited = std::collections::HashMap::<String, usize>::new(); //Which string and in which iteration.
    let mut collected = Vec::new(); //Which iteration ha
    let mut order = String::from(size);

    collected.push(order.clone());
    visited.insert(order.clone(), 0);

    for i in 1..=1_000_000_000 {
        order = algorithm1(input.iter().cloned(), &order);
        if let Some(j) = visited.insert(order.clone(), i) {
            // We already found this order at index i
            let cycle_size = i - j;
            let remaining = (1_000_000_000 - i) % cycle_size;
            return collected[j + remaining].clone();
        } else {
            collected.push(order.clone());
        }
    }

    order    
}

#[test]
fn run16() {
    let input = include_str!("input/day16.txt");

    let result1 = algorithm1(input.split(',').map(read_action), "abcdefghijklmnop");
    let result2 = algorithm2(input.split(',').map(read_action), "abcdefghijklmnop");

    println!("dance result: {} after a while {}", result1, result2);
}

#[test]
fn example_test() {
    assert_eq!(&algorithm1("s1,x3/4,pe/b".split(',').map(read_action), "abcde"), "baedc");
    assert_eq!(&algorithm2("s1,x3/4,pe/b".split(',').map(read_action), "abcde"), "abcde");
}