use std::collections::VecDeque;
use std::iter::FromIterator;

use hashbrown::HashSet;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day22.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

type Input = (VecDeque<usize>, VecDeque<usize>);
fn read_input(input: &str) -> Input {
    let mut it = input.lines();

    let player_1 = VecDeque::from_iter(
        it.by_ref()
            .skip(1)
            .take_while(|x| !x.is_empty())
            .map(|x| x.parse().unwrap()),
    );

    let player_2 = VecDeque::from_iter(it.skip(1).map(|x| x.parse().unwrap()));

    (player_1, player_2)
}

fn exercise_1((player_1, player_2): &Input) -> usize {
    let mut player_1 = player_1.clone();
    let mut player_2 = player_2.clone();

    while !player_1.is_empty() && !player_2.is_empty() {
        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();
        if p1 > p2 {
            player_1.push_back(p1);
            player_1.push_back(p2);
        } else {
            player_2.push_back(p2);
            player_2.push_back(p1);
        }
    }

    let winner = if !player_1.is_empty() {
        println!("The crab wins in the space game");
        player_1
    } else if !player_2.is_empty() {
        println!("You win in the space game");
        player_2
    } else {
        unreachable!()
    };

    let len = winner.len();

    winner
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, x)| acc + (len - index) * x)
}

fn exercise_2((player_1, player_2): &Input) -> usize {
    let player_1 = player_1.clone();
    let player_2 = player_2.clone();

    let (player, winner) = play_game(player_1, player_2);

    let len = winner.len();

    match player {
        Player::Player1 => println!("The crab wins in the recursion game"),
        Player::Player2 => println!("You win in the recursion game"),
    }

    winner
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, x)| acc + (len - index) * x)
}

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

fn play_game(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
) -> (Player, VecDeque<usize>) {
    let mut seen = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        if !seen.insert((player1.clone(), player2.clone())) {
            //we've seen the configuration
            return (Player::Player1, player1);
        }

        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        let winner = if p1 <= player1.len() && p2 <= player2.len() {
            let cl1 = player1.iter().take(p1).cloned().collect();
            let cl2 = player2.iter().take(p2).cloned().collect();
            play_game(cl1, cl2).0
        } else if p1 > p2 {
            Player::Player1
        } else if p2 > p1 {
            Player::Player2
        } else {
            unreachable!()
        };

        match winner {
            Player::Player1 => {
                player1.push_back(p1);
                player1.push_back(p2);
            }
            Player::Player2 => {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        }
    }

    if !player1.is_empty() {
        (Player::Player1, player1)
    } else if !player2.is_empty() {
        (Player::Player2, player2)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d18ex1() {
        let input = read_input(include_str!("input/day22test.txt"));
        assert_eq!(306, exercise_1(&input));
        // assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input(include_str!("input/day22test.txt"));
        assert_eq!(291, exercise_2(&input));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day22.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day22.txt"));
        b.iter(|| exercise_1(&input));
    }
}
