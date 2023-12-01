use std::collections::VecDeque;
use hashbrown::HashSet;

pub fn execute_exercises() {
    let (a, b) = read_input();
    println!("Sum: {}",exercise_1_sparse(a, b, 20));
    let (a, b) = read_input();
    println!("larger: {}",exercise_1_sparse(a, b, 50000000000));        
}

fn read_input() -> (VecDeque<bool>, [bool; 32]) {
    parse_input(include_str!("../input/day12_in.txt"))
}

fn parse_input(input: &str) -> (VecDeque<bool>, [bool; 32]) {
    let mut iter = input.lines();

    let init_state = iter.next().unwrap()[15..].chars().map(|c| c == '#').collect::<VecDeque<bool>>();
    iter.next();

    let mut translation_table = [false; 32];

    for s in iter {
        let s: Vec<_> = s.split(" => ").collect();
        let pots: Vec<_> = s[0].chars().map(|c| c == '#').collect();

        let pot_id = pots_to_id([
            Some(&pots[0]), Some(&pots[1]), Some(&pots[2]), Some(&pots[3]), Some(&pots[4])
        ]);
        let transformed = s[1] == "#";
        translation_table[pot_id] = transformed;        
    }

    (init_state, translation_table)
}

fn pots_to_id(c: [Option<&bool>; 5]) -> usize {
    c.into_iter().enumerate().map(|(i, o)| {
        if let Some(e) = o {
            (**e as usize) * 2usize.pow(i as u32)
        } else {
            0
        }
    }).sum()
}

fn exercise_1_sparse(mut state: VecDeque<bool>, input: [bool; 32], cycles: usize) -> i64 {
    //print!("{}: ", 0);    
    let mut starting_pos = 0i64;

    let mut prev_positives = 0;
    let mut converged_counter = 0;
    let mut prev_value = 0;

    for cycle in 0..cycles {
        let mut counter = 0u8;

        // Handle the left 4 values
        counter = (counter / 2) + state[0] as u8 * 16;
        let l_2 = input[counter as usize];
        counter = (counter / 2) + state[1] as u8 * 16;
        let l_1 = input[counter as usize];
        counter = (counter / 2) + state[2] as u8 * 16;
        state[0] = input[counter as usize];
        counter = (counter / 2) + state[3] as u8 * 16;
        state[1] = input[counter as usize];

        let mut positives = l_1 as usize + l_2 as usize + state[0] as usize + state[1] as usize;

        // Handle the middle values
        for i in 2..state.len() - 2 {
            counter = (counter / 2) + state[i + 2] as u8 * 16;
            state[i] = input[counter as usize];
            positives += state[i] as usize;
        }

        // Handle the 4 right values
        let last = state.len() - 1;
        counter = counter / 2;
        state[last - 1] = input[counter as usize];
        counter = counter / 2;
        state[last] = input[counter as usize];
        counter = counter / 2;       
        let r_1 = input[counter as usize];
        counter = counter / 2;
        let r_2 = input[counter as usize];

       positives += r_1 as usize + r_2 as usize + state[last - 1] as usize + state[last] as usize;

       match (l_2, l_1) {
            (true, true) => {
                state.push_front(true);
                state.push_front(true);
                starting_pos -= 2;
            }
            (true, false) => {
                state.push_front(false);
                state.push_front(true);
                starting_pos -= 2;
            }
            (false, true) => {
                state.push_front(true);
                starting_pos -= 1;
            }
            _ => {}
        }

        match (r_1, r_2) {
            (true, true) => {
                state.push_back(true);
                state.push_back(true);
            }
            (true, false) => {
                state.push_back(true);                
            }
            (false, true) => {
                state.push_back(false);
                state.push_back(true);
            }
            _ => {}
        }      
        if prev_positives != positives {
            prev_positives = positives;
            converged_counter = 0;
        } else {
            converged_counter += 1;
            if converged_counter == 4 {
                prev_value = calculate_sum(starting_pos, state.iter().cloned());
            }
            if converged_counter == 5 {
                let current_value = calculate_sum(starting_pos, state.iter().cloned());                
                let increase = current_value - prev_value;

                let cycles_to_go = cycles - cycle - 1;
                println!("Converged! cycles to go: {}/{} +{}", cycle, cycles_to_go, increase);

                return current_value + (cycles_to_go as i64 * increase);
            }
        }
    }
    
    calculate_sum(starting_pos, state.iter().cloned())
}

fn calculate_sum(starting_pos: i64, state: impl Iterator<Item = bool>) -> i64 {
    state.enumerate().map(|(i, s)| {
        if s {
            i as i64 + starting_pos
        } else {
            0
        }
    }).sum()
}

fn pretty_print(input: &HashSet<i64>, left: isize, right: isize) {
    for i in left..=right {
        if input.contains(&(i as i64)) {
            print!("#");
        } else {
            print!(".");
        }        
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day12_ex1_s1() {
        let input = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";
        let (a, b) = parse_input(input);        
        let result = exercise_1_sparse(a, b, 20);

        assert_eq!(result, 325);
    }

    #[test]
    fn day12_ex1_s2() {
        let (a, b) = read_input();
        assert_eq!(exercise_1_sparse(a, b, 20), 4818);
    }

    #[test]
    fn day12_ex2_s1() {
        let (a, b) = read_input();
        assert_eq!(exercise_1_sparse(a, b, 50000000000), 5100000001377);
    }

    #[bench]
    fn day12_ex1(b: &mut Bencher) {
        b.iter(|| {
            let (a, b) = read_input();
            exercise_1_sparse(a, b, 20);
        })
    }

    #[bench]
    fn day12_ex2(b: &mut Bencher) {
        b.iter(|| {
            let (a, b) = read_input();
            exercise_1_sparse(a, b, 50000000000);
        })
    }

}