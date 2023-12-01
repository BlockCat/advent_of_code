use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Valve = u8;
type ValveMap = HashMap<Valve, (usize, Vec<(Valve, usize)>)>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers.clone()));
    // println!("Exercise 3: {}", exercise_3(numbers.clone()));
    // println!("Exercise 4: {}", exercise_4(numbers.clone()));
}

fn input() -> (Valve, ValveMap) {
    let valves = include_str!("../input/day_16.txt")
        .lines()
        .map(parse_line)
        .map(|(a, b, c)| (a, (b, c)))
        .collect::<Vec<_>>();

    let translator = valves
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a.clone().0, i as Valve))
        .collect::<HashMap<_, _>>();

    println!("{:?}", translator);

    let valves = valves
        .into_iter()
        .map(|(v, (flow, ne))| {
            (
                translator[&v],
                (
                    flow,
                    ne.into_iter().map(|n| translator[&n]).collect::<Vec<_>>(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let start = translator[&"AA".to_string()];

    let valves = valves
        .iter()
        .map(|(valve, (flow, _))| {
            let neighbours = all_shortest(*valve, &valves);

            (*valve, (*flow, neighbours))
        })
        .collect::<HashMap<_, _>>();

    (start, valves)
}

fn parse_line(line: &str) -> (String, usize, Vec<String>) {
    let mut parts = line.split(' ');

    let valve = String::from(parts.nth(1).unwrap());
    let flow_rate = {
        let s = parts.nth(2).unwrap();
        s[5..s.len() - 1].parse().unwrap()
    };

    let start = parts.skip(4);

    let target_valves = start.map(|x| x.replace(',', "")).collect();

    // println!("{}, {}, {:?}", valve, flow_rate, target_valves);
    (valve, flow_rate, target_valves)
}

fn all_shortest(
    source: Valve,
    valves: &HashMap<Valve, (usize, Vec<Valve>)>,
) -> Vec<(Valve, usize)> {
    let mut queue = VecDeque::new();

    let mut help = HashMap::new();

    queue.push_back((source, 0usize));

    while help.len() != valves.len() {
        let (valve, dist) = queue.pop_front().unwrap();

        if help.contains_key(&valve) {
            continue;
        }
        help.insert(valve, dist);

        valves[&valve].1.iter().for_each(|x| {
            queue.push_back((*x, dist + 1));
        });
    }

    help.into_iter().filter(|x| valves[&x.0].0 > 0).collect()
}

fn exercise_1((start, valves): (Valve, ValveMap)) -> usize {
    let mut queue: BinaryHeap<(usize, Vec<(usize, u8)>, u32)> = BinaryHeap::new();

    queue.push((0usize, vec![(31usize, start)], 0));

    let max = fun_name(queue, valves, start);

    max
}

fn exercise_2((start, valves): (Valve, ValveMap)) -> usize {
    let mut queue: BinaryHeap<(usize, Vec<(usize, u8)>, u32)> = BinaryHeap::new();

    queue.push((0usize, vec![(27usize, start), (27, start)], 0));

    let max = fun_name(queue, valves, start);

    max
}

fn exercise_3((start, valves): (Valve, ValveMap)) -> usize {
    let mut queue: BinaryHeap<(usize, Vec<(usize, u8)>, u32)> = BinaryHeap::new();

    queue.push((0usize, vec![(23usize, start), (23, start), (23, start)], 0));

    let max = fun_name(queue, valves, start);

    max
}

fn exercise_4((start, valves): (Valve, ValveMap)) -> usize {
    let mut queue: BinaryHeap<(usize, Vec<(usize, u8)>, u32)> = BinaryHeap::new();

    queue.push((
        0usize,
        vec![(19usize, start), (19, start), (19, start), (19, start)],
        0,
    ));

    let max = fun_name(queue, valves, start);

    max
}

fn fun_name(
    mut queue: BinaryHeap<(usize, Vec<(usize, u8)>, u32)>,
    valves: HashMap<u8, (usize, Vec<(u8, usize)>)>,
    start: u8,
) -> usize {
    let mut max = 0;
    let mut visited = HashSet::new();
    while let Some((dist, mut agents, open)) = queue.pop() {
        agents.sort();

        let (time_b, valve_b) = agents.first().unwrap().clone();
        let (time_a, valve_a) = agents.last().unwrap().clone();

        assert!(time_a >= time_b);

        let (flow_a, n_a) = &valves[&valve_a];
        let open: u32 = open | (1 << valve_a);
        let pressure = (time_a - 1) * flow_a;

        let next_pressure = dist + pressure;

        max = next_pressure.max(max);
        if time_a == 0 && time_b == 0 {
            continue;
        }

        if !visited.insert((open, agents.clone())) {
            continue;
        }
        // if !visited.insert((time_b, valve_b, time_a, valve_a, open)) {
        //     println!("{:?}", agents);
        //     continue;
        // }

        // let max_sum: usize = valves
        //     .iter()
        //     .filter(|a| (open & (1 << a.0)) == 0)
        //     .map(|x| x.1 .0)
        //     .sum();

        // if dist + max_sum * (time_a) < max {
        //     continue;
        // }

        let next_neighbours = n_a
            .iter()
            .filter(|a| (open & (1 << a.0)) == 0)
            .filter(|a| agents.iter().all(|x| a.0 != x.1))
            .filter(|a| time_a > a.1 + 1)
            .collect::<Vec<_>>();

        let mut head = agents[0..agents.len() - 1].to_vec();

        if next_neighbours.is_empty() {
            head.push((0, start));
            assert_eq!(head.len(), agents.len());
            queue.push((next_pressure, head, open));
            continue;
        }

        for (a, dist_a) in next_neighbours {
            let mut head = head.clone();
            head.push((time_a - *dist_a - 1, *a));
            assert_eq!(head.len(), agents.len());

            queue.push((next_pressure, head, open));
        }
    }
    max
}
