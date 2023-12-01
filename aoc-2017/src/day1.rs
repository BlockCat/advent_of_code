
fn run() {

}


fn run_algorithm(slice: Vec<u32>) -> u32 {
    let first = slice[0];    
    let mut sum = slice.windows(2)
        .filter(|v| v[0] == v[1])
        .map(|v| v[0])
        .sum();
    
    if slice[0] == *slice.last().unwrap() {
        sum += slice[0];
    }

    sum
}

fn run_algorithm2(slice: Vec<u32>) -> u32 {
    let offset = slice.len() / 2;
    let size = slice.len();
    let mut sum = 0u32;
    for i in 0..slice.len() {
        let a = slice[i];
        let b = slice[(i + offset) % size];
        if a == b {
            sum += a;
        }
    }
    sum
}

fn read_input(input: &str) -> Vec<u32> {
    input.chars().map(|x| (x as u32 - '0' as u32) ).collect::<Vec<u32>>()
}

#[test]
fn test_example() {
    assert_eq!(run_algorithm(vec!(1,1,2,2)), 3);
    assert_eq!(run_algorithm(vec!(1,1,1,1)), 4);
    assert_eq!(run_algorithm(vec!(1,2,3,4)), 0);
    assert_eq!(run_algorithm(vec!(9,1,2,1,2,1,2,9)), 9);
    assert_eq!(run_algorithm2(read_input("1212")), 6);
    assert_eq!(run_algorithm2(read_input("1221")), 0);
    assert_eq!(run_algorithm2(read_input("123425")), 4);
    assert_eq!(run_algorithm2(read_input("123123")), 12);
    assert_eq!(run_algorithm2(read_input("12131415")), 4);
}
#[test]
fn run1() {
    println!("{}", run_algorithm(read_input(include_str!("input/day1.txt"))));
    println!("{}", run_algorithm2(read_input(include_str!("input/day1.txt"))));
}