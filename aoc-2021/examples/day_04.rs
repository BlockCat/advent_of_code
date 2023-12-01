use std::collections::{HashMap, HashSet};

type Input = (Vec<u8>, Vec<Board>);

pub fn main() {
    let input = parse_input(include_str!("../input/day04.txt"));

    println!("Ex1: {}", exercise_1(input.clone()));
    println!("Ex2: {}", exercise_2(input));
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = input.lines();

    let chosen_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(parse_line)
        .collect::<Vec<_>>();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let board = lines
            .by_ref()
            .take(5)
            .flat_map(|x| x.split_whitespace().map(parse_line));

        boards.push(Board::new(board.collect()));
    }

    (chosen_numbers, boards)
}

fn parse_line(line: &str) -> u8 {
    line.parse().unwrap()
}

fn exercise_1((chosen, mut boards): Input) -> usize {
    for number in chosen {
        for board in boards.iter_mut() {
            board.choose(number);

            if board.has_won() {
                return board.win_value() * (number as usize);
            }
        }
    }

    unreachable!()
}

fn exercise_2((chosen, mut boards): Input) -> usize {
    for number in chosen {
        if boards.len() > 1 {
            boards = boards
                .into_iter()
                .map(|mut x| {
                    x.choose(number);
                    x
                })
                .filter(|x| !x.has_won())
                .collect();
        } else {
            assert!(boards.len() == 1);

            boards[0].choose(number);

            if boards[0].has_won() {
                return boards[0].win_value() * (number as usize);
            }
        }
    }

    unreachable!()
}

#[derive(Clone, PartialEq, Eq)]
struct Board {
    map: HashMap<u8, usize>,    
    cols: [u8; 5],
    rows: [u8; 5],
}

impl Board {
    pub fn new(slice: Vec<u8>) -> Self {
        let map = slice.iter().enumerate().map(|x| (*x.1, x.0)).collect();        
        Self {
            map,
            cols: [0; 5],
            rows: [0; 5],
        }
    }

    pub fn choose(&mut self, number: u8) {
        if let Some(index) = self.map.get(&number) {
            let row = index / 5;
            let col = index % 5;
            self.cols[col] += 1;
            self.rows[row] += 1;
            self.map.remove(&number);
        }
    }

    pub fn has_won(&self) -> bool {
        (0..5usize).any(|index| self.cols[index] == 5 || self.rows[index] == 5)
    }

    pub fn win_value(&self) -> usize {
        self.map.keys().map(|x| *x as usize).sum()
    }
}
