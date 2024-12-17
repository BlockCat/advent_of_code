use aoc_2024::stopwatch;
use std::{collections::HashMap, usize};

type Input = CPU;

#[derive(Debug, Clone)]
struct CPU {
    register: [usize; 3],
    program: Vec<usize>,
}

pub fn main() {
    let real_input = CPU {
        register: [59590048, 0, 0],
        program: vec![2, 4, 1, 5, 7, 5, 0, 3, 1, 6, 4, 3, 5, 5, 3, 0],
    };
    let numbers = real_input;

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn exercise_1(input: &Input) -> String {
    let numbers = decompiled(input.register[0]);

    numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn bits_minus_3_overlap(left: usize, right: usize) -> bool {
    let mask = 0b1_111_111_111_111;

    let left = left << 3;
    let together = left ^ right;
    let together = together & mask;
    let together = together >> 3;

    together == 0
}

#[test]
fn help() {
    assert!(bits_minus_3_overlap(
        0b0_001_111_111_111,
        0b1_111_111_111_001
    ));
}

fn exercise_2(input: &Input) -> usize {
    let program = input.program.clone();

    let mut numbers = HashMap::<usize, Vec<usize>>::new();

    println!("0..{}", 0b1_111_111_111_111);
    for i in 1..0b1_111_111_111_111 {
        let n = decompiled(i);
        numbers.entry(n[0]).or_default().push(i);
    }

    // Program: 2,4,1,5,7,5,0,3,1,6,4,3,5,5,3,0
    let mut min = usize::MAX;
    for &a in &numbers[&2] {
        for &b in &numbers[&4] {
            if !bits_minus_3_overlap(b, a) {
                continue;
            }
            for &c in &numbers[&1] {
                if !bits_minus_3_overlap(c, b) {
                    continue;
                }
                for &d in &numbers[&5] {
                    if !bits_minus_3_overlap(d, c) {
                        continue;
                    }
                    for &e in &numbers[&7] {
                        if !bits_minus_3_overlap(e, d) {
                            continue;
                        }
                        for &f in &numbers[&5] {
                            if !bits_minus_3_overlap(f, e) {
                                continue;
                            }
                            for &g in &numbers[&0] {
                                if !bits_minus_3_overlap(g, f) {
                                    continue;
                                }
                                for &h in &numbers[&3] {
                                    if !bits_minus_3_overlap(h, g) {
                                        continue;
                                    }
                                    for &i in &numbers[&1] {
                                        if !bits_minus_3_overlap(i, h) {
                                            continue;
                                        }
                                        for &j in &numbers[&6] {
                                            if !bits_minus_3_overlap(j, i) {
                                                continue;
                                            }
                                            for &k in &numbers[&4] {
                                                if !bits_minus_3_overlap(k, j) {
                                                    continue;
                                                }
                                                for &l in &numbers[&3] {
                                                    if !bits_minus_3_overlap(l, k) {
                                                        continue;
                                                    }
                                                    for &m in &numbers[&5] {
                                                        if !bits_minus_3_overlap(m, l) {
                                                            continue;
                                                        }
                                                        for &n in &numbers[&5] {
                                                            if n & 0b111_111_111 != n {
                                                                continue;
                                                            }
                                                            if !bits_minus_3_overlap(n, m) {
                                                                continue;
                                                            }
                                                            for &o in &numbers[&3] {
                                                                if o & 0b111_111 != o {
                                                                    continue;
                                                                }
                                                                if !bits_minus_3_overlap(o, n) {
                                                                    continue;
                                                                }
                                                                for &p in &numbers[&0] {
                                                                    if p & 0b111 != p {
                                                                        continue;
                                                                    }
                                                                    if !bits_minus_3_overlap(p, o) {
                                                                        continue;
                                                                    }

                                                                    let number = a                                                                        
                                                                        | d << 9
                                                                        | h << 21
                                                                        | l << 33
                                                                        | p << 45;

                                                                    min = min.min(number);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    min
}

fn decompiled(a: usize) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut a = a;

    while a != 0 {
        let mut b = (a ^ 0b101) & 0b111;
        let c = (a >> b) ^ 0b110;

        a = a >> 3;

        b = b ^ c;

        numbers.push(b & 0b111);
    }

    numbers
}
