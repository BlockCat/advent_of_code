use crate::day21::cpu::*;

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
    let samples = parse_input(include_str!("../input/day21_in.txt"));
    //println!("Amount: {}", exercise_1([0; 6], samples.clone()));

    
    let result = exercise_2([0; 6], samples.clone(), 0);

    println!("Part 1: {:?}", result.iter().min_by_key(|(value, &counter)| counter).unwrap());
    println!("Part 2: {:?}", result.iter().max_by_key(|(value, &counter)| counter).unwrap());
    //println!("Ex2: {}", exercise_1([0, 0, 0, 0, 0, 0], samples.clone(), 0));
    
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



fn exercise_1(mut register: Register, input: Vec<Instruction>, mut counter: u64) -> i64 {
    let mut instruction_register = 0;
    match input[0] {
        Instruction::Binding(i) => instruction_register = i,
        _ => {}
    }

    let input = &input[1..];

    let mut laster: (u64, Register) = (counter, register.clone());
    
    loop {
        if let Instruction::Instruction(f, r, a, b, c) = &input[register[instruction_register as usize] as usize] {            
            match &r[..] {
                "gtir" | "gtri" | "gtrr" | "eqir" |"eqri" | "eqrr"  => {                    
                    if &r[..] == "eqrr" {
                        return register[1];
                    }
                    if laster.1[instruction_register as usize] == register[instruction_register as usize] {
                        let dif = register.iter().zip(laster.1.iter()).map(|(a, b)| a - b).collect::<Vec<_>>();
                       
                        match &r[..] {
                            "gtrr" => {                       
                                let times_to_go = (laster.1[*b as usize] - laster.1[*a as usize]) / (dif[*a as usize] - dif[*b as usize]);
                                
                                counter += (counter - laster.0) * times_to_go as u64;

                                for (i, m) in register.clone().iter().zip(dif.iter()).map(|(a, b)| a + b*times_to_go).enumerate() {
                                    register[i] = m as i64;
                                }                                
                            }

                            _ => panic!()
                        }
                        
                    }                    
                    laster = (counter, register.clone());

                    register = f([0, *a, *b, *c], register);
                    counter += 1;
                }
                _ => {                    
                    register = f([0, *a, *b, *c], register);
                    counter += 1;        
                }
            }

            //println!(" {:?}: {}", register, counter);
            
            if register[instruction_register as usize] + 1  < input.len() as i64 {
                register[instruction_register as usize] += 1;            
            } else {
                break;
            }
            
        } else {
            unreachable!()
        }        
    }

    println!("counter: {}", counter);
    unreachable!()
    //register[0]
}

use hashbrown::HashMap;
fn exercise_2(mut register: Register, input: Vec<Instruction>, mut counter: u64) -> HashMap<i64, u64> {
    

    let mut instruction_register = 0;
    match input[0] {
        Instruction::Binding(i) => instruction_register = i,
        _ => {}
    }

    let input = &input[1..];

    let mut laster: (u64, Register) = (counter, register.clone());
    let mut recurring = HashMap::new();    

    let mut last_regi = register[1];
    loop {
        if let Instruction::Instruction(f, r, a, b, c) = &input[register[instruction_register as usize] as usize] {            
            //println!("{}: {:?} ({}: {}, {}, {}) ->", counter, register, r, a, b, c);
            
            match &r[..] {
                "gtir" | "gtri" | "gtrr" | "eqir" |"eqri" | "eqrr"  => {
                    /*if &r[..] == "eqrr" {
                        return register[1];
                    }*/
                    // Check if this one is the same instruction as last one
                    if &r[..] == "eqrr" {
                        
                        if recurring.contains_key(&register[1]) {
                            return recurring;
                        } else {
                            recurring.insert(register[1], counter);
                        }
                    }
                    
                    if laster.1[instruction_register as usize] == register[instruction_register as usize] {
                        let dif = register.iter().zip(laster.1.iter()).map(|(a, b)| a - b).collect::<Vec<_>>();
                        //println!("{}: {:?} -> {}: {:?} = {:?}", laster.0, laster.1, counter, register, dif);

                        match &r[..] {
                            "gtrr" => {
                                
                                // Increasing line starting y_1 = laster[a] + i * dif[a]
                                // increasing line starting y_2 = laster[b] + i * dif[b]
                                // Find intersection
                                // (laster[a] - laster[b]) + i * (dif[a] - dif[b]) = 0                                
                                // find i
                                // i = (laster[b] - laster[a]) / (dif[a] - dif[b])
                                let times_to_go = (laster.1[*b as usize] - laster.1[*a as usize]) / (dif[*a as usize] - dif[*b as usize]);
                                //println!("For a gtrr between {} > {}, needed: {}", register[*a as usize], register[*b as usize], times_to_go);
                                counter += (counter - laster.0) * times_to_go as u64;

                                for (i, m) in register.clone().iter().zip(dif.iter()).map(|(a, b)| a + b*times_to_go).enumerate() {
                                    register[i] = m as i64;
                                }
                                //continue;
                            }

                            _ => panic!()
                        }
                        
                    }                    
                    laster = (counter, register.clone());

                    register = f([0, *a, *b, *c], register);
                    counter += 1;
                }
                _ => {                    
                    register = f([0, *a, *b, *c], register);
                    counter += 1;        
                }
            }            
            
            
            if register[instruction_register as usize] + 1  < input.len() as i64 {
                register[instruction_register as usize] += 1;            
            } else {
                break;
            }
            
        } else {
            unreachable!()
        }        
        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(1));

        
    }

    println!("counter: {}", counter);
    unreachable!()
    //register[0]
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[bench]
    fn day21_bench_ex1(b: &mut Bencher) {
        let samples = parse_input(include_str!("../input/day21_in.txt"));
        b.iter(move || exercise_1([0; 6], samples.clone(), 0));       
    }

    #[bench]
    fn day21_bench_ex2(b: &mut Bencher) {
        let samples = parse_input(include_str!("../input/day21_in.txt"));
        b.iter(move ||
            exercise_2([0; 6], samples.clone(), 0).into_iter().max_by_key(|(value, counter)| *counter).unwrap()
        );
    }
}