use std::unimplemented;

use utils::VectorN;

#[derive(Debug, Clone, Copy)]
enum Syntax {
    Operation(char),
    ModeOpen,
    ModeClose,
    Value(isize),
}

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day18.txt"));
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

type Vector = VectorN<4>;

fn read_input(input: &str) -> Vec<Vec<Syntax>> {
    input
        .replace('(', "( ")
        .replace(')', " )")
        .lines()
        .map(read_line)
        .collect()
}

fn read_line(line: &str) -> Vec<Syntax> {
    line.split(' ')
        .map(|x| match x {
            "(" => Syntax::ModeOpen,
            ")" => Syntax::ModeClose,
            "+" => Syntax::Operation('+'),
            "*" => Syntax::Operation('*'),
            _ => Syntax::Value(x.parse().unwrap()),
        })
        .collect()
}

fn exercise_1(input: &Vec<Vec<Syntax>>) -> isize {
    input
        .iter()
        .map(|x| ex_helper1(&mut x.iter().cloned()))
        .sum()
}

fn exercise_2(input: &Vec<Vec<Syntax>>) -> isize {
    input
        .iter()
        .map(|x| ex_helper2(&mut x.iter().cloned()).evaluate())
        .sum()
}

fn ex_helper1<'a>(input: &'a mut impl Iterator<Item = Syntax>) -> isize {
    let mut value = 0;
    let mut mode = ' ';
    while let Some(syntax) = input.next() {
        match (syntax, mode) {
            (Syntax::Operation('+'), _) => mode = '+',
            (Syntax::Operation('*'), _) => mode = '*',
            (Syntax::Value(v), ' ') => value = v,
            (Syntax::Value(v), '+') => value += v,
            (Syntax::Value(v), '*') => value *= v,
            (Syntax::ModeOpen, ' ') => value = ex_helper1(input),
            (Syntax::ModeOpen, '+') => value += ex_helper1(input),
            (Syntax::ModeOpen, '*') => value *= ex_helper1(input),
            (Syntax::ModeClose, _) => return value,
            _ => unimplemented!("{:?} and {}", syntax, mode),
        }
    }
    return value;
}

struct SyntaxTree {
    value: Syntax,
    left: Option<Box<SyntaxTree>>,
    right: Option<Box<SyntaxTree>>,
}

impl SyntaxTree {
    fn new(value: Syntax) -> SyntaxTree {
        SyntaxTree {
            value,
            left: None,
            right: None,
        }
    }

    fn evaluate(&self) -> isize {
        match self.value {
            Syntax::Operation('*') => {
                self.left.as_ref().unwrap().evaluate() * self.right.as_ref().unwrap().evaluate()
            }
            Syntax::Operation('+') => {
                self.left.as_ref().unwrap().evaluate() + self.right.as_ref().unwrap().evaluate()
            }
            Syntax::Value(v) => v,
            _ => unreachable!(),
        }
    }
}

fn ex_helper2<'a>(input: &'a mut impl Iterator<Item = Syntax>) -> SyntaxTree {

    let left = match input.next() {
        Some(Syntax::Value(v)) => SyntaxTree::new(Syntax::Value(v)),
        Some(Syntax::ModeOpen) => ex_helper2(input),
        _ => unreachable!(),
    };

    let mut operation = match input.next() {
        Some(Syntax::Operation(v)) => SyntaxTree::new(Syntax::Operation(v)),
        Some(Syntax::ModeClose) => return left,
        _ => unreachable!(),
    };

    let right = match input.next() {
        Some(Syntax::Value(v)) => SyntaxTree::new(Syntax::Value(v)),
        Some(Syntax::ModeOpen) => ex_helper2(input),
        _ => unreachable!(),
    };

    // left (+*) right
    operation.left = Some(Box::new(left));
    operation.right = Some(Box::new(right));

    while let Some(syntax) = input.next() {
        match syntax {
            // 1 * 2 + 4
            Syntax::Operation('+') => {
                let r = operation.right;
                let mut nop = SyntaxTree::new(syntax);
                nop.left = r;
                nop.right = Some(Box::new(match input.next() {
                    Some(Syntax::Value(v)) => SyntaxTree::new(Syntax::Value(v)),
                    Some(Syntax::ModeOpen) => ex_helper2(input),
                    _ => unreachable!(),
                }));
                operation.right = Some(Box::new(nop));
            }
            Syntax::Operation('*') => {
                let mut nop = SyntaxTree::new(syntax);
                nop.left = Some(Box::new(operation));
                nop.right = Some(Box::new(match input.next() {
                    Some(Syntax::Value(v)) => SyntaxTree::new(Syntax::Value(v)),
                    Some(Syntax::ModeOpen) => ex_helper2(input),
                    _ => unreachable!(),
                }));
                operation = nop;
            }
            Syntax::ModeClose => {
                return operation;
            }
            _ => unreachable!("{:?}", syntax),
        }
    }

    return operation;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d18ex1() {
        let input = read_input("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(231, exercise_2(&input));
        let input = read_input("1 + (2 * 3) + (4 * (5 + 6))");
        assert_eq!(51, exercise_2(&input));
        let input = read_input("2 * 3 + (4 * 5)");
        assert_eq!(46, exercise_2(&input));
        let input = read_input("5 + (8 * 3 + 9 + 3 * 4 * 3)");
        assert_eq!(1445, exercise_2(&input));
        let input = read_input("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
        assert_eq!(669060, exercise_2(&input));
        let input = read_input("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(23340, exercise_2(&input));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day18.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day18.txt"));
        b.iter(|| exercise_2(&input));
    }
}
