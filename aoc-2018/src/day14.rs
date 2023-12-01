
pub fn execute_exercises() {  
    
    println!("Collision at: {:?}", exercise_1(607331));
    println!("Collision at: {:?}", exercise_2(vec!(6u8,0,7,3,3,1)));
    
    //println!("Final cart at: {:?}", exercise_2(a, b))
}

fn exercise_1(input: usize) -> Vec<u8> {
    let mut recipe_grades = Vec::with_capacity(input + 20);
    recipe_grades.push(3);
    recipe_grades.push(7u8);    

    let mut elf_1 = 0;
    let mut elf_2 = 1;

    while recipe_grades.len() < (input + 10) {
        let (a, b) = (recipe_grades[elf_1], recipe_grades[elf_2]);
        let new_grade = a + b;

        if new_grade >= 10 {
            recipe_grades.push(new_grade / 10);
            recipe_grades.push(new_grade % 10);
        } else {
            recipe_grades.push(new_grade);
        }

        elf_1 = (elf_1 + a as usize + 1) % recipe_grades.len();
        elf_2 = (elf_2 + b as usize + 1) % recipe_grades.len();

        //println!("{:?}", recipe_grades);
    }

    recipe_grades[input..(input+10)].iter().cloned().collect::<Vec<_>>()
}

fn exercise_2(input: Vec<u8>) -> usize {
    let mut recipe_grades = Vec::with_capacity(10000);
    recipe_grades.push(3u8);
    recipe_grades.push(7u8);    

    let mut elf_1 = 0;
    let mut elf_2 = 1;

    let input_len = input.len();
    loop {
        let (a, b) = (recipe_grades[elf_1], recipe_grades[elf_2]);
        let new_grade = a + b;
        let recipe_len = recipe_grades.len();
        if new_grade >= 10 {
            recipe_grades.push(new_grade / 10);
            if recipe_len + 1 >= input_len && check_sequence(&input, &recipe_grades[(recipe_len + 1 - input_len)..(recipe_len + 1)]) {
                return recipe_grades.len() - input.len();
            }
            recipe_grades.push(new_grade % 10);
            if recipe_len + 2 >= input_len && check_sequence(&input, &recipe_grades[(recipe_len + 2 - input_len)..(recipe_len + 2)]) {
                return recipe_grades.len() - input.len();
            }
        } else {
            recipe_grades.push(new_grade);
            if recipe_len + 1 >= input_len && check_sequence(&input, &recipe_grades[(recipe_len + 1 - input_len)..(recipe_len + 1)]) {
                return recipe_grades.len() - input.len();
            }
        }

        elf_1 = (elf_1 + a as usize + 1) % recipe_grades.len();
        elf_2 = (elf_2 + b as usize + 1) % recipe_grades.len();
    }
}

fn check_sequence(input: &[u8], slice: &[u8]) -> bool {
    input.iter().zip(slice.iter()).all(|(a, b)| a == b)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day14_ex1_s1() {
        assert_eq!(exercise_1(9), vec!(5,1,5,8,9,1,6,7,7,9));
        assert_eq!(exercise_1(5), vec!(0,1,2,4,5,1,5,8,9,1));
        assert_eq!(exercise_1(18), vec!(9,2,5,1,0,7,1,0,8,5));
        assert_eq!(exercise_1(2018), vec!(5,9,4,1,4,2,9,8,8,2));
    }

    #[test]
    fn day14_ex2_s1() {
        assert_eq!(exercise_2(vec!(5,1,5,8,9)), 9);
        assert_eq!(exercise_2(vec!(0,1,2,4,5)), 5);
        assert_eq!(exercise_2(vec!(9,2,5,1,0)), 18);
        assert_eq!(exercise_2(vec!(5,9,4,1,4)), 2018);
    }

    #[bench]
    fn day14_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(607331));
    }

    #[bench]
    fn day14_bench_ex2(b: &mut Bencher) {
        b.iter(|| exercise_2(vec!(6u8,0,7,3,3,1)));
    }
}