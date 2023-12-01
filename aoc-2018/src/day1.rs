//! Day 1
//! After feeling like you've been falling for a few minutes, you look at the device's tiny screen. "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain destination lock." Below the message, the device shows a sequence of changes in frequency (your puzzle input). A value like +6 means the current frequency increases by 6; a value like -3 means the current frequency decreases by 3.
use hashbrown::HashSet;

pub fn execute_exercises() {
    println!("Calibrated frequency: {}", exercise_1(frequency_delta_list()));
    println!("First reoccuring frequency: {}", exercise_2(frequency_delta_list()));
}

fn exercise_1(frequency_list: impl Iterator<Item = i32>) -> i32 {
    frequency_list.sum::<i32>()
}

fn exercise_2(frequency_list: impl Iterator<Item = i32> + std::clone::Clone) -> i32 {    
    let mut counter = 0i32;

    let mut visited_set = HashSet::with_capacity(1200);    
    visited_set.insert(0);
    
    for freq_change in frequency_list.cycle() {
        counter += freq_change;
        if !visited_set.insert(counter) {
            return counter;
        }
    }
    
    unreachable!()
}
fn frequency_delta_list() -> impl Iterator<Item = i32> + std::clone::Clone {
    include_str!("../input/day1_in.txt").lines().map(|l| l.parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    
    use super::{exercise_1, exercise_2, frequency_delta_list};    
    use crate::test::Bencher;

    #[test]
     fn d1_ex1_s1() {
        let input = vec!(1, 1, 1).into_iter();
        assert_eq!(exercise_1(input), 3);
    }

    #[test]
    fn d1_ex1_s2() {
        let input = vec!(1, 1, -2).into_iter();
        assert_eq!(exercise_1(input), 0);
    }

    #[test]
    fn d1_ex1_s3() {
        let input = vec!(-1, -2, -3).into_iter();
        assert_eq!(exercise_1(input), -6);
    }
    
    #[test]
    fn d1_ex2_s1() {
        let input = vec![1, -2, 3, 1].into_iter();
        assert_eq!(exercise_2(input), 2);
    }

    #[test]
    fn d1_ex2_s2() {
        assert_eq!(exercise_2(vec![1, -1].into_iter()), 0);
    }

    #[test]
    fn d1_ex2_s3() {
        assert_eq!(exercise_2(vec![3, 3, 4, -2, -4].into_iter()), 10);
    }

    #[test]
    fn d1_ex2_s4() {
        assert_eq!(exercise_2(vec![-6, 3, 8, 5, -6].into_iter()), 5);
    }

    #[test]
    fn d1_ex2_s5() {
        assert_eq!(exercise_2(vec![7, 7, -2, -7, -4].into_iter()), 14);
    }

    #[bench]
    fn d1_bench_ex1(b: &mut Bencher) {        
        b.iter(|| {
            exercise_1(frequency_delta_list());
        });        
    }
    
    #[bench]
    fn d1_bench_ex2(b: &mut Bencher) {        
        b.iter(|| {
            exercise_2(frequency_delta_list());
        });        
    }

}