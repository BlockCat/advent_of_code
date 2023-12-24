use std::collections::{HashMap, VecDeque};

type InputType = Vec<Module>;

pub fn main() {
    let input = parse(include_str!("../input/day_20.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    let res = exercise_1(input.clone());
    println!("Exercise 1: {}", res);
    // assert_eq!(res, 19114);

    // let res = exercise_2(input);
    println!("Exercise 2: {}", res);
    // assert_eq!(res, 167409079868000);
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Module {
    let (name, dests) = line.split_once(" -> ").unwrap();
    let dests = dests.split(", ").map(|s| s.to_string()).collect();
    match &name[0..1] {
        "b" => Module {
            mtype: ModuleType::Broadcast,
            name: "broadcaster".to_string(),
            dests,
        },
        "&" => Module {
            mtype: ModuleType::Conjunction(HashMap::new()),
            name: name[1..].to_string(),
            dests,
        },
        "%" => Module {
            mtype: ModuleType::FlipFlop(false),
            name: name[1..].to_string(),
            dests,
        },
        _ => unreachable!(),
    }
}

fn create_map(input: InputType) -> HashMap<String, Module> {
    let mut map = input
        .into_iter()
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<_, _>>();

    let cmap = map.clone();

    for (k, v) in map.iter_mut() {
        if let ModuleType::Conjunction(srcs) = &mut v.mtype {
            for (k2, v2) in cmap.iter() {
                if v2.dests.contains(k) {
                    srcs.insert(k2.clone(), false);
                }
            }
        }
    }
    map
}

fn exercise_1(input: InputType) -> usize {
    let mut map = create_map(input);
    let mut queue = VecDeque::new();

    let mut low_pulses = 1000;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), false));
        start_queue(&mut queue, &mut map, &mut low_pulses, &mut high_pulses);
    }

    low_pulses * high_pulses
}

fn exercise_2(input: InputType) -> usize {
    let mut map = create_map(input);
    let mut queue = VecDeque::new();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for i in 1..5000 {
        low_pulses += 1;
        queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

        if start_queue_2(&mut queue, &mut map, &mut low_pulses, &mut high_pulses, i) {
            return i;
        }
    }

    unreachable!()
}

fn start_queue(
    queue: &mut VecDeque<(String, String, bool)>,
    map: &mut HashMap<String, Module>,
    low_pulses: &mut usize,
    high_pulses: &mut usize,
) -> bool {
    while let Some((src, dest, pulse)) = queue.pop_front() {
        // println!("{} -> {} ({})", src, dest, pulse);
        if let Some(module) = map.get_mut(&dest) {
            match &mut module.mtype {
                ModuleType::Broadcast => {
                    for dest in &module.dests {
                        queue.push_back((module.name.clone(), dest.clone(), pulse));
                        if pulse {
                            *high_pulses += 1;
                        } else {
                            *low_pulses += 1;
                        }
                    }
                }
                ModuleType::Conjunction(map) => {
                    map.insert(src.clone(), pulse);
                    if map.values().all(|s| *s) {
                        for dest in &module.dests {
                            queue.push_back((module.name.clone(), dest.clone(), false));
                            *low_pulses += 1;
                        }
                    } else {
                        for dest in &module.dests {
                            queue.push_back((module.name.clone(), dest.clone(), true));
                            *high_pulses += 1;
                        }
                    }
                }
                ModuleType::FlipFlop(b) => {
                    if !pulse {
                        *b = !*b;
                        for dest in &module.dests {
                            queue.push_back((module.name.clone(), dest.clone(), *b));
                            if *b {
                                *high_pulses += 1;
                            } else {
                                *low_pulses += 1;
                            }
                        }
                    }
                }
            }
        } else {
            // println!("{} not found", dest);
        }
    }
    return false;
}

fn start_queue_2(
    queue: &mut VecDeque<(String, String, bool)>,
    map: &mut HashMap<String, Module>,
    low_pulses: &mut usize,
    high_pulses: &mut usize,
    bp: usize,
) -> bool {
    while let Some((src, dest, pulse)) = queue.pop_front() {
        if let Some(module) = map.get_mut(&dest) {
            match &mut module.mtype {
                ModuleType::Broadcast => {
                    for dest in &module.dests {
                        queue.push_back((module.name.clone(), dest.clone(), pulse));
                        if pulse {
                            *high_pulses += 1;
                        } else {
                            *low_pulses += 1;
                        }
                    }
                }
                ModuleType::Conjunction(map) => {
                    map.insert(src.clone(), pulse);
                    if map.values().all(|s| *s) {
                        for dest in &module.dests {
                            queue.push_back((module.name.clone(), dest.clone(), false));
                            *low_pulses += 1;
                        }
                    } else {
                        if module.name == "hf"
                            || module.name == "rh"
                            || module.name == "jm"
                            || module.name == "jg"
                        {
                            println!("f: {} ---- {}", module.name, bp);
                        }
                        for dest in &module.dests {
                            queue.push_back((module.name.clone(), dest.clone(), true));
                            *high_pulses += 1;
                        }
                    }
                }
                ModuleType::FlipFlop(b) => {
                    if !pulse {
                        *b = !*b;
                        for dest in &module.dests {
                            queue.push_back((module.name.clone(), dest.clone(), *b));
                            if *b {
                                *high_pulses += 1;
                            } else {
                                *low_pulses += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    return false;
}

#[derive(Debug, Clone)]
struct Module {
    mtype: ModuleType,
    name: String,
    dests: Vec<String>,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}
