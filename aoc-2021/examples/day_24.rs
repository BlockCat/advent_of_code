use std::collections::HashSet;

struct HashKey {
    add_x: isize,
    add_y: isize,
    div: isize,
}

impl HashKey {
    const fn new(add_x: isize, add_y: isize, div: isize) -> Self {
        Self { add_x, add_y, div }
    }
}

const INSTRUCTIONS: [HashKey; 14] = [
    HashKey::new(13, 3, 1),
    HashKey::new(11, 12, 1),
    HashKey::new(15, 9, 1),
    HashKey::new(-6, 12, 26),
    HashKey::new(15, 2, 1),
    HashKey::new(-8, 1, 26),
    HashKey::new(-4, 1, 26),
    HashKey::new(15, 13, 1),
    HashKey::new(10, 1, 1),
    HashKey::new(11, 6, 1),
    HashKey::new(-11, 2, 26),
    HashKey::new(0, 11, 26),
    HashKey::new(-8, 10, 26),
    HashKey::new(-7, 3, 26),
];

pub fn main() {
    println!("Ex1: {}", exercise_1());
    println!("Ex2: {}", exercise_2());
}

fn exercise_1() -> usize {
    let mut seen = vec![HashSet::new(); 14];
    let order = (1..=9).rev().collect::<Vec<_>>();
    let mut a = handle(0, 0, &order, &mut seen).unwrap();
    a.reverse();

    a.into_iter().fold(0, |acc, x| acc * 10 + x as usize)
}

fn exercise_2() -> usize {
    let mut seen = vec![HashSet::new(); 14];
    let order = (1..=9).collect::<Vec<_>>();
    let mut a = handle(0, 0, &order, &mut seen).unwrap();
    a.reverse();

    a.into_iter().fold(0, |acc, x| acc * 10 + x as usize)
}

fn handle(
    hash: isize,
    depth: usize,
    order: &[isize],
    seen: &mut [HashSet<isize>],
) -> Option<Vec<isize>> {
    let instruction = &INSTRUCTIONS[depth];
    for input in order {
        let hash = calculate(
            *input,
            hash,
            instruction.add_x,
            instruction.add_y,
            instruction.div,
        );
        if depth == 13 {
            if hash == 0 {
                return Some(vec![*input]);
            }
        } else {
            if !seen[depth].insert(hash) {
                continue;
            }

            if let Some(mut r) = handle(hash, depth + 1, order, seen) {
                r.push(*input);
                return Some(r);
            }
        }
    }
    None
}

fn calculate(input: isize, hash: isize, add_x: isize, add_y: isize, div: isize) -> isize {
    let x = (hash % 26) + add_x;
    let hash = hash / div;

    if x == input {
        return hash;
    } else {
        return hash * 26 + input + add_y;
    }
}
