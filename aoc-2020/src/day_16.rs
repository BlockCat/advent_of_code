use hashbrown::{HashMap, HashSet};
use std::iter::FromIterator;
use utils::interval_tree::{Interval, IntervalTree};

use std::{collections::binary_heap::Iter, unimplemented};

struct Ticket {
    values: Vec<usize>,
}

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day16.txt"), 500);
    println!("{}", exercise_1(&input));
    // let input = read_input_hack(include_str!("input/day16.txt"));
    // println!("{}", exercise_2(&input, "departure"));
}

fn read_input(input: &str, center: usize) -> (IntervalTree<String>, Ticket, Vec<Ticket>) {
    let mut it = input.lines();
    let tree = it
        .by_ref()
        .take_while(|&x| !x.is_empty())
        .flat_map(|x| {
            let mut it1 = x.split(": ");
            let name = it1.next().unwrap().to_string();
            let mut it2 = it1.next().unwrap().split(" or ");
            let mut range1 = it2.next().unwrap().split("-");
            let mut range2 = it2.next().unwrap().split("-");
            let i1 = Interval {
                value: name.clone(),
                start: range1.next().unwrap().parse().unwrap(),
                end: range1.next().unwrap().parse().unwrap(),
            };
            let i2 = Interval {
                value: name.clone(),
                start: range2.next().unwrap().parse().unwrap(),
                end: range2.next().unwrap().parse().unwrap(),
            };
            vec![i1, i2]
        })
        .fold(IntervalTree::new(center), |mut acc, x| {
            acc.add(x);
            acc
        });

    let my_ticket = Ticket {
        values: it
            .by_ref()
            .skip_while(|&x| x != "your ticket:")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().expect(x))
            .collect(),
    };

    let other_tickets = it
        .skip_while(|&x| x != "nearby tickets:")
        .skip(1)
        .map(|x| Ticket {
            values: x.split(",").map(|x| x.parse().expect(x)).collect(),
        })
        .collect();

    (tree, my_ticket, other_tickets)
}

fn read_input_hack(input: &str) -> (Vec<(String, usize, usize)>, Ticket, Vec<Ticket>) {
    let mut it = input.lines();
    let tree = it
        .by_ref()
        .take_while(|&x| !x.is_empty())
        .flat_map(|x| {
            let mut it1 = x.split(": ");
            let name = it1.next().unwrap().to_string();
            let mut it2 = it1.next().unwrap().split(" or ");
            let mut range1 = it2.next().unwrap().split("-");
            let mut range2 = it2.next().unwrap().split("-");
            vec![
                (
                    name.clone(),
                    range1.next().unwrap().parse().unwrap(),
                    range1.next().unwrap().parse().unwrap(),
                ),
                (
                    name,
                    range2.next().unwrap().parse().unwrap(),
                    range2.next().unwrap().parse().unwrap(),
                ),
            ]
        })
        .collect::<Vec<_>>();

    // println!("{:?}", tree);

    let my_ticket = Ticket {
        values: it
            .by_ref()
            .skip_while(|&x| x != "your ticket:")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().expect(x))
            .collect(),
    };

    let other_tickets = it
        .skip_while(|&x| x != "nearby tickets:")
        .skip(1)
        .map(|x| Ticket {
            values: x.split(",").map(|x| x.parse().expect(x)).collect(),
        })
        .collect();

    (tree, my_ticket, other_tickets)
}

fn exercise_1((tree, _, tickets): &(IntervalTree<String>, Ticket, Vec<Ticket>)) -> usize {
    tickets
        .iter()
        .flat_map(|x| {
            x.values
                .iter()
                .filter(|&v| !tree.intersecting(*v).any(|x| true))
                // .inspect(|x| println!("{}", x))
                .map(|x| *x)
        })
        .sum()
}

fn exercise_2(
    (tree, mine, tickets): &(IntervalTree<String>, Ticket, Vec<Ticket>),
    target: &str,
) -> usize {
    let mut indexes: Vec<Option<HashSet<String>>> =
        (0..mine.values.len()).map(|_| None).collect::<Vec<_>>();

    tickets
        .iter()
        .filter(|&ticket| {
            ticket
                .values
                .iter()
                .all(|v| tree.intersecting(*v).any(|x| x.contains(*v)))
            // true
        })
        .chain(vec![mine])
        .for_each(|x| {
            x.values.iter().enumerate().for_each(|(a, value)| {
                let possible = tree
                    .intersecting(*value)
                    .map(|x| x.value.clone())
                    .collect::<HashSet<_>>();

                if let Some(set) = indexes.get_mut(a).unwrap() {
                    *set = set
                        .intersection(&possible)
                        .cloned()
                        .collect::<HashSet<String>>();
                } else {
                    indexes[a] = Some(possible);
                }
            });
        });

    let mut indexes = indexes
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<HashSet<_>>>();

    // println!("{:?}", indexes);

    while indexes.iter().any(|x| x.len() > 1) {
        let to_remove: Vec<(usize, String)> = indexes
            .iter()
            .enumerate()
            .filter(|x| x.1.len() == 1)
            .map(|x| (x.0, x.1.iter().next().unwrap().clone()))
            .collect::<Vec<_>>();

        indexes
            .iter_mut()
            .filter(|x| x.len() >= 1)
            .enumerate()
            .for_each(|(i, x)| {
                to_remove.iter().filter(|v| v.0 != i).for_each(|(_, v)| {
                    x.remove(v);
                })
            });
    }

    return indexes
        .iter()
        .zip(mine.values.iter())
        .filter(|(x, a)| x.iter().next().unwrap().starts_with(target))
        // .inspect(|x| println!("{:?}", x))
        .map(|x| x.1)
        .product();
}

// TODO("Stable matching algorithm")

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d16ex1() {
        let input = read_input(include_str!("input/day16test.txt"), 500);
        // println!("{:#?}", input.0);
        assert_eq!(2, input.0.intersecting(7).count());
        assert_eq!(true, input.0.intersecting(3).next().is_some());
        assert_eq!(true, input.0.intersecting(47).next().is_some());
        assert_eq!(true, input.0.intersecting(40).next().is_some());
        assert_eq!(false, input.0.intersecting(4).next().is_some());
        assert_eq!(true, input.0.intersecting(50).next().is_some());
        assert_eq!(71, exercise_1(&input));
    }

    #[test]
    fn d16ex1_2() {
        let input = read_input(include_str!("input/day16.txt"), 500);
        assert_eq!(24980, exercise_1(&input));
    }

    #[test]
    fn d16ex2() {
        let input = read_input(include_str!("input/day16.txt"), 500);
        assert_eq!(809376774329, exercise_2(&input, "departure"));
    }

    #[bench]
    fn d16_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day16.txt"), 500);
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d16_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day16.txt"), 500);
        b.iter(|| exercise_2(&input, "departure"));
    }
}
