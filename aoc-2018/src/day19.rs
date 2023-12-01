use crate::day19::cpu::*;

mod cpu {
    pub type Register = [i64; 6];
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
                let $a = input[1] as i64;
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
                let $b = input[2] as i64;
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

    //opcrr!(setr, a _b {a});
    //opcir!(seti, a _b {a});

    pub fn setr(input: Input, mut register: Register) -> Register {
        register[input[3]] = register[input[1]];
        register
    }

    pub fn seti(input: Input, mut register: Register) -> Register {
        register[input[3]] = input[1] as i64;
        register
    }

    opcir!(gtir, a b {(a > b) as i64});
    opcri!(gtri, a b {(a > b) as i64});
    opcrr!(gtrr, a b {(a > b) as i64});

    opcir!(eqir, a b {(a == b) as i64});
    opcri!(eqri, a b {(a == b) as i64});
    opcrr!(eqrr, a b {(a == b) as i64});
}

#[derive(Clone)]
enum Instruction {
    Binding(i32),
    Instruction(fn(Input, Register) -> Register, String, usize, usize, usize)
}

pub fn execute_exercises() {
    let samples = parse_input(include_str!("../input/day19_in.txt"));
    println!("Amount: {}", exercise_1([0; 6], samples.clone()));
    //println!("Ex2: {}", exercise_1([1, 0, 0, 0, 0, 0], samples.clone()));
    
    // The program basically takes the divisors of 10551355 and sums them.
    // We need to optimize the program...
    // Basically we are a compiler and need to optimize it.
    // Note the instructions in the register.
    // Find cycles and conditions for these cycles. (they are linear)
    // edit register to match conditions.
    //
    
    
}

//const OPCODES: &[fn(Input, Register) -> Register; 16] = &[addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| {
        if &l[0..1] == "#" {
            Instruction::Binding(l[4..5].parse().unwrap())
        } else {
            let s = l.split(' ').collect::<Vec<_>>();            
            let f = match s[0] {
                "addr" => addr,
                "addi" => addi,
                "mulr" => mulr,
                "muli" => muli,
                "banr" => banr,
                "bani" => bani,
                "borr" => borr,
                "bori" => bori,
                "setr" => setr,
                "seti" => seti,
                "gtir" => gtir,
                "gtri" => gtri,
                "gtrr" => gtrr,
                "eqir" => eqir,
                "eqri" => eqri,
                "eqrr" => eqrr,
                _ => unreachable!(s[0])
            };
            Instruction::Instruction(f, String::from(s[0]), s[1].parse().unwrap(), s[2].parse().unwrap(), s[3].parse().unwrap())

        }
    }).collect()
}



fn exercise_1(mut register: Register, input: Vec<Instruction>) -> i64 {
    let mut instruction_register = 0;
    match input[0] {
        Instruction::Binding(i) => instruction_register = i,
        _ => {}
    }

    let input = &input[1..];

    loop {
        if let Instruction::Instruction(f, r, a, b, c) = &input[register[instruction_register as usize] as usize] {            
            //print!("{:?} ({}: {}, {}, {}) ->", register, r, a, b, c);
            register = f([0, *a, *b, *c], register);
            
            //println!(" {:?}", register);
            
            if register[instruction_register as usize] + 1  < input.len() as i64 {
                register[instruction_register as usize] += 1;            
            } else {
                break;
            }
            
        } else {
            unreachable!()
        }

        use std::{thread, time};
        thread::sleep(time::Duration::from_millis(10));

        
    }
    register[0]
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day19_ex1_s1() {
       let input = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let input = parse_input(input);
        let result = exercise_1([0; 6], input);
        assert_eq!(result, 6);
    }
}