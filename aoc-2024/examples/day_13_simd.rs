#![feature(portable_simd)]

use core::simd;
use std::ops::Not;
use std::ops::Rem;
use std::simd::cmp::SimdPartialEq;
use std::simd::num::SimdInt;

use aoc_2024::grouped_lines;
use aoc_2024::stopwatch;

type Input = ButtonSet;

pub fn main() {
    let numbers = input(include_str!("../input/day_13.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    let mut set = ButtonSet::default();
    grouped_lines(input).for_each(|mut x| {
        let ba = parse_line(x.next().unwrap());
        let bb = parse_line(x.next().unwrap());
        let prize = parse_line(x.next().unwrap());

        set.button_a_x.push(ba.0);
        set.button_a_y.push(ba.1);

        set.button_b_x.push(bb.0);
        set.button_b_y.push(bb.1);

        set.price_x.push(prize.0);
        set.price_y.push(prize.1);
    });

    set
}

fn parse_line(b: &str) -> (i64, i64) {
    let offsets = &b[b.find(':').unwrap() + 4..];
    let offset_komma = offsets.find(',').unwrap();

    let x = &offsets[..offset_komma];
    let y = &offsets[offset_komma + 4..];

    let x = x.parse().unwrap();
    let y = y.parse().unwrap();

    (x, y)
}

fn exercise_1(input: &Input) -> usize {
    let (a, b) = lowest_cost(input, 0);
    a * 3 + b
}
fn exercise_2(input: &Input) -> usize {
    // 10000000000000
    let (a, b) = lowest_cost(input, 10000000000000);
    a * 3 + b
}

fn lowest_cost(set: &ButtonSet, offset: i64) -> (usize, usize) {
    // let prize_x = offset + set.prize.0;
    // let prize_y = offset + set.prize.1;

    // let a1 = prize_x as isize;
    // let a2 = prize_y as isize;
    // let b1 = set.button_a.0 as isize;
    // let b2 = set.button_a.1 as isize;
    // let c1 = set.button_b.0 as isize;
    // let c2 = set.button_b.1 as isize;

    let mut a = 0;
    let mut b = 0;

    let size = 64;
    type SimdLanes = simd::i64x64;

    let zeros = SimdLanes::splat(0);
    let offset = SimdLanes::splat(offset);

    for i in 0..=(set.button_a_x.len() / size) {
        let start = i * size;
        let end = ((i + 1) * size).min(set.price_x.len());

        if end == start {
            break;
        }

        let a1 = SimdLanes::load_or_default(&set.price_x[start..end]) + offset;
        let a2 = SimdLanes::load_or_default(&set.price_y[start..end]) + offset;

        let b1 = SimdLanes::load_or_default(&set.button_a_x[start..end]);
        let b2 = SimdLanes::load_or_default(&set.button_a_y[start..end]);

        let c1 = SimdLanes::load_or_default(&set.button_b_x[start..end]);
        let c2 = SimdLanes::load_or_default(&set.button_b_y[start..end]);

        let inv_determinant = b1 * c2 - b2 * c1;

        let mask = inv_determinant.simd_eq(zeros).not();

        let inv_determinant = inv_determinant - mask.not().to_int();

        let atop = a1 * c2 - a2 * c1;
        let btop = a2 * b1 - a1 * b2;

        let remainders_a = atop.rem(inv_determinant);
        let remainders_b = btop.rem(inv_determinant);

        let mask = remainders_a.simd_eq(zeros) & remainders_b.simd_eq(zeros) & mask;

        let presses_a = mask.select(atop / inv_determinant, zeros);
        let presses_b = mask.select(btop / inv_determinant, zeros);

        a += presses_a.reduce_sum();
        b += presses_b.reduce_sum();
    }

    (a as usize, b as usize)

    // [a1] = [b1, c1] * [x1]
    // [a2] = [b2, c2] * [x2]
    // \vec{a} = M * \vec{x}
    // \vec{x} = M^-1 * \vec{a}

    // [x1] = [c2, -c1]           * [a1]
    // [x2] = [-b2, b1] / inv_det * [a2]

    // let inv_determinant = b1 * c2 - b2 * c1;

    // let (a, ra) = (a1 * c2 - a2 * c1).div_rem_euclid(&inv_determinant);
    // let (b, rb) = (a2 * b1 - a1 * b2).div_rem_euclid(&inv_determinant);

    // if ra == 0 && rb == 0 {
    //     Some((a as usize, b as usize))
    // } else {
    //     None
    // }
}

#[derive(Debug, Default)]
struct ButtonSet {
    button_a_x: Vec<i64>,
    button_a_y: Vec<i64>,

    button_b_x: Vec<i64>,
    button_b_y: Vec<i64>,

    price_x: Vec<i64>,
    price_y: Vec<i64>,
}
