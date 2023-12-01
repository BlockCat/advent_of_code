use hashbrown::{HashMap, HashSet};
use itertools::*;
use utils::Vector2;

#[derive(Debug)]
pub enum Instruction {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day14.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

pub fn read_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|x| {
            let mut it = x.split(" = ");
            if x.starts_with("mask") {
                let (ones, zeros, xes) = it.skip(1).next().unwrap().trim().chars().fold(
                    (0u64, 0u64, 0u64),
                    |(ones, zeros, xes), x| match x {
                        'X' => (ones << 1, zeros << 1, (xes << 1) | 1),
                        '1' => ((ones << 1) | 1, zeros << 1, xes << 1),
                        '0' => (ones << 1, (zeros << 1) | 1, xes << 1),
                        _ => unreachable!(x),
                    },
                );
                Instruction::Mask(ones, zeros, xes)
            } else {
                let mem = it.next().unwrap();
                let num = mem[4..mem.len() - 1].parse().unwrap();
                let val = it.next().unwrap().parse().unwrap();
                Instruction::Mem(num, val)
            }
        })
        .collect()
}

fn exercise_1(input: &Vec<Instruction>) -> u64 {
    let mut mask = (0, 0);
    let mut map = HashMap::new();

    for instruction in input {
        match instruction {
            Instruction::Mask(ones, zeros, _) => {
                mask = (!*zeros, *ones);
            }
            Instruction::Mem(key, val) => {
                map.insert(*key, (*val & mask.0) | mask.1);
            }
        }
    }

    map.values().sum()
}

fn exercise_2(input: &Vec<Instruction>) -> u64 {
    let mut mask = (0, 0, 0);
    let mut map = HashMap::new();

    for instruction in input {
        match instruction {
            Instruction::Mask(ones, zeros, xes) => {
                mask = (!*zeros, *ones, *xes);
            }
            Instruction::Mem(key, val) => {
                for address in values(&mask, *key) {
                    map.insert(address, *val);
                }
            }
        }
    }

    map.values().sum()
}

fn values((_, ones, x): &(u64, u64, u64), address: u64) -> Vec<u64> {
    let masked_address = address | ones;
    let xones = x.count_ones() as usize;
    // let mut xmap = Vec::with_capacity(xones);

    let xmap = (0..64u64)
        .filter(|i| ((x >> i) & 1) == 1)
        .collect::<Vec<_>>();

    let addresses = (0..2usize.pow(xones as u32)).map(|number| {
        let mut current = number;
        let mut new_address = masked_address;
        for mapping in &xmap {
            let va = current & 1;
            new_address = if va == 0 {
                new_address & !(1 << mapping)
            } else if va == 1 {
                new_address | (1 << mapping)
            } else {
                unreachable!()
            };

            current /= 2;
        }

        new_address
    }).collect::<Vec<_>>();
   
    // assert!(addresses.len() == 2usize.pow(xones as u32));

    addresses
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d14ex1() {
        let r = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let i = read_input(r);
        println!("{:?}", i);

        assert_eq!(165, exercise_1(&i));
    }

    #[test]
    fn d14ex2() {
        let r = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let i = read_input(r);
        println!("{:?}", i);
        assert_eq!(208, exercise_2(&i));
    }

    //     #[bench]
    //     fn d13_bench_parse(b: &mut Bencher) {
    //         b.iter(|| read_input(include_str!("input/day13.txt")));
    //     }
    //     #[bench]
    //     fn d13_bench_ex1(b: &mut Bencher) {
    //         let input = read_input(include_str!("input/day13.txt"));
    //         b.iter(|| exercise_1(&input));
    //     }

    //     #[bench]
    //     fn d13_bench_ex2(b: &mut Bencher) {
    //         let input = read_input(include_str!("input/day13.txt"));
    //         b.iter(|| exercise_2(&input));
    //     }
}
