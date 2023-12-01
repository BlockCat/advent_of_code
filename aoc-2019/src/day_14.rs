use crate::test::Bencher;

use hashbrown::HashMap;

#[derive(Clone, Debug)]
struct Reaction {
    pub input: Vec<(u64, String)>,
    pub output: String,
    pub output_num: u64,
}

impl Reaction {
    fn is_ore(&self) -> bool {
        self.input.len() == 1 && &self.input[0].1 == "ORE"
    }
}

type Reactions = HashMap<String, Reaction>;

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day14.txt")); // 1239890 too high
    println!("ex1: {}", exercise_1(&input.0, &input.1, 1));
    println!("ex2: {}", exercise_2(&input));
}

const TOPOLOGY_PROCESSED: u8 = 1;
const TOPOLOGY_IN_STACK: u8 = 2;

fn exercise_1(reactions: &Reactions, list: &Vec<String>, target: u64) -> u64 {
    let mut mmap = HashMap::new();
    mmap.insert(String::from("FUEL"), target);
    for i in list {
        let reaction = &reactions[i]; // The reaction to create the current chemical
        let amount_needed = mmap[i]; // Amount of chemicals that need to be created

        let remaining_chemicals = amount_needed % reaction.output_num; // Chemicals remaining after reactions
        let reactions_needed =
            (amount_needed / reaction.output_num) + (remaining_chemicals > 0) as u64; // Reactions needed to create required amount and remaining chemicals

        for (amount, child_chemical) in &reaction.input {
            *mmap.entry(child_chemical.clone()).or_insert(0) += reactions_needed * amount;            
        }
    }
    mmap["ORE"]
}

fn exercise_2((reactions, list): &(Reactions, Vec<String>)) -> u64 {
    // Is this a max flow or something?
    let search: u64 = 1_000_000_000_000;

    // increase
    let mut it = 1;
    while exercise_1(&reactions, &list, it) < search {
        it *= 2;
    }
    // it's between [it / 2; it) inclusive

    let mut down = it / 2;
    let mut up = it;
    let mut mid = (down + up) / 2;

    loop {
        if mid == down {
            return mid;
        }
        let result = exercise_1(&reactions, &list, mid);

        if result < search {
            down = mid;
        } else {
            up = mid;
        }
        mid = (down + up) / 2;
    }
}

fn read_input(input: &str) -> (Reactions, Vec<String>) {    
    enum Status {
        Visisted(String),
        Unvisited(String),
    }

    let reactions = input
        .lines()
        .map(read_line)
        .map(|x| (x.output.clone(), x))
        .collect::<HashMap<_, _>>();

    let mut stack = Vec::new();
    stack.push(Status::Unvisited("FUEL".to_string()));

    let mut node_state: HashMap<String, u8> =
        reactions.iter().map(|x| (x.0.clone(), 0u8)).collect();
    node_state.insert("ORE".to_string(), 0);

    let mut list = Vec::new();

    while let Some(current_node) = stack.pop() {
        match current_node {
            Status::Unvisited(current_node) => {
                if node_state[&current_node] == 0 {
                    *node_state.entry(current_node.clone()).or_insert(0) |= TOPOLOGY_IN_STACK;
                    if let Some(predecessors) = reactions.get(&current_node).map(|x| &x.input) {
                        stack.reserve(predecessors.len() + 1);
                        stack.push(Status::Visisted(current_node));
                        stack.extend(predecessors.iter().map(|x| Status::Unvisited(x.1.clone())));
                    }
                }
            }
            Status::Visisted(current_node) => {
                if node_state[&current_node[..]] & TOPOLOGY_PROCESSED == 0 {
                    node_state.insert(current_node.clone(), TOPOLOGY_PROCESSED);
                    list.push(current_node);
                }
            }
        }
    }

    list.reverse();

    (reactions, list)
}

fn read_line(line: &str) -> Reaction {
    let mut line = line.split(" => ");

    let inputs = line
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            let mut c = x.trim().split(' ');
            (
                c.next().unwrap().parse::<u64>().unwrap(),
                c.next().unwrap().to_string(),
            )
        })
        .collect::<Vec<_>>();
    let outputs = line
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| {
            let mut c = x.split(' ');
            (
                (c.next().unwrap().parse::<u64>().unwrap()),
                c.next().unwrap().to_string(),
            )
        })
        .next()
        .unwrap();

    Reaction {
        input: inputs,
        output: outputs.1,
        output_num: outputs.0,
    }
}

#[test]
fn d14_test() {
    mini_test(
        r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        165,
    );
    mini_test_2(
        r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        13312,
        82892753,
    );

    mini_test_2(
        r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        180697,
        5586022,
    );

    mini_test_2(
        r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        2210736,
        460664,
    );

    mini_test_2(include_str!("input/day14.txt"), 1185296, 1376631)
}

fn mini_test(input: &str, expected: u64) {
    let input = read_input(input);
    assert_eq!(exercise_1(&input.0, &input.1, 1), expected);
}

fn mini_test_2(input: &str, expected: u64, expected_2: u64) {
    let input = read_input(input);
    assert_eq!(exercise_1(&input.0, &input.1, 1), expected);
    assert_eq!(exercise_2(&input), expected_2);
}

#[bench]
fn d14_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day14.txt"));
    b.iter(|| exercise_1_2(&input.0, &input.1, 1));
}

#[bench]
fn d14_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day14.txt"));
    b.iter(|| exercise_2(&input));
}

#[bench]
fn d14_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day14.txt")));
}
