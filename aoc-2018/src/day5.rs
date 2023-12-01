pub fn execute_exercises() {
    //preprocess::pre_process(include_str!("../input/day4_in.txt")).into_iter().for_each(|(guard, sleep, wake)| println!("{} {} {}", guard, sleep, wake));
    println!("units remaining: {}", exercise_1(read_input()));
    println!("collapsed remaining: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<u8> {  
    include_str!("../input/day5_in.txt").bytes().collect()
}

fn exercise_1(input: impl IntoIterator<Item = u8>) -> usize {
    
    // This old implementation can be found in https://github.com/BlockCat/adventofcode2018/commit/f1fc8bf07df0f1c84670eef352f2fc26b9ecbaee
    // Start: dequeue first char C from unseen 
    // ---- part 1 ----
    // dequeue next char D from unseen.

    // If opposite polarity: don't add to seen stack and just drop chars C and D. Go to part 2
    // If no issues: add C to seen stack, set C = D and go to part 1.

    //
    // ---- part 2 ---- C and D got removed
    // scenario 1: stack is empty
    // Dequeue from unseen as char C go to 'part 1'
    //
    // scenario 2: Stack is not empty
    // take char C from stack and char D from unseen queue

    // If opposite polarity drop C and D and go to part 2.
    // If no issues: Add C back to seen stack and set C = D, and go to part 1.

    //
    // ----= part 3 ---- There is nothing left in unseen queue
    // Add char D to seen stack and exit

    
    // Other algorithm    
    // Instead of looking forward, look backward.
    // Oops, inspired [CryZe](https://gist.github.com/CryZe/0182994a72762e099034b706a8fadca3)
    let mut seen: Vec<u8> = Vec::with_capacity(1000);
    let mut prev = 0u8;

    for current in input {
        if current ^ 0x20 == prev {
            seen.pop();
            prev = seen.last().cloned().unwrap_or_default();
        } else {
            prev = current;
            seen.push(prev);
        }
    }

    seen.len()
}


fn exercise_2(input: Vec<u8>) -> usize {
    use std::thread;
    use std::sync::Arc;

    let lower_case: Vec<u8> = "abcdefghijklmnopqrstuvwxyz".bytes().collect();
    let upper_case: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".bytes().collect();

    let input = Arc::new(input);
    let children: Vec<thread::JoinHandle<usize>> = (0..26).map(|i| {                
        thread::spawn({
            let l = lower_case[i];
            let u = upper_case[i];
            let input = Arc::clone(&input);
            move || {
                exercise_1(input.iter().cloned().filter(|&x| x != l && x != u))    
            }
        })
    }).collect();

    children.into_iter().map(|t| t.join().unwrap()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;


    #[test]
    fn d5_ex1_s1() {
        assert_eq!(exercise_1("dabAcCaCBAcCcaDA".bytes()), 10);        
    }
    
    #[test]
    fn d5_ex1_s2() {
        let input = read_input();
        assert_eq!(exercise_1(input), 10368);
    }

    #[test]
    fn d5_ex2_s1() {
        let input = "dabAcCaCBAcCcaDA".bytes().collect();
        assert_eq!(exercise_2(input), 4);
    }

    #[test]
    fn d5_ex2_s2() {
        let input = read_input();
        assert_eq!(exercise_2(input), 4122);
    }

    #[bench]
    fn d5_bench_read(b: &mut Bencher) {
        b.iter(|| read_input());
    }

    #[bench]
    fn d5_bench_ex1(b: &mut Bencher) {        
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d5_bench_ex2(b: &mut Bencher) {        
        b.iter(|| exercise_2(read_input()));
    }
}