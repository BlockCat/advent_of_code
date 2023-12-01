use std::ops::Add;

macro_rules! read_input {
    ($input:expr) => {$input.split(',').map(|dir| {
        match dir {
            "n" => Direction::N,
            "s" => Direction::S,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "nw" => Direction::NW,
            "sw" => Direction::SW,
            c => panic!("{} is not a valid direction", c),
        }
    })}
}

trait Distance {
    fn distance(&self) -> u32;
}
enum Direction {
    N, S, NE, SE, NW, SW
}

impl Distance for (i32, i32, i32) {
    
    fn distance(&self) -> u32 {
        (self.0.abs() + self.1.abs() + self.2.abs()) as u32 / 2
    }

}

impl Add<(i32, i32, i32)> for Direction {
    type Output = (i32, i32, i32);

    fn add(self, (x, y, z): (i32, i32, i32)) -> Self::Output {
        match self {
            Direction::N => (x, y - 1, z + 1),
            Direction::S => (x, y + 1, z - 1),
            Direction::NE => (x + 1, y - 1, z),
            Direction::SE => (x + 1, y, z - 1),
            Direction::NW => (x - 1, y, z + 1),
            Direction::SW => (x - 1, y + 1, z),
        }
    }
}

fn algorithm1(input: impl Iterator<Item = Direction>) -> (u32, u32) {
    let (pos, furthest) = input.fold(((0, 0, 0), 0), |(pos, dist), dir| {
        let fff = dir + pos;
        let nd = fff.distance();

        (fff, std::cmp::max(dist, nd))
    });

    (pos.distance(), furthest)
}

#[test]
fn test_examples() {
    assert_eq!(algorithm1(read_input!("ne,ne,ne")), (3, 3));
    assert_eq!(algorithm1(read_input!("ne,ne,sw,sw")), (0, 2));
    assert_eq!(algorithm1(read_input!("ne,ne,s,s")), (2, 2));
    assert_eq!(algorithm1(read_input!("se,sw,se,sw,sw")), (3, 3));
    
}

#[test]
fn run11() {
    let result = algorithm1(read_input!(include_str!("input/day11.txt")));


    println!("Score: {:?}", result);
}