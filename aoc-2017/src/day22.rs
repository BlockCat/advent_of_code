use std::collections::HashMap;
use utils::{ Direction, Vector2 };

#[derive(PartialEq, Eq, Clone, Copy)]
enum Status {
    Clean = 0, Weakened = 1, Infected = 2, Flagged = 3
}
struct Board {
    position: Vector2,
    direction: Direction,
    board: HashMap<Vector2, Status>,
    pub infected_counter: usize
}

impl Board {
    pub fn burst(&mut self) {
        let infected = *self.board.get(&self.position).unwrap_or(&Status::Clean);

        let (next_direction, next_status) = match infected {
            Status::Infected    => (self.direction.right(), Status::Clean),
            Status::Clean       => (self.direction.left(), Status::Infected),
            _ => unreachable!()
        };

        self.direction = next_direction;
        self.board.insert(self.position, next_status);

        if next_status == Status::Infected {
            self.infected_counter += 1;
        }
        
        self.position += self.direction;
    }

    pub fn burst_evolved(&mut self) {
        let infected = *self.board.get(&self.position).unwrap_or(&Status::Clean);

        let (next_direction, next_status) = match infected {
            Status::Clean       => (self.direction.left(), Status::Weakened),
            Status::Weakened    => (self.direction, Status::Infected),
            Status::Infected    => (self.direction.right(), Status::Flagged),
            Status::Flagged     => (self.direction.reverse(), Status::Clean),
            _ => unreachable!()
        };

        self.direction = next_direction;
        self.board.insert(self.position, next_status);

        if next_status == Status::Infected {
            self.infected_counter += 1;
        }
        
        self.position += self.direction;
    }
}

impl std::str::FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let board: HashMap<Vector2, Status> = s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        (Vector2(x as isize, y as isize), if c== '#' { Status::Infected } else { Status::Clean })
                    })
            })
            .collect();
        let y = s.lines().count() / 2;
        let x = s.lines().next().unwrap().chars().count() / 2;
        Ok(Board {
            direction: Direction::North,
            board,
            position: Vector2(x as isize, y as isize),
            infected_counter: 0
        })
    }
}

#[test]
fn test_examples() {
    let input = r"..#
#..
...";
    let mut input = input.parse::<Board>().unwrap();

    for _ in 0..10_000 {
        input.burst();
    }

    assert_eq!(input.infected_counter, 5587);

    let input = r"..#
#..
...";
    let mut input = input.parse::<Board>().unwrap();

    for _ in 0..10_000_000 {
        input.burst_evolved();
    }

    assert_eq!(input.infected_counter, 2511944);
}

#[test]
fn run22() {
    let mut input = include_str!("input/day22.txt").parse::<Board>().unwrap();
    for _ in 0..10_000 {
        input.burst();
    }
    assert_eq!(input.infected_counter, 5182);
    println!("infected: {}", input.infected_counter);

    let mut input = include_str!("input/day22.txt").parse::<Board>().unwrap();
    for _ in 0..10_000_000 {
        input.burst_evolved();
    }
    println!("infected evolved: {}", input.infected_counter);
}