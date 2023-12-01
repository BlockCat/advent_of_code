use std::iter::FromIterator;

macro_rules! read_input {
    ($input:expr) => {$input.split(',').map(|s| s.parse::<usize>().unwrap())}
}

macro_rules! read_input_2 {
    ($input:expr) => {$input.bytes()}
}

fn knot_hash_1(input: impl Iterator<Item = usize>, size: usize) -> usize {
    let mut index = 0usize;
    let mut skip = 0usize;    
    let mut list = Vec::from_iter(0..size);

    knot_hash_round(input, size, &mut index, &mut skip, &mut list)
}

fn knot_hash_round(input: impl Iterator<Item = usize>, size: usize, index: &mut usize, skip: &mut usize, list: &mut Vec<usize>) -> usize {    
    
    for length in input {
        let end = *index+length;        
        for i in 0..(length/2) {
            let start = (i + *index) % size;
            let end = (end - i - 1) % size;
            let temp = list[start];
            list[start] = list[end];
            list[end] = temp;            
        }
        *index += length + *skip;
        *skip += 1;
    }

    list[0] * list[1]
}

pub fn knot_hash_2(input: impl Iterator<Item = u8>, size: usize, rounds: usize) -> String {
    let input = input.chain(vec![17, 31, 73, 47, 23].into_iter());
    let lengths = input.map(|x| x as usize).collect::<Vec<_>>();
    let mut index = 0usize;
    let mut skip = 0usize;
    let mut list = Vec::from_iter(0..size);
    for _ in 0..rounds {
        knot_hash_round(lengths.iter().cloned(), size, &mut index, &mut skip, &mut list);
    }

    list.chunks(16).map(|chunk| {
        chunk.into_iter().fold(0, |old, new| {
            old ^ new
        })
    }).flat_map(|fh| format!("{:02x}", fh).chars().collect::<Vec<_>>())
    .collect::<String>()
}


#[test]
fn test_example() {
    assert_eq!(knot_hash_1(read_input!("3,4,1,5"), 5), 12);
    assert_eq!(&knot_hash_2(read_input_2!(""), 256, 64), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(&knot_hash_2(read_input_2!("AoC 2017"), 256, 64), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(&knot_hash_2(read_input_2!("1,2,3"), 256, 64), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(&knot_hash_2(read_input_2!("1,2,4"), 256, 64), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn run10() {
    let input = include_str!("input/day10.txt");
    let result = knot_hash_1(read_input!(input), 256);
    let second = knot_hash_2(read_input_2!(input), 256, 64);

    println!("Result: {}, hash: {}", result, second);
}