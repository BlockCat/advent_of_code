use hashbrown::HashMap;

// #[test]
pub fn run() {
    let input = vec![13, 16, 0, 12, 15, 1];
    println!("{}", exercise_1(&input, 2020));
    println!("{}", exercise_1(&input, 30000000));
}

fn exercise_1(input: &Vec<usize>, target: usize) -> usize {
    ElfIterator::new(input)
        .take(target - input.len())
        .last()
        .unwrap()
}

struct ElfIterator {
    last_number: usize,
    index: usize,
    map: HashMap<usize, usize>,
}

impl ElfIterator {
    pub fn new(input: &Vec<usize>) -> ElfIterator {
        let mut map = HashMap::new();
        for x in input.iter().take(input.len() - 1).enumerate() {
            map.insert(*x.1, x.0);
        }

        ElfIterator {
            map,
            index: input.len() - 1,
            last_number: *input.last().unwrap(),
        }
    }
}

impl Iterator for ElfIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.map.get(&self.last_number).cloned();
        self.map.insert(self.last_number, self.index);
        self.last_number = if let Some(pos) = result {
            self.index - pos
        } else {
            0
        };
        self.index += 1;
        Some(self.last_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d15ex1() {
        assert_eq!(436, exercise_1(&vec![0, 3, 6], 2020));
        assert_eq!(1, exercise_1(&vec![1, 3, 2], 2020));
        assert_eq!(10, exercise_1(&vec![2, 1, 3], 2020));
        assert_eq!(27, exercise_1(&vec![1, 2, 3], 2020));
        assert_eq!(78, exercise_1(&vec![2, 3, 1], 2020));
        assert_eq!(438, exercise_1(&vec![3, 2, 1], 2020));
        assert_eq!(1836, exercise_1(&vec![3, 1, 2], 2020));
    }

    #[test]
    fn d15ex2() {
        assert_eq!(175594, exercise_1(&vec![0, 3, 6], 30000000));
        assert_eq!(2578, exercise_1(&vec![1, 3, 2], 30000000));
        assert_eq!(3544142, exercise_1(&vec![2, 1, 3], 30000000));
        assert_eq!(261214, exercise_1(&vec![1, 2, 3], 30000000));
        assert_eq!(6895259, exercise_1(&vec![2, 3, 1], 30000000));
        assert_eq!(18, exercise_1(&vec![3, 2, 1], 30000000));
        assert_eq!(362, exercise_1(&vec![3, 1, 2], 30000000));
    }

    #[bench]
    fn d15_bench_ex1(b: &mut Bencher) {
        let input = vec![13, 16, 0, 12, 15, 1];
        b.iter(|| exercise_1(&input, 2020));
    }

    #[bench]
    fn d15_bench_ex2(b: &mut Bencher) {
        let input = vec![13, 16, 0, 12, 15, 1];
        b.iter(|| exercise_1(&input, 30000000));
    }
}
