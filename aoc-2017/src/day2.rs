fn run_algorithm(input: Vec<Vec<u32>>) -> u32 {
    let sum = input.into_iter()
        .map(|line| {
            let mut min = std::u32::MAX;
            let mut max = 0;

            for n in line {
                min = std::cmp::min(min, n);
                max = std::cmp::max(max, n); 
            }

            max - min
        }).sum();

    sum
}

fn run_algorithm2(input: Vec<Vec<u32>>) -> u32 {
    let sum = input.into_iter()
        .map(find_division)
        .sum();

    sum
}

fn find_division(row: Vec<u32>) -> u32 {
    for i in 0..row.len() {
        for j in 0..row.len() {
            if i == j { continue; }

            if row[i] % row[j] == 0 {
                return row[i] / row[j];
            }
        }
    }

    return 0;
}

fn read_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<Vec<u32>>>()
}

#[test]
fn run_examples() {
    assert_eq!(run_algorithm(read_input("5 1 9 5\n7 5 3\n2 4 6 8")), 18);
    assert_eq!(run_algorithm2(read_input("5 9 2 8\n9 4 7 3\n3 8 6 5")), 9);
}

#[test]
fn start2() {    
    println!("{}", run_algorithm2(read_input(include_str!("input/day2.txt"))));
}