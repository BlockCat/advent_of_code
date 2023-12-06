pub fn main() {
    let input = vec![(38, 234), (67, 1027), (76, 1157), (73, 1236)];
    println!("Exercise 1: {}", exercise_1(input));
    let input2 = vec![(38677673, 234102711571236)];
    println!("Exercise 2: {}", exercise_1(input2));
}

fn exercise_1(input: Vec<(usize, usize)>) -> usize {
    input
        .into_iter()
        .map(|(time, record)| ways_to_beat(time, record))
        .product()
}

fn ways_to_beat(time: usize, record: usize) -> usize {
    let b = time as f64;

    let d = ((time.pow(2) - 4 * record) as f64).sqrt();
    let left = (-b + d) / (-2f64);
    let right = (-b - d) / (-2f64);

    let left = left.ceil() as usize;
    let right = right.floor() as usize;

    right - left + 1
}
