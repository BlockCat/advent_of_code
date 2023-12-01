use hashbrown::HashSet;

#[derive(Clone, Debug)]
enum Instruction {
    NOP(isize),
    JMP(isize),
    ACC(isize),
}

pub fn run() {
    let input = read_input(include_str!("input/day8bigboi.txt").trim());
    println!("{}", exercise_1(&input, None).unwrap_err());
    println!("{}", exercise_2(&input));
}

fn read_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut i = line.split(' ');
            let op = i.next().unwrap();
            let m = i.next().unwrap();
            let val = m.parse().expect(&format!("error {}", m));
            match op {
                "nop" => Instruction::NOP(val),
                "jmp" => Instruction::JMP(val),
                "acc" => Instruction::ACC(val),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn exercise_1(
    codes: &Vec<Instruction>,
    replacement: Option<(usize, Instruction)>,
) -> Result<isize, isize> {
    let mut visited = HashSet::<usize>::new();

    let mut glob = 0isize;
    let mut i = 0usize;

    while i < codes.len() {
        if !visited.insert(i) {
            return Err(glob);
        }

        let code = replacement
            .as_ref()
            .filter(|x| x.0 == i)
            .map(|x| &x.1)
            .unwrap_or(&codes[i]);
        match code {
            Instruction::NOP(_) => {}
            Instruction::JMP(v) => {
                i = (i as isize + (v - 1)) as usize;
            }
            Instruction::ACC(v) => {
                glob += v;
            }
        }

        i += 1;
    }
    Ok(glob)
}

fn exercise_2_sub(
    codes: &Vec<Instruction>,
    oldvisited: &HashSet<usize>,
    pointer: usize,
    global: isize,
    replacement: Instruction,
) -> Result<isize, isize> {
    let mut visited = HashSet::<usize>::new();

    let mut glob = global;
    let mut i = pointer;

    match replacement {
        Instruction::NOP(_) => {}
        Instruction::JMP(v) => {
            i = (i as isize + (v - 1)) as usize;
        }
        Instruction::ACC(v) => {
            glob += v;
        }
    }
    i += 1;

    while i < codes.len() {
        if oldvisited.contains(&i) || !visited.insert(i) {
            return Err(glob);
        }
        match codes[i] {
            Instruction::NOP(_) => {}
            Instruction::JMP(v) => {
                i = (i as isize + (v - 1)) as usize;
            }
            Instruction::ACC(v) => {
                glob += v;
            }
        }
        i += 1;
    }
    Ok(glob)
}

fn exercise_2(codes: &Vec<Instruction>) -> isize {
    let mut visited = HashSet::<usize>::new();

    let mut glob = 0isize;
    let mut i = 0usize;

    while i < codes.len() {
        if !visited.insert(i) {
            return glob;
        }

        match codes[i] {
            Instruction::NOP(v) => {
                if let Ok(g) = exercise_2_sub(codes, &visited, i, glob, Instruction::JMP(v)) {
                    return g;
                }
            }
            Instruction::JMP(v) => {
                if let Ok(g) = exercise_2_sub(codes, &visited, i, glob, Instruction::NOP(v)) {
                    return g;
                }
                i = (i as isize + (v - 1)) as usize;
            }
            Instruction::ACC(v) => {
                glob += v;
            }
        }

        i += 1;
    }

    unreachable!()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d8p1a_test() {
        let input = read_input(
            r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );
        assert_eq!(Err(5), exercise_1(&input, None));
    }

    #[test]
    fn d8p2_test() {
        let input = read_input(include_str!("input/day8.txt"));
        assert_eq!(2003, exercise_1(&input, None).unwrap_err());
        assert_eq!(1984, exercise_2(&input));
    }

    #[bench]
    fn d8_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day8.txt"));
        b.iter(|| exercise_1(&input, None));
    }

    #[bench]
    fn d8_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day8.txt"));
        b.iter(|| exercise_2(&input));
    }
}
