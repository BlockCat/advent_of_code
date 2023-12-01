use std::collections::HashMap;
use std::iter::Iterator;
use std::str::FromStr;

type Register = [i64; 8];

#[derive(Debug, Clone, Copy)]
enum RegValue {
    Value(i64), Reg(char)
}

impl FromStr for RegValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {    
        let c = s.chars().next().unwrap();
        if c >= 'a' && c <= 'z'  {
            Ok(RegValue::Reg(c))
        } else {
            Ok(RegValue::Value(s.parse::<i64>().map_err(|_| format!("Error: {}", s))?))
        }
    }
}


#[derive(Debug, Clone)]
enum Op {
    Set(char, RegValue), 
    Sub(char, RegValue), 
    Mul(char, RegValue),    
    Jnz(RegValue, RegValue)
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {    
        let mut line = s.split_whitespace();

        let instruction = line.next().unwrap();
        let va = line.next().unwrap();
        let vb = line.next().unwrap();

        let op = match instruction {
            "set" => Op::Set(va.parse().unwrap(), vb.parse().unwrap()),
            "sub" => Op::Sub(va.parse().unwrap(), vb.parse().unwrap()),
            "mul" => Op::Mul(va.parse().unwrap(), vb.parse().unwrap()),
            "jnz" => Op::Jnz(va.parse().unwrap(), vb.parse().unwrap()),
            c => panic!("Invalid instruction: {}", c)
        };

        Ok(op)
    }
}

struct CPU {
    registers: Register,
    instructions: Vec<Op>,
    instruction_pointer: usize,
}

impl CPU {

    fn new(instructions: Vec<Op>) -> CPU {
        CPU {
            registers: [0; 8],
            instructions: instructions,
            instruction_pointer: 0
        }
    }

    fn get_registry_value(&self, rv: RegValue) -> i64 {
        match rv {
            RegValue::Reg(c) => self.registers[(c as usize - 'a' as usize)],
            RegValue::Value(i) => i
        }
    }

    // This came for writing down the code and refactoring it.
    // g was the counter that was used for checks.
    // f was used as the checker if it was prime.
    fn algorithm() -> usize {        
        let start = 106_700usize;
        let end = 123_700usize;
        let endsqrt = 1 + (end as f32).sqrt() as usize;
        let mut primes = 0;

        // 17_000
        for number in (start..=end).step_by(17) {
            if number % 2 == 0 {                
                primes += 1;
            } else {
                for j in (3..endsqrt).step_by(2) {
                    if number % j == 0 {
                        primes += 1;
                        break;
                    }
                }
            }
        }
        primes
    }

    fn set(&mut self, register: char, rv: RegValue) {
        self.registers[register as usize - 'a' as usize] = self.get_registry_value(rv);
    }

    fn sub(&mut self, register: char, rv: RegValue) {                
        self.registers[register as usize - 'a' as usize] -= self.get_registry_value(rv);
    }

    fn mul(&mut self, register: char, rv: RegValue) {        
        self.registers[register as usize - 'a' as usize] *= self.get_registry_value(rv);
    }

    fn jnz(&mut self, x: RegValue, y: RegValue) {
        if self.get_registry_value(x) != 0 {
            self.instruction_pointer = (self.instruction_pointer as i64 + self.get_registry_value(y) - 1) as usize;
        }
    }
}

impl Iterator for CPU {
    type Item = bool;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.instruction_pointer < self.instructions.len() {
            let instruction = self.instructions[self.instruction_pointer].clone();

            match instruction {
                Op::Set(c, rv) => self.set(c, rv),
                Op::Sub(c, rv) => self.sub(c, rv),
                Op::Mul(c, rv) => self.mul(c, rv),
                Op::Jnz(c, rv) => self.jnz(c, rv),
            }

            self.instruction_pointer += 1;

            println!("{:?} {}", self.registers, self.instruction_pointer);
            match instruction {
                Op::Mul(_, _) => Some(true),
                _ => Some(false)
            }
        } else {
            None
        }
    }
}

#[test]
fn run23() { 
    let input = include_str!("input/day23.txt").lines().map(str::parse::<Op>).map(Result::unwrap).collect::<Vec<_>>();
    let cpu = CPU::new(input);

    let count = cpu.filter(|x| *x).count();

    println!("good: {} multiplications", count);


    println!("bad: {} multiplications", CPU::algorithm());
}