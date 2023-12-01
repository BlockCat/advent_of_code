use std::iter::Iterator;
use std::collections::HashMap;

fn sequence(number: u32) -> i32 {
    let (dx, dy) = SequenceIterator::new()
        .take((number - 1) as usize)
        .fold((0i32, 0i32), |(ox, oy), (nx, ny)| {
            (ox + nx, oy + ny)
        });

    dx.abs() + dy.abs()
}


fn stress_test(number: u32) -> u32 {
    let mut map: HashMap<(i32, i32), usize> = HashMap::new();

    let mut iterator = SequenceIterator::new();
    let (mut px, mut py) = (0i32, 0i32);

    map.insert((0, 0), 1);

    while let Some((dx, dy)) = iterator.next() {
        px += dx;
        py += dy;

        let mut sum = 0usize;

        for i in -1..=1 {
            for j in -1..=1 {
                if let Some(v) = map.get(&(px + i, py + j)) {
                    sum += v;
                }
            }
        }

        //println!("{}, {}: {}", px, py, sum);

        map.insert((px, py), sum);

        if sum > number as usize {
            return sum as u32;
        }
    }

    0
}

fn read_input(input: &str) -> u32 {
    input.parse().unwrap()
}


struct SequenceIterator {
    number: u32,
    counter1: u32,
    counter2: u32,
    positive: bool
}

impl SequenceIterator {
    fn new() -> SequenceIterator {
        SequenceIterator {
            number: 1,
            counter1: 1,
            counter2: 1,
            positive: true
        }
    }
}

impl Iterator for SequenceIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {        

        if self.counter1 == 0 && self.counter2 == 0 {
            self.number += 1;
            self.counter1 = self.number;
            self.counter2 = self.number;
            self.positive = !self.positive;
        }

        let number: i32 = if self.positive {
            1
        } else {
            -1
        };

        if self.counter1 > 0 {
            self.counter1 -= 1;
            Some((number, 0))
        } else if self.counter2 > 0 {
            self.counter2 -= 1;
            Some((0, -number))
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test_sequence() {
    for (a, b) in SequenceIterator::new().take(20) {
        println!("{}\t{}", a, b);
    }    
}

#[test]
fn test_example() {
    assert_eq!(sequence(1), 0);
    assert_eq!(sequence(12), 3);
    assert_eq!(sequence(23), 2);
    assert_eq!(sequence(1024), 31);
}

#[test]
fn test_example2() {
    assert_eq!(stress_test(1), 2);
    assert_eq!(stress_test(12), 23);
    assert_eq!(stress_test(23), 25);
    assert_eq!(stress_test(147), 304);
}

#[test]
fn run_input() {
    println!("{}", sequence(325489));
    println!("{}", stress_test(325489));
}