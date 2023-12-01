use std::collections::HashMap;
use std::collections::VecDeque;

use std::str::FromStr;

#[derive(Debug)]
enum Registry {
    Value(i64), Reg(char)
}

impl FromStr for Registry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {    
        let c = s.chars().next().unwrap();
        if c >= 'a' && c <= 'z'  {
            Ok(Registry::Reg(c))
        } else {
            Ok(Registry::Value(s.parse::<i64>().map_err(|_| format!("Error: {}", s))?))
        }
    }
}


#[derive(Debug)]
enum Op {
    Set(char, Registry), 
    Add(char, Registry), 
    Mul(char, Registry),
    Mod(char, Registry),
    Rcv(char),
    Jgz(Registry, Registry),
    Snd(Registry)
}

fn read_input(input: &str) -> Vec<Op> {
    input.lines()
        .map(|line| {
            let line = line.split_whitespace().collect::<Vec<_>>();
            match *&line[0] {
                "set" => Op::Set(line[1].chars().next().unwrap(), line[2].parse::<Registry>().unwrap()),
                "add" => Op::Add(line[1].chars().next().unwrap(), line[2].parse::<Registry>().unwrap()),
                "mul" => Op::Mul(line[1].chars().next().unwrap(), line[2].parse::<Registry>().unwrap()),
                "mod" => Op::Mod(line[1].chars().next().unwrap(), line[2].parse::<Registry>().unwrap()),
                "rcv" => Op::Rcv(line[1].chars().next().unwrap()),
                "jgz" => Op::Jgz(line[1].parse().unwrap(), line[2].parse::<Registry>().unwrap()),
                "snd" => Op::Snd(line[1].parse::<Registry>().unwrap()),
                c => panic!("invalid op: {}", c)
            }
        })
        .collect::<Vec<_>>()
}


fn get_registry(map: &mut HashMap<char, i64>, registry: &Registry) -> i64 {
    match registry {
        Registry::Value(x) => *x,
        Registry::Reg(c) => *map.entry(*c).or_insert(0)
    }
}

fn algorithm1(input: Vec<Op>) -> Option<i64> {
    let mut counter = 0isize;
    let mut last_sound = None;
    let mut map: HashMap<char, i64> = HashMap::new();
    while (counter as usize) < input.len() {
        //println!("{} {:?} after {:?}", counter, map, input[counter as usize]);
        match &input[counter as usize] {
            Op::Set(r, v) => {
                let value = get_registry(&mut map, v);
                map.insert(*r, value);
            },
            Op::Add(r, v) => *map.entry(*r).or_insert(0) += get_registry(&mut map, v),
            Op::Mul(r, v) => *map.entry(*r).or_insert(0) *= get_registry(&mut map, v),
            Op::Mod(r, v) => *map.entry(*r).or_insert(0) %= get_registry(&mut map, v),
            Op::Rcv(r) => if *map.entry(*r).or_insert(0) != 0 { return last_sound; },
            Op::Jgz(r, v) => if get_registry(&mut map, r) > 0 { counter += get_registry(&mut map, v) as isize - 1 },
            Op::Snd(r) => last_sound = Some(get_registry(&mut map, r))
        }
        counter += 1;
        //println!("{:?}", map);
    }

    unreachable!()
}

fn step_one(input: &Vec<Op>, counter: &mut isize, map: &mut HashMap<char, i64>, queue: &mut VecDeque<i64>, pr: &str) -> Option<i64> {
    let mut delayed_send = None;
    if (*counter as usize) < input.len() {
        
        //println!("{} - {} {:?}\t\t {:?}", pr, counter, map, input[*counter as usize]);
        
        match &input[*counter as usize] {
            Op::Set(r, v) => {
                let value = get_registry(map, v);
                map.insert(*r, value);
            },
            Op::Add(r, v) => *map.entry(*r).or_insert(0) += get_registry(map, v),
            Op::Mul(r, v) => *map.entry(*r).or_insert(0) *= get_registry(map, v),
            Op::Mod(r, v) => *map.entry(*r).or_insert(0) %= get_registry(map, v),
            Op::Rcv(r) => {
                if let Some(v) = queue.pop_front() {
                    map.insert(*r, v);
                } else {
                    *counter -= 1;
                }
            },
            Op::Jgz(r, v) => if get_registry(map, r) > 0 { *counter += get_registry(map, v) as isize - 1 },
            Op::Snd(r) => delayed_send = Some(get_registry(map, r)),
        }

        *counter += 1;
    }

    delayed_send
}

fn algorithm2(input: Vec<Op>) -> i64 {
    let mut counter_1 = 0isize;
    let mut counter_2 = 0isize;
    
    let mut map_1: HashMap<char, i64> = HashMap::new();
    let mut map_2: HashMap<char, i64> = HashMap::new();

    let mut queue_1 = VecDeque::<i64>::new();
    let mut queue_2 = VecDeque::<i64>::new();

    map_1.insert('p', 0);
    map_2.insert('p', 1);

    let mut send_counter = 0;
    
    //p1
    /*{
        counter_1 = 33;
        counter_2 = 30;

        map_1.insert('l', 0);
        map_1.insert('b', 31);
        map_1.insert('a', 31);
        map_1.insert('f', 1);
        map_1.insert('i', 79-55);
        map_1.insert('p', 0);
        
        map_2.insert('b', 31);
        map_2.insert('f', 1);
        map_2.insert('i', 1);
        map_2.insert('l', 0);
        map_2.insert('a', 31);
        map_2.insert('p', 0);
    }*/
    loop {
        
        let ds1 = step_one(&input, &mut counter_1, &mut map_1, &mut queue_1, "p0");
        let ds2 = step_one(&input, &mut counter_2, &mut map_2, &mut queue_2, "p1");
        
        if let Some(d1) = ds2 {
            queue_1.push_back(d1);
            send_counter += 1;
            if send_counter & 524287 == 0 {
                println!("{}", send_counter);
            }
        }
        if let Some(d1) = ds1 {
            queue_2.push_back(d1);
        }

        match (&input[counter_1 as usize], &input[counter_2 as usize]) {
            (Op::Rcv(_), Op::Rcv(_)) => {
                if queue_1.len() == 0 && queue_2.len() == 0 { 
                    return send_counter;
                }
            },
            _ => {}
        }

        if counter_1 as usize >= input.len() && counter_2 as usize >= input.len() {
            return send_counter;
        }
    }
    

    unreachable!()
}


#[test]
fn run18() {
    let input = read_input(include_str!("input/day18.txt"));
    println!("Snd {:?}", algorithm1(input));
    let input = read_input(include_str!("input/day18.txt"));
    println!("Sends: {:?}", algorithm2(input));
}

#[test]
fn test_examples() {
    let input = r"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
    let input = read_input(input);
    assert_eq!(algorithm1(input), Some(4));

    let input = r"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
    let input = read_input(input);
    assert_eq!(algorithm2(input), 3);
}