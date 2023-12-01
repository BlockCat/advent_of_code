use std::collections::{HashMap, HashSet, VecDeque};

type Invoer = (Vec<(Grot, Grot)>, u8);

pub fn main() {
    let invoer = vertaal_invoer(include_str!("../input/test.txt"));
    let start = std::time::SystemTime::now();

    println!("{:?}", invoer);

    println!("O1: {}", oefening_1(&invoer));
    let end = std::time::SystemTime::now();    
    let dur = end.duration_since(start).unwrap();

    println!("Took: {:?}", dur);

    let start = std::time::SystemTime::now();
    println!("O2: {}", oefening_2(&invoer));
    let end = std::time::SystemTime::now();    
    let dur = end.duration_since(start).unwrap();
    
    println!("Took: {:?}", dur);
}

fn vertaal_invoer(invoer: &str) -> Invoer {
    let matches = invoer
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            let links = split.next().unwrap();
            let rechts = split.next().unwrap();
            (links, rechts)
        })
        .collect::<Vec<_>>();

    let mut small_caves: HashSet<&str> = matches
        .iter()
        .flat_map(|(l, r)| [*l, *r])
        .filter(|x| x.to_uppercase() != *x)
        .collect::<HashSet<_>>();
    let grote_caves: HashSet<&str> = matches
        .iter()
        .flat_map(|(l, r)| [*l, *r])
        .filter(|x| x.to_uppercase() == *x)
        .collect::<HashSet<_>>();

    let start_id = 0;
    let end_id = (small_caves.len() - 1) as u8;

    small_caves.remove("start");
    small_caves.remove("end");

    let mut kleine_mapping: HashMap<&str, u8> = small_caves
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a as u8 + 1u8))
        .collect();
    let grote_mapping: HashMap<&str, u8> = grote_caves
        .into_iter()
        .enumerate()
        .map(|(a, b)| (b, a as u8))
        .collect();

    kleine_mapping.insert("start", start_id);
    kleine_mapping.insert("end", end_id);

    println!("{:?}", kleine_mapping);
    println!("{:?}", grote_mapping);

    (
        matches
            .iter()
            .map(|(links, rechts)| {
                let links = if links.to_uppercase() == *links {
                    Grot::Groot(grote_mapping[links])
                } else {
                    Grot::Klein(kleine_mapping[links])
                };
                let rechts = if rechts.to_uppercase() == *rechts {
                    Grot::Groot(grote_mapping[rechts])
                } else {
                    Grot::Klein(kleine_mapping[rechts])
                };

                (links, rechts)
            })
            .collect(),
        end_id,
    )
}

fn oefening_1((invoer, end_id): &Invoer) -> usize {
    let connecties = vindt_verbindingen(invoer);
    let mut rij: VecDeque<(Grot, BitSet)> = VecDeque::new();
    rij.push_front((Grot::Klein(0), BitSet::default()));

    let mut gezien = 0;

    while let Some((grot, bezocht)) = rij.pop_front() {
        for buurman in &connecties[&grot] {
            match buurman {
                Grot::Groot(_) => rij.push_front((buurman.clone(), bezocht.clone())),
                Grot::Klein(c) => {
                    if c == end_id {
                        gezien += 1;
                    } else if !bezocht.get(*c) {
                        let mut bezocht = bezocht.clone();
                        bezocht.set(*c);
                        rij.push_front((buurman.clone(), bezocht));
                    }
                }
            }
        }
    }

    gezien
}

fn oefening_2((invoer, end_id): &Invoer) -> usize {
    let connecties = vindt_verbindingen(invoer);
    let mut rij: VecDeque<(Grot, BitSet)> = VecDeque::new();
    rij.push_front((Grot::Klein(0), BitSet::default()));

    let mut gezien = 0;

    while let Some((grot, bezocht)) = rij.pop_front() {
        for buurman in &connecties[&grot] {
            match buurman {
                Grot::Groot(_) => rij.push_front((buurman.clone(), bezocht.clone())),
                Grot::Klein(c) => {
                    if c == end_id {
                        gezien += 1;
                    } else if !bezocht.is_bezocht() && *c != 0 && bezocht.get(*c) {
                        let mut bezocht = bezocht.clone();
                        bezocht.set_bezocht();
                        rij.push_front((buurman.clone(), bezocht));
                    } else if !bezocht.get(*c) {
                        let mut bezocht = bezocht.clone();
                        bezocht.set(*c);
                        rij.push_front((buurman.clone(), bezocht));
                    }
                }
            }
        }
    }

    gezien
}

fn vindt_verbindingen(invoer: &Vec<(Grot, Grot)>) -> HashMap<Grot, Vec<Grot>> {
    let mut connecties: HashMap<Grot, Vec<Grot>> = HashMap::new();
    for (links, rechts) in invoer {
        connecties
            .entry(links.clone())
            .or_insert(Vec::new())
            .push(rechts.clone());
        connecties
            .entry(rechts.clone())
            .or_insert(Vec::new())
            .push(links.clone());
    }
    connecties
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Grot {
    Groot(u8),
    Klein(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BitSet(u64);

impl BitSet {
    fn get(&self, index: u8) -> bool {
        (self.0 >> index & 1) == 1
    }
    fn set(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    fn is_bezocht(&self) -> bool {
        self.get(63)
    }
    fn set_bezocht(&mut self) {
        self.set(63)
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self(1)
    }
}
