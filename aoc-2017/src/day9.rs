
fn algorithm1(mut iterator: impl Iterator<Item = char>) -> (u32, u32) {
    let mut score = 0;
    let mut group = 0;    
    let mut removed = 0;
    while let Some(token) = iterator.next() {
        
        match token {
            '{' => group += 1,
            '}' => {
                score += group;
                group -= 1;
            },
            ',' => {},
            '<' => removed += garbage(&mut iterator),
            c => panic!("{} is not implemented", c)
        }
    
    }
    (score, removed)
}

// Exhaust garbage
fn garbage(iterator: &mut impl Iterator<Item = char>) -> u32 {
    let mut removed = 0;
    while let Some(token) = iterator.next() {
        match token {
            '!' => {iterator.next();},
            '>' => return removed,
            _ => removed += 1
        }
    }

    unreachable!()
}



#[test]
fn test_examples() {
    assert_eq!(algorithm1("{}".chars()), (1, 0));
    assert_eq!(algorithm1("{{{}}}".chars()), (6, 0));
    assert_eq!(algorithm1("{{},{}}".chars()), (5, 0));
    assert_eq!(algorithm1("{{{},{},{{}}}}".chars()), (16, 0));
    assert_eq!(algorithm1("{<a>,<a>,<a>,<a>}".chars()), (1, 4));
    assert_eq!(algorithm1("{{<ab>},{<ab>},{<ab>},{<ab>}}".chars()), (9, 8));
    assert_eq!(algorithm1("{{<!!>},{<!!>},{<!!>},{<!!>}}".chars()), (9, 0));
    assert_eq!(algorithm1("{{<a!>},{<a!>},{<a!>},{<ab>}}".chars()), (3, 17));
}

#[test]
fn run9() {
    let input = include_str!("input/day9.txt");
    let (score, removed) = algorithm1(input.chars());
    println!("Score: {}, Removed: {}", score, removed);
}


