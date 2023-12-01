type Input = Vec<usize>;

pub fn main() {
    let input = parse_input(include_str!("../input/day06.txt"));

    println!("Ex1: {}", exercise_n::<80>(&input));
    println!("Ex2: {}", exercise_n::<256>(&input));
}

fn parse_input(input: &str) -> Input {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn exercise_n<const N: usize>(fishes: &Input) -> usize {
    let mut group = [0; 9];    

    for fish in fishes {
        group[*fish] += 1;
    }

    for _ in 0..N {
        let mut ng = [0; 9];    
        ng[6] += group[0];
        ng[8] += group[0];
        for days in 1..9 {
            ng[days-1] += group[days];
        }
        group = ng;
    }

    group.iter().sum()
}

// fn exercise_2(lines: &Input) -> usize {}
