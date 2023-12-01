#![feature(iter_intersperse)]

use aoc_2021::stopwatch;
use std::{
    collections::HashMap,
    iter::{Cycle, Skip},
    ops::RangeInclusive,
};

type Input = (usize, usize);

const DISTRIBUTION: [usize; 7] = [
    1, // 3
    3, // 4
    6, // 5
    7, // 6
    6, // 7
    3, // 8
    1, // 9
];

pub fn main() {
    let input = (7, 8);
    // let input = (4, 8);
    println!("Ex1: {}", exercise_1(&input));

    let d = stopwatch(|| {
        println!("Ex2: {}", exercise_2(&input));
    });

    println!("Duration: {:?}", d);
}

fn exercise_1(input: &Input) -> usize {
    let mut p1 = DicerIterator::new(input.0, 1);
    let mut p2 = DicerIterator::new(input.1, 4);

    p1.by_ref()
        .intersperse_with(|| p2.next().unwrap())
        .take_while(|x| x < &1000)
        .count();

    p1.score.min(p2.score) * (p1.thrown + p2.thrown)
}

fn exercise_2(input: &Input) -> usize {
    let mut map = HashMap::new();

    map.insert((0, input.0 - 1, 0, input.1 - 1), 1);

    let mut p1 = 0;
    let mut p2 = 0;

    while !map.is_empty() {
        let res = round(map);
        map = res.0;
        p1 += res.1;
        p2 += res.2;
    }
    p1.max(p2)
}

fn round(
    map: HashMap<(usize, usize, usize, usize), usize>,
) -> (HashMap<(usize, usize, usize, usize), usize>, usize, usize) {
    let mut p1_win_sum = 0;
    let mut p2_win_sum = 0;

    let mut nmap = HashMap::new();
    for ((score_1, pos_1, score_2, pos_2), founds) in map {
        let p1_collect = collect_state(pos_1, score_1);
        let p2_collect = collect_state(pos_2, score_2);

        for (p1_new_score, p1_new_pos, p1_dist, p1_win) in p1_collect.iter() {
            if let Some(win) = p1_win {
                p1_win_sum += *win * founds;
            } else {
                for (p2_new_score, p2_new_pos, p2_dist, p2_win) in p2_collect.iter() {
                    if let Some(win) = p2_win {
                        p2_win_sum += win * founds;
                    } else {
                        *nmap
                            .entry((*p1_new_score, *p1_new_pos, *p2_new_score, *p2_new_pos))
                            .or_insert(0) += *p1_dist * *p2_dist * founds;
                    }
                }
            }
        }
    }
    (nmap, p1_win_sum, p2_win_sum)
}

fn collect_state(pos: usize, score: usize) -> Vec<(usize, usize, usize, Option<usize>)> {
    let p1_collect = (0..7)
        .map(|i| {
            let point = i + 3;
            let dist = DISTRIBUTION[i];
            let pos_new = (pos + point) % 10;
            let score_new = score + pos_new + 1;

            if score_new >= 21 {
                (score_new, pos_new, dist, Some(dist))
            } else {
                (score_new, pos_new, dist, None)
            }
        })
        .collect::<Vec<_>>();
    p1_collect
}

struct DicerIterator {
    score: usize,
    space: Skip<Cycle<RangeInclusive<usize>>>,
    dice: Skip<Cycle<RangeInclusive<usize>>>,
    thrown: usize,
}

impl DicerIterator {
    pub fn new(start: usize, dice: usize) -> Self {
        let dice = (1..=100usize).cycle().skip(dice - 1);
        let space = (1..=10).cycle().skip(start);
        DicerIterator {
            score: 0,
            thrown: 0,
            dice,
            space,
        }
    }
}

impl Iterator for DicerIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let score = self.dice.by_ref().take(3).collect::<Vec<_>>();
        self.dice.next();
        self.dice.next();
        self.dice.next();

        self.thrown += 3;

        let add = self
            .space
            .by_ref()
            .nth(score.iter().sum::<usize>() - 1)
            .unwrap();

        self.score += add;

        Some(self.score)
    }
}
