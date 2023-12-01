use crate::test::Bencher;
use hashbrown::HashMap;
use hashbrown::HashSet;
use std::iter::FromIterator;

type Input<'a> = Tree<'a>;
type Output = usize;

#[derive(Clone)]
struct TreeNode {
    pub parent: usize,
    pub children: Vec<usize>,
}

#[derive(Clone)]
struct IntTree {
    pub nodes: Vec<TreeNode>,
    pub root: usize,
    pub you: usize,
    pub san: usize,
}

#[derive(Clone)]
struct Tree<'a> {
    pub nodes: HashMap<&'a str, Vec<&'a str>>,
    pub parents: HashMap<&'a str, &'a str>,
}

#[test]
pub fn run() {
    let input = read_input(include_str!("input/day6.txt"));
    println!("ex1: {}", exercise_1(input.clone()));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(input: Tree) -> Output {
    let mut queue = Vec::with_capacity(input.parents.len());    
    queue.push((0, "COM"));

    let mut sum = 0usize;
    while let Some((d, node)) = queue.pop() {
        sum += d;
        if let Some(children) = input.nodes.get(&node) {
            for k in children.iter().map(|c| (d + 1, c.clone())) {
                queue.push(k);
            }
        }
    }

    sum
    
}


fn exercise_2(input: Tree) -> Output {
    let san =
        std::iter::successors(Some(&"YOU"), |&x| input.parents.get(x)).collect::<HashSet<_>>();
    let you =
        std::iter::successors(Some(&"SAN"), |&x| input.parents.get(x)).collect::<HashSet<_>>();
    san.symmetric_difference(&you).count() - 2
}

fn read_input<'a>(input: &'a str) -> Input<'a> {
    let pairs = input.lines().map(|line| {
        let mut s = line.split(')');
        let mass = s.next().unwrap();
        let orb = s.next().unwrap();
        (mass, orb)
    });
    let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    let mut parents: HashMap<&'a str, &'a str> = HashMap::new();

    for (com, orb) in pairs {
        parents.insert(orb, com);
        map.entry(com)
            .and_modify(|f| f.push(orb))
            .or_insert(vec![orb]);
    }

    Tree {
        nodes: map,
        parents,
    }
}

#[test]
fn d6_test() {
    let test = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    assert_eq!(exercise_1(read_input(test)), 42);
    let test = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
    assert_eq!(exercise_2(read_input(test)), 4);
    assert_eq!(
        exercise_1(read_input(include_str!("input/day6.txt"))),
        130681
    );
    assert_eq!(exercise_2(read_input(include_str!("input/day6.txt"))), 313);
}

#[bench]
fn d6_bench_ex1(b: &mut Bencher) {
    let input = read_input(include_str!("input/day6.txt"));
    b.iter(|| exercise_1(input.clone()));
}

#[bench]
fn d6_bench_ex2(b: &mut Bencher) {
    let input = read_input(include_str!("input/day6.txt"));
    b.iter(|| exercise_2(input.clone()));
}

#[bench]
fn d6_bench_parse(b: &mut Bencher) {
    b.iter(|| read_input(include_str!("input/day6.txt")));
}
