fn read_input(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect::<Vec<_>>()
}

fn algorithm1(mut input: Vec<i32>) -> usize {
    let mut counter = 0usize;
    let mut steps = 0usize;
    while counter < input.len() {
        steps += 1;
        let passer = input[counter];
        input[counter] += 1;
        counter = (counter as i32 + passer) as usize;
    }

    return steps;
}

fn algorithm2(mut input: Vec<i32>) -> usize {
    let mut counter = 0usize;
    let mut steps = 0usize;
    while counter < input.len() {
        steps += 1;
        let passer = input[counter];
        if passer >= 3 {
            input[counter] -= 1;
        } else {
            input[counter] += 1;
        }
        counter = (counter as i32 + passer) as usize;
    }

    return steps;
}

#[test]
fn test_examples() {
    assert_eq!(algorithm1(vec!(0, 3, 0, 1, -3)), 5);
    assert_eq!(algorithm2(vec!(0, 3, 0, 1, -3)), 10);
}

#[test]
fn run5() {
    let input = include_str!("input/day5.txt");

    println!("{}", algorithm1(read_input(input)));
    println!("{}", algorithm2(read_input(input)));
}