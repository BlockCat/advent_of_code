use crate::day16::cpu::*;
use rayon::iter::*;

mod cpu {
    pub type Register = [i32; 4];
    pub type Input = [usize; 4];

    macro_rules! opcrr {        
            ($x:ident, $a:ident $b:ident $e:block) => {
            #[inline]
            pub fn $x(input: Input, mut register: Register) -> Register {
                let $a = register[input[1]];
                let $b = register[input[2]];
                register[input[3]] = $e;
                register
            }
        };
    }

    macro_rules! opcir {        
            ($x:ident, $a:ident $b:ident $e:block) => {
            #[inline]
            pub fn $x(input: Input, mut register: Register) -> Register {
                let $a = input[1] as i32;
                let $b = register[input[2]];
                register[input[3]] = $e;
                register
            }
        };
    }

    macro_rules! opcri {        
            ($x:ident, $a:ident $b:ident $e:block) => {
            #[inline]
            pub fn $x(input: Input, mut register: Register) -> Register {
                let $a = register[input[1]];
                let $b = input[2] as i32;
                register[input[3]] = $e;
                register
            }
        };
    }

    opcrr!(addr, a b {a + b});
    opcri!(addi, a b {a + b});

    opcrr!(mulr, a b {a * b});
    opcri!(muli, a b {a * b});

    opcrr!(banr, a b {a & b});
    opcri!(bani, a b {a & b});

    opcrr!(borr, a b {(a | b)});
    opcri!(bori, a b {(a | b)});

    opcrr!(setr, a _b {a});
    opcir!(seti, a _b {a});

    opcir!(gtir, a b {(a > b) as i32});
    opcri!(gtri, a b {(a > b) as i32});
    opcrr!(gtrr, a b {(a > b) as i32});

    opcir!(eqir, a b {(a == b) as i32});
    opcri!(eqri, a b {(a == b) as i32});
    opcrr!(eqrr, a b {(a == b) as i32});
}


#[derive(Debug, Clone)]
struct Sample {
    before: Register,
    after: Register,
    input: Input
}

pub fn execute_exercises() {
    let (samples, inputs) = parse_input(include_str!("../input/day16_in.txt"));
    println!("Amount: {}", exercise_1(samples.clone()));
    println!("register 0: {}", exercise_2(samples, inputs));
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<Input>) {
    let mut iter = input.lines();
    let mut samples = Vec::with_capacity(1000);
    let mut inputs = Vec::with_capacity(2000);
    loop {
        if let Some(before) = iter.next() {            
            if before.len() == 0 {
                break;
            }
            let input: Vec<usize> = iter.next().unwrap().split(' ').map(|s| s.parse().unwrap()).collect();
            let after = iter.next().unwrap();
            iter.next();

            samples.push(Sample {
                before: [before[9..10].parse().unwrap(), before[12..13].parse().unwrap(), before[15..16].parse().unwrap(), before[18..19].parse().unwrap()],
                after: [after[9..10].parse().unwrap(), after[12..13].parse().unwrap(), after[15..16].parse().unwrap(), after[18..19].parse().unwrap()],
                input: [input[0], input[1], input[2], input[3]]
            });            
        }
    }
    iter.next();

    while let Some(i) = iter.next() {        
        let input: Vec<usize> = i.split(' ').map(|s| s.parse().unwrap()).collect();
        inputs.push([input[0], input[1], input[2], input[3]]);
    }

    (samples, inputs)
}

const OPCODES: &[fn([usize; 4], [i32; 4]) -> [i32; 4]; 16] = &[addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];

fn exercise_1(input: Vec<Sample>) -> usize {
    input.into_par_iter()
        .filter(|input| {
            OPCODES.into_iter()
            .filter(|f| f(input.input, input.before) == input.after)
            .count() >= 3
        }).count()
}

fn exercise_2(input: Vec<Sample>, program: Vec<Input>) -> i32 {
    let mapping = map_opcodes(input);

    let end_register: Register = program.into_iter()
        .fold([0, 0, 0, 0], |acc, input| {
            OPCODES[mapping[input[0]]](input, acc)
        });

    end_register[0]
}

fn map_opcodes(input: Vec<Sample>) -> [usize; 16] {
    use hashbrown::HashSet;
    let mut mapping: Vec<HashSet<usize>> = vec!((0usize..16).collect(); 16);
    for (opcode, functions) in input.iter()
        .map(|input| {  // Get opcodes and the functions they can have
            let opc = OPCODES.into_iter()
                .enumerate()
                .filter(|(_, f)| f(input.input, input.before.clone()) == input.after)
                .map(|(i, _)| i)
                .collect();            
            (input.input[0], opc)            
        }) {            
            mapping[opcode] = mapping[opcode].intersection(&functions).cloned().collect();            
        }
    
    let mut real_mapping = [0usize; 16];
    // This should probably be done with a queue and all.
    loop {
        for i in 0..mapping.len() {
            if mapping[i].len() == 1 {
                real_mapping[i] = *mapping[i].iter().next().unwrap();                

                for j in 0..mapping.len() {
                    mapping[j].remove(&real_mapping[i]);
                }
            }            
        }
        if mapping.iter().filter(|m| m.len() > 0).count() == 0 {
            break;
        }
    }
    real_mapping
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day16_ex1_s1() {
        let input = Sample {
            before: [3, 2, 1, 1],
            after: [3, 2, 2, 1],
            input: [9, 2, 1, 2]
        };

        let amount = OPCODES.into_iter()
            .filter(|f| f(input.input, input.before) == input.after)
            .count();

        println!("{}", amount);
        println!("{:?}", mulr(input.input, input.before));
        println!("{:?}", addi(input.input, input.before));
        println!("{:?}", seti(input.input, input.before));        
    }   

    #[test]
    fn day16_ex1_s2() {
        let (samples, _) = parse_input(include_str!("../input/day16_in.txt"));
        
        assert_eq!(exercise_1(samples), 580);
    }

    
    #[test]
    fn day16_ex2_s1() {
        let (samples, inputs) = parse_input(include_str!("../input/day16_in.txt"));        
        assert_eq!(exercise_2(samples, inputs), 537);
    }

    #[bench]
    fn day16_bench_ex1(b: &mut Bencher) {
        let (samples, _) = parse_input(include_str!("../input/day16_in.txt"));
        
        b.iter(move || exercise_1(samples.clone()));
    }

    #[bench]
    fn day16_bench_ex2(b: &mut Bencher) {
        let (samples, inputs) = parse_input(include_str!("../input/day16_in.txt"));
        
        b.iter(move ||exercise_2(samples.clone(), inputs.clone()));
    }
}