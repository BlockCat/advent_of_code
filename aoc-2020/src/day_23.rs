use std::collections::VecDeque;

use hashbrown::HashMap;

// #[test]
pub fn run() {
    let x = "963275481";
    // let x = "389125467";

    let input = read_input(x, x.len());
    // println!("{}", exercise_1(input, 100));
    let input = read_input(x, 1_000_000);

    println!("{:?}", exercise_2(&input, 10_000_000));
}

type Input = VecDeque<u32>;

fn read_input(input: &str, len: usize) -> Input {
    let x: VecDeque<_> = input.chars().map(|x| (x as u8 - b'0').into()).collect();
    let max = *x.iter().max().unwrap();

    let numbers_to_pick = len - x.len();

    x.into_iter()
        .chain(((max + 1)..).take(numbers_to_pick))
        .collect()
}

fn exercise_1(input: Input, len: usize) -> String {
    let mut input = normal(input, len);
    let loc = input.iter().position(|x| x == &1).unwrap();
    input.rotate_left(loc);
    input
        .into_iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<String>()
}

fn exercise_2(input: &Input, len: usize) -> usize {
    let max = *input.iter().max().unwrap();

    let mut holder = vec![0; input.len()];

    // holder[input[input.len() - 1] as usize - 1] = input[0];

    for i in 0..input.len() {
        let m = input[i];
        holder[m as usize - 1] = input[(i + 1) % input.len()];
    }

    let mut picked = input[0];
    for _ in 0..len {
        let next = holder[(picked - 1) as usize];

        let a1 = next;
        let a2 = holder[(a1 - 1) as usize];
        let a3 = holder[(a2 - 1) as usize];
        let a4 = holder[(a3 - 1) as usize];        

        let destination = destination(picked, max, &[a1, a2, a3]);
        let destv = holder[(destination - 1) as usize];
        
        holder[(picked - 1) as usize] = a4;
        holder[(destination - 1) as usize] = a1;        
        holder[(a3 - 1) as usize] = destv;

        picked = a4;
    }

    holder[0] as usize * holder[holder[0] as usize - 1] as usize
}

fn normal(mut input: Input, len: usize) -> Input {
    let max = *input.iter().max().unwrap();

    for i in 0..len {
        // println!("{}: {:?}", i, input);
        // print(&input, 20);
        if i % 524288 == 0 {
            println!("arrived at: {}", i);
        }
        let pick = *input.front().unwrap();
        input.rotate_left(1);
        let mut index = input.len();
        let a = vec![
            input.pop_front().unwrap(),
            input.pop_front().unwrap(),
            input.pop_front().unwrap(),
        ];
        let destination_number = destination(pick, max, &a);

        let destination_index = input.iter().position(|x| *x == destination_number).unwrap() + 1;

        input.rotate_left(destination_index % input.len());
        index -= destination_index;

        input.push_front(a[2]);
        input.push_front(a[1]);
        input.push_front(a[0]);

        input.rotate_left(index);
    }

    input
}

fn print(input: &Input, len: usize) {
    println!("{:?}", input.iter().take(len).collect::<Vec<_>>());
}

fn destination(mut pick: u32, max: u32, a: &[u32]) -> u32 {
    while {
        if pick == 1 {
            pick = max;
        } else {
            pick -= 1;
        }
        a.contains(&pick)
    } {}

    pick
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    // #[test]
    // fn d18ex1() {
    //     let input = read_input("963275481", "963275481".len());
    //     assert_eq!("".to_string(), exercise_1(input, 100));
    //     // assert_eq!(71, exercise_1(&input))
    // }

    #[test]
    fn d18ex2() {
        let input = read_input("963275481", 1_000_000);
        assert_eq!(412990492266, exercise_2(&input, 10_000_000));
    }

    // #[bench]
    // fn d18_bench_ex1(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day23.txt"));
    //     b.iter(|| exercise_1(&input));
    // }

    // #[bench]
    // fn d18_bench_ex2(b: &mut Bencher) {
    //     let input = read_input(include_str!("input/day23.txt"));
    //     b.iter(|| exercise_1(&input));
    // }
}
