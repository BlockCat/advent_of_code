use std::collections::VecDeque;

pub fn execute_exercises() {
    println!("High Score: {}", exercise_1(446, 71522));
    println!("Larger High Score: {}", exercise_1(446, 7_152_200));
}

#[derive(Debug)]
struct Reiger {        
    right: VecDeque<u32>,
    current: u32,
    counter: u32,
}

impl Iterator for Reiger {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..22 {
            self.counter += 1;            
            self.right.push_back(self.current);
            self.current = self.right.pop_front().unwrap();
            self.right.push_back(self.current);
            self.current = self.counter;            
        }

        self.counter += 1;
        for _ in 0..7 {
            self.right.push_front(self.current);
            self.current = self.right.pop_back().unwrap();            
        }        
        let value = self.current;
        
        self.current = self.right.pop_front().unwrap();
        Some((self.counter, value))            
    }
}

impl Reiger {

    fn with_capacity(capacity: usize) -> Self {
        Reiger {     
            right: VecDeque::with_capacity(capacity),
            current: 0,
            counter: 0,
        }
    }
}

fn exercise_1(players: u32, marbles: usize) -> u64 {    
    let mut scores = (0..players).map(|_| 0u64).collect::<Vec<_>>();

    for (marble, score) in Reiger::with_capacity(marbles).take(marbles / 23) {
        scores[((marble - 1) % players) as usize] += (marble + score) as u64;
    }
    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d9_ex1_s1() {        
        assert_eq!(exercise_1(9, 25), 32);
        assert_eq!(exercise_1(10, 1618), 8317);
        assert_eq!(exercise_1(13, 7999), 146373);
        assert_eq!(exercise_1(17, 1104), 2764);
        assert_eq!(exercise_1(21, 6111), 54718);
        assert_eq!(exercise_1(30, 5807), 37305);
    }
    
    #[bench]
    fn d9_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(446, 71522));
    }

    #[bench]
    fn d9_bench_ex2(b: &mut Bencher) {
        b.iter(|| exercise_1(446, 7152200));
    }
}