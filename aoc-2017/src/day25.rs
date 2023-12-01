use std::boxed::Box;
use crate::state;
use cpu::*;

mod cpu {
    use std::boxed::Box;
    use std::collections::HashMap;

    pub type Tape = HashMap<isize, bool>;
    
    pub trait State {
        fn next(&self, index: &mut isize, tape: &mut Tape) -> Box<dyn State>;
    }   

    #[macro_export]
    macro_rules! state {
        ($name:ident, 0 => ($n0:expr, $i0:expr, $zero:ident), 1 => ($n1:expr, $i1:expr, $one:ident)) => {
            struct $name;
            impl $name {
                fn new() -> $name {
                    $name
                }
            }

            impl State for $name {
                fn next(&self, index: &mut isize, tape: &mut Tape) -> Box<dyn State> {
                    let v = tape.entry(*index).or_insert(false);                    
                    if *v {
                        *v = $n1;
                        *index += $i1;
                        Box::new($one::new())
                    } else {
                        *v = $n0;
                        *index += $i0;
                        Box::new($zero::new())
                    }
                }
            }

        };
    }



    pub struct CPU;

    impl CPU {
        pub fn run_1(start: Box<dyn State>, steps: usize) -> usize {
            let mut index = 0isize;
            let mut tape = Tape::new();

            let mut state = start;

            for _ in 0..steps {
                state = (*state).next(&mut index, &mut tape);
            }

            tape.values().filter(|x| **x).count()
        }
    }
}

state!(A, 0 => (true,   1, B), 1 => (false,  1, C));
state!(B, 0 => (false, -1, A), 1 => (false,  1, D));
state!(C, 0 => (true,   1, D), 1 => (true,   1, A));
state!(D, 0 => (true,  -1, E), 1 => (false, -1, D));
state!(E, 0 => (true,   1, F), 1 => (true,  -1, B));
state!(F, 0 => (true,   1, A), 1 => (true,   1, E));

#[test]
fn run25() {
    println!("Checksum: {}", CPU::run_1(Box::new(A), 12399302));
}

mod tests {
    #[macro_use] 
    use super::cpu::*;        
    use crate::state;
    use std::boxed::Box;

    state!(A, 0 => (true, 1, B), 1 => (false, -1, B));    
    state!(B, 0 => (true, -1, A), 1 => (true, 1, A));

    #[test]
    fn test_examples() {
        assert_eq!(CPU::run_1(Box::new(A), 6), 3);
    }
    
}