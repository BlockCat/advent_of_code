#![feature(test, portable_simd)]
use std::{ simd::{i16x16, u16x16} };

const ONES: i16x16 = i16x16::from_array([0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

pub fn main() {
    let numbers = include_str!("../input/day03.txt")
        .lines()
        .map(decode_binary)
        .collect::<Vec<_>>();

    println!("ex1: {}", exercise_1(&numbers));
    println!("ex2: {}", exercise_2(&numbers));
}

fn decode_binary(line: &str) -> u16x16 {

    let prepend = 16 - line.len();
    let mut values = vec![0u16; prepend];
    values.extend(line.chars().map(|c| match c {
        '1' => 1,
        '0' => 0,
        _ => unreachable!(),
    }));
    u16x16::from_slice(&values)
}

fn exercise_1(numbers: &Vec<u16x16>) -> usize {
    let number_len = numbers.len() as u16;
    let number_len = u16x16::splat(number_len / 2);

    let sum = count_lanes(&numbers);

    let a = (-sum.lanes_le(number_len).to_int()) & ONES;    
    let b = a ^ ONES;

    i16x16_to_u16(a) * i16x16_to_u16(b)
}

fn count_lanes(numbers: &[u16x16]) -> u16x16 {
    numbers.iter().fold(u16x16::splat(0), |acc, x| acc + x)
}

fn exercise_2(numbers: &Vec<u16x16>) -> usize {
    let a = find_value(
        numbers.clone(),
        |zeros, ones| (zeros <= ones) as u16,
    );
    let b = find_value(
        numbers.clone(),
        |zeros, ones| (zeros > ones) as u16,
    );    
    a * b
}

fn find_value<T>(mut numbers: Vec<u16x16>, pred: T) -> usize
where
    T: Fn(u16, u16) -> u16,
{
    let mut counter = 4;
    while numbers.len() > 1 {
        let ones = count_lanes(&numbers);
        let zeros = u16x16::splat(numbers.len() as u16) - ones;

        let search = pred(zeros[counter], ones[counter]);
        numbers = numbers
            .into_iter()
            .filter(|&a| a[counter] == search)
            .collect();
        counter += 1;
    }
    u16x16_to_u16(numbers[0])
}

fn u16x16_to_u16(a: u16x16) -> usize {
    let a = a
        .to_array()
        .into_iter()
        .map(|a| a.to_string())
        .collect::<String>();
    u16::from_str_radix(&a, 2).unwrap() as usize
}

fn i16x16_to_u16(a: i16x16) -> usize {
    let a = a
        .to_array()
        .into_iter()
        .map(|a| a.to_string())
        .collect::<String>();
    u16::from_str_radix(&a, 2).unwrap() as usize
}
