use std::unimplemented;

use hashbrown::HashMap;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day19.txt"));
    println!("{}", exercise_1(&input));
    let input = read_input(include_str!("input/day19_2.txt"));
    println!("{}", exercise_1(&input));
}

#[derive(Clone, Debug)]
enum Rule {
    None,
    Value(Vec<char>),
    Rules(Vec<Vec<usize>>),
}

fn read_input(input: &str) -> (Vec<Rule>, Vec<Vec<char>>) {
    let mut iter = input.lines();
    let map = iter
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let mut it = x.split(": ");
            let rule_id: usize = it.next().and_then(|x| x.parse().ok()).unwrap();
            let value = it.next().unwrap();
            let rule = if value.starts_with("\"") {
                Rule::Value(value.replace("\"", "").chars().collect())
            } else {
                Rule::Rules(
                    value
                        .split(" | ")
                        .map(|x| {
                            x.split(' ')
                                .map(|x| x.parse().unwrap())
                                .collect::<Vec<usize>>()
                        })
                        .collect::<Vec<_>>(),
                )
            };
            (rule_id, rule)
        })
        .collect::<HashMap<_, _>>();

    let mut vec = vec![None; (*map.keys().max().unwrap()) + 1];

    for (id, x) in map {
        vec[id] = Some(x);
    }

    let vec = vec.into_iter().map(|x| x.unwrap_or(Rule::None)).collect();
    let messages = iter.map(|x| x.chars().collect()).collect();

    (vec, messages)
}

fn exercise_1((rules, messages): &(Vec<Rule>, Vec<Vec<char>>)) -> usize {
    messages
        .iter()
        .flat_map(|message| {
            check_rule(&rules[0], &rules, 0, message)
                .map(move |x| x == message.len())
                .next()
        })
        .filter(|x| *x)
        .count()
}

fn check_rule<'a>(
    rule: &'a Rule,
    rules: &'a Vec<Rule>,
    index: usize,
    message: &'a Vec<char>,
) -> impl Iterator<Item = usize> + 'a {
    let iterator: Box<dyn Iterator<Item = usize> + 'a> = match rule {
        Rule::None => unreachable!(),
        Rule::Value(val) => {
            if (index + val.len()) <= message.len()
                && message[index..].iter().zip(val.iter()).all(|(a, b)| a == b)
            {
                Box::new(vec![index + val.len()].into_iter())
            } else {
                Box::new(vec![].into_iter())
            }
        }
        Rule::Rules(ref_rules) => Box::new(
            ref_rules
                .iter()
                .cloned()
                .map(move |x| {
                    let b: Box<dyn Iterator<Item = usize> + 'a> = Box::new(vec![index].into_iter());
                    x.into_iter().fold(b, |acc, rule_id| {
                        Box::new(
                            acc.map(move |x| check_rule(&rules[rule_id], rules, x, message))
                                .flatten(),
                        )
                    })
                })
                .flatten(),
        ),
    };

    iterator
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d19in() {
        read_input(include_str!("input/day19test.txt"));
    }
    #[test]
    fn d18ex1() {
        let input = read_input(include_str!("input/day19test.txt"));
        assert_eq!(2, exercise_1(&input));
        // assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input(include_str!("input/day19test2.txt"));
        assert_eq!(12, exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day19.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day19_2.txt"));
        b.iter(|| exercise_1(&input));
    }
}
