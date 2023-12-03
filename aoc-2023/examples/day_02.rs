use std::str::FromStr;

type InputType = (usize, Vec<Vec<(usize, Colour)>>);

pub fn main() {
    let input = parse(include_str!("../input/day_02_big.txt")).collect::<Vec<_>>();
    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = InputType> + 'a {
    input.lines().map(|x| parse_line(x))
}

fn parse_line(line: &str) -> InputType {
    let mut i = line.split(':');

    let game = i.next().unwrap().replace("Game ", "").parse::<usize>();
    let colours = i
        .next()
        .unwrap()
        .split(';')
        .filter(|x| !x.is_empty())
        .map(|s| {
            s.split(',')
                .map(|s| {
                    let mut i = s.trim().split_whitespace();
                    let amount = i
                        .next()
                        .expect(&format!("No amount in {}", line))
                        .parse::<usize>()
                        .unwrap();
                    let colour = i.next().unwrap().parse::<Colour>().unwrap();
                    (amount, colour)
                })
                .collect()
        })
        .collect();

    (game.unwrap(), colours)
}

fn exercise_1(input: Vec<InputType>) -> usize {
    input.iter().filter(|i| is_possible(i)).map(|s| s.0).sum()
}

fn is_possible(input: &InputType) -> bool {
    input.1.iter().all(|s| {
        s.iter().all(|(amount, colour)| match colour {
            Colour::Red => amount <= &12,
            Colour::Green => amount <= &13,
            Colour::Blue => amount <= &14,
        })
    })
}

fn exercise_2(input: Vec<InputType>) -> usize {
    input
        .iter()
        .map(|(_, game)| {
            game.iter()
                .flat_map(|set| set.iter())
                .fold([0, 0, 0], |[r, g, b], (amount, colour)| match colour {
                    Colour::Green => [r, g.max(*amount), b],
                    Colour::Blue => [r, g, b.max(*amount)],
                    Colour::Red => [r.max(*amount), g, b],
                })
                .into_iter()
                .product::<usize>()
        })
        .sum()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    Green,
    Blue,
    Red,
}

impl FromStr for Colour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "green" => Ok(Colour::Green),
            "blue" => Ok(Colour::Blue),
            "red" => Ok(Colour::Red),
            _ => Err(()),
        }
    }
}
