use std::cmp::Ordering;

type InputType = Vec<(PacketItem, PacketItem)>;

pub fn main() {
    let numbers = input();
    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    let mut lines = include_str!("../input/day_13.txt")
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| parse_line(&mut x.chars()));
    let mut pairs = InputType::with_capacity(200);
    while let Some(l) = lines.next() {
        let o = lines.next().unwrap();
        pairs.push((l, o));
    }
    pairs
}

fn parse_line(s: &mut impl Iterator<Item = char>) -> PacketItem {
    let mut items = Vec::new();
    let mut number = None;
    loop {
        match s.next() {
            Some('[') => {
                let item = parse_line(s);
                items.push(item);
            }
            Some(']') => {
                if let Some(n) = number {
                    items.push(PacketItem::Number(n));
                }
                return PacketItem::List(items);
            }
            Some(',') => {
                if let Some(n) = number {
                    items.push(PacketItem::Number(n));
                }
                number = None;
            }
            Some(a) => {
                let a = a as u8 - b'0';
                number = if let Some(n) = number {
                    Some(n * 10 + a as usize)
                } else {
                    Some(a as usize)
                };
            }
            None => {
                return PacketItem::List(items);
            }
        };
    }
}

fn exercise_1(input: InputType) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
        .map(|x| x.0 + 1)
        .sum()
}

fn exercise_2(input: InputType) -> usize {
    let mut input = input
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect::<Vec<_>>();

    let key_0 = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Number(2)])]);
    let key_1 = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Number(6)])]);

    input.push(key_0.clone());
    input.push(key_1.clone());

    input.sort();

    let key_0 = input.iter().position(|x| x == &key_0).unwrap() + 1;
    let key_1 = input.iter().position(|x| x == &key_1).unwrap() + 1;

    key_0 * key_1
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketItem {
    Number(usize),
    List(Vec<PacketItem>),
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, r: &Self) -> Option<Ordering> {
        match (self, r) {
            (PacketItem::Number(l), PacketItem::Number(r)) => l.partial_cmp(r),
            (PacketItem::Number(l), PacketItem::List(_)) => {
                PacketItem::List(vec![PacketItem::Number(*l)]).partial_cmp(r)
            }
            (PacketItem::List(_), PacketItem::Number(r)) => {
                self.partial_cmp(&PacketItem::List(vec![PacketItem::Number(*r)]))
            }
            (PacketItem::List(l), PacketItem::List(r)) => l.partial_cmp(r),
        }
    }
}
