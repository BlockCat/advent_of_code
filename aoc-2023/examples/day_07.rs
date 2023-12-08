use std::collections::HashMap;

type InputType = Vec<([char; 5], usize)>;

pub fn main() {
    let input = parse(include_str!("../input/day_07.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> InputType {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> ([char; 5], usize) {
    let (a, b) = line.split_once(" ").unwrap();
    let b = b.parse::<usize>().unwrap();

    let h = a.chars().collect::<Vec<_>>();

    ([h[0], h[1], h[2], h[3], h[4]], b)
}

fn exercise_1(input: InputType) -> usize {
    let mapping = [
        ('1', 1i8),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut lekker = input
        .into_iter()
        .map(|(hand, bid)| {
            (
                rank(&hand),
                [
                    mapping[&hand[0]],
                    mapping[&hand[1]],
                    mapping[&hand[2]],
                    mapping[&hand[3]],
                    mapping[&hand[4]],
                ],
                bid,
            )
        })
        .collect::<Vec<_>>();

    lekker.sort_by_cached_key(|(rank, hand, _)| (*rank, *hand));

    lekker.iter().enumerate().map(|(s, a)| (s + 1) * a.2).sum()
}

fn exercise_2(input: InputType) -> usize {
    let mapping = [
        ('J', 1),
        ('1', 2i8),
        ('2', 3),
        ('3', 4),
        ('4', 5),
        ('5', 6),
        ('6', 7),
        ('7', 8),
        ('8', 9),
        ('9', 10),
        ('T', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut lekker = input
        .into_iter()
        .map(|(hand, bid)| {
            (
                rank2(&hand),
                [
                    mapping[&hand[0]],
                    mapping[&hand[1]],
                    mapping[&hand[2]],
                    mapping[&hand[3]],
                    mapping[&hand[4]],
                ],
                bid,
            )
        })
        .collect::<Vec<_>>();

    lekker.sort_by_cached_key(|(rank, hand, _)| (*rank, *hand));

    lekker.iter().enumerate().map(|(s, a)| (s + 1) * a.2).sum()
}

fn shared_rank(pairs: &[(char, i32)]) -> usize {
    if pairs[0].1 == 4 {
        return 6;
    }

    if pairs[0].1 == 3 && pairs[1].1 == 2 {
        return 5;
    }

    if pairs[0].1 == 3 {
        return 4;
    }

    if pairs[0].1 == 2 && pairs[1].1 == 2 {
        return 3;
    }

    if pairs[0].1 == 2 {
        return 2;
    }

    1
}

fn rank2(a: &[char; 5]) -> usize {
    let mut map = HashMap::new();

    for c in a.iter() {
        let e = map.entry(*c).or_insert(0);
        *e += 1;
    }

    if map.len() == 1 {
        return 7;
    }

    let jokers = map.remove(&'J').unwrap_or(0);

    if jokers == 4 {
        return 7;
    }

    let mut pairs = map.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|x| -x.1);

    if jokers == 3 {
        if pairs[0].1 == 2 {
            // 5
            return 7;
        }
        if pairs[0].1 == 1 {
            // 4
            return 6;
        }
        unreachable!("jokers == 3");
    } else if jokers == 2 {
        if pairs[0].1 == 3 {
            // 5 of a kind
            return 7;
        }
        if pairs[0].1 == 2 {
            // 4 of a kind
            return 6;
        }

        if pairs[0].1 == 1 && pairs[1].1 == 2 {
            // full house
            return 5;
        }

        if pairs[0].1 == 1 && pairs[1].1 == 1 {
            // 3 of a kind
            return 4;
        }
        unreachable!("jokers == 2");
    } else if jokers == 1 {
        if pairs[0].1 == 4 {
            // 5 of a kind
            return 7;
        }
        if pairs[0].1 == 3 {
            // 4 of a kind
            return 6;
        }
        if pairs[0].1 == 2 && pairs[1].1 == 2 {
            // full house
            return 5;
        }

        if pairs[0].1 == 2 && pairs[1].1 == 1 {
            // 3 of a kind
            return 4;
        }

        if pairs[0].1 == 1 && pairs[1].1 == 1 {
            // 1 pairs
            return 2;
        }
        unreachable!("jokers == 1");
    }
    shared_rank(&pairs)
}

fn rank(a: &[char; 5]) -> usize {
    let mut map = HashMap::new();

    for c in a.iter() {
        let e = map.entry(*c).or_insert(0);
        *e += 1;
    }

    if map.len() == 1 {
        return 7;
    }

    let mut pairs = map.into_iter().collect::<Vec<_>>();
    pairs.sort_by_key(|x| -x.1);

    shared_rank(&pairs)
}
