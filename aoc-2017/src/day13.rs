const INPUT: &'static str = include_str!("input/day13.txt");


fn read_input(input: &str) -> Vec<(usize, usize)> {
    input.lines().map(read_line).collect::<Vec<_>>()
}

fn read_line(line: &str) -> (usize, usize) {
    let mut iterator = line.split(':').map(str::trim);

    let layer = iterator.next().map(str::parse::<usize>).unwrap().expect("Could not parse layer");
    let depth = iterator.next().map(str::parse::<usize>).unwrap().expect("Could not parse depth");

    (layer, depth)
}


fn calculate_location(time: usize, depth: usize) -> usize {
    let index = time % ((depth * 2) - 2);

    if index >= depth {        
        2 * depth - 2 - index
    } else {
        index
    }
}

fn algorithm1(input: &Vec<(usize, usize)>, delay: usize) -> (usize, bool) {
    let mut caught = false;
    (input.iter()
        .map(|(a, b)| {
            if calculate_location(a + delay, *b) == 0 {
                caught = true;
                a * b
            } else { 
                0
            }
        }).sum::<usize>(), caught)
}

fn algorithm2(input: &Vec<(usize, usize)>) -> usize {
    for i in 0.. {
        if algorithm1(input, i) == (0, false) {
            return i;
        }
    }

    unreachable!()
}

#[test]
fn test_location() {
    assert_eq!(calculate_location(0, 5), 0);
    assert_eq!(calculate_location(1, 5), 1);
    assert_eq!(calculate_location(2, 5), 2);
    assert_eq!(calculate_location(3, 5), 3);
    assert_eq!(calculate_location(4, 5), 4);
    assert_eq!(calculate_location(5, 5), 3);
    assert_eq!(calculate_location(6, 5), 2);
    assert_eq!(calculate_location(7, 5), 1);
    assert_eq!(calculate_location(8, 5), 0);
}

#[test]
fn test_examples() {
    let input = read_input(r"0: 3
1: 2
4: 4
6: 4");
    assert_eq!(algorithm1(&input, 0), (24, true));
    assert_eq!(algorithm2(&input), 10);
}

#[test]
fn run13() {
    let input = read_input(INPUT);
    println!("Score with 0: {}, Min delay: {}", algorithm1(&input, 0).0, algorithm2(&input));
}