use std::iter::Iterator;

#[derive(PartialEq, Eq, Clone, Debug)] enum Maze { Horizontal, Vertical, Corner, Character(char) }
#[derive(PartialEq, Eq, Clone, Debug)] enum Direction { North, East, South, West } 

type Board = Vec<Vec<Option<Maze>>>;

struct BoardIterator {
    board: Board,
    position: (usize, usize),    
    direction: Direction,
    width: usize,
    height: usize
}

impl BoardIterator {
    fn new(board: Board) -> BoardIterator {
        let mut position = (board[0].iter().position(|x| x == &Some(Maze::Vertical)).expect("Could not find starting position"), 0);
        let height = board.len();
        let width = board[0].len();
        BoardIterator { 
            position,
            board,
            direction: Direction::South,
            width, 
            height
        }
    }
}

impl Iterator for BoardIterator {
    type Item = Option<Maze>;

    fn next(&mut self) -> Option<Self::Item> {        
        let item = &self.board[self.position.1][self.position.0];                
        //println!("{:?}", item);
        let new_direction = match (item, &self.direction) {
            (Some(Maze::Corner), Direction::North) 
            | (Some(Maze::Corner), Direction::South) => {
                if self.position.0 == 0 {
                    Direction::East                    
                } else if let Some(Maze::Horizontal) = self.board[self.position.1][self.position.0 - 1] {
                    Direction::West
                } else if let Some(Maze::Character(_)) = self.board[self.position.1][self.position.0 - 1] {
                    Direction::West
                } else {
                    Direction::East
                }
            
            },
            (Some(Maze::Corner), Direction::East) 
            | (Some(Maze::Corner), Direction::West) => {
                if self.position.1 == 0 {
                    Direction::South
                } else if let Some(Maze::Vertical) = self.board[self.position.1 - 1][self.position.0] {
                    Direction::North
                } else if let Some(Maze::Character(_)) = self.board[self.position.1 - 1][self.position.0] {
                    Direction::North
                } else {
                    Direction::South
                }
            },
            (Some(_), c) => c.clone(),
            (None, _) => return None,
        };        

        self.direction = new_direction;

        self.position = match &self.direction {
            Direction::North => (self.position.0, self.position.1 - 1),
            Direction::South => (self.position.0, self.position.1 + 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0 - 1, self.position.1),
        };

        Some(item.clone())
    }
}

fn algorithm1(input: Board) -> String {
    BoardIterator::new(input).filter_map(|m| {
        match m {
            Some(Maze::Character(c)) => Some(c),
            _ => None
        }
    }).collect::<String>()
}

fn algorithm2(input: Board) -> usize {
    BoardIterator::new(input).count()
}

fn read_input(input: &str) -> Board {
    input.lines().map(read_line).collect()
}
fn read_line(line: &str) -> Vec<Option<Maze>> {
    line
        .chars()
        .map(|c| {
            match c {
                ' ' => None,
                '|' => Some(Maze::Vertical),
                '-' => Some(Maze::Horizontal),
                '+' => Some(Maze::Corner),
                c => Some(Maze::Character(c))
            }
        })
        .collect()
}

#[test]
fn run19() {
    let input = read_input(include_str!("input/day19.txt"));

    println!("name: {}, left: {}", algorithm1(input.clone()), algorithm2(input));
}
#[test]
fn test_examples() {
    let input = read_input(include_str!("input/day19test.txt"));    
    assert_eq!(&algorithm1(input.clone()), "ABCDEF");    
    assert_eq!(algorithm2(input), 38);

}