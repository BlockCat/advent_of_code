// #![feature(test)]
// include!("../examples/day_13_simd.rs");

// extern crate test;
// use self::test::Bencher;

// #[bench]
// fn d13_simd_input_bench(b: &mut Bencher) {
//     b.iter(|| input(include_str!("../input/day_13.txt")));
// }

// #[bench]
// fn d13_simd_exercise_1_bench(b: &mut Bencher) {
//     let input = input(include_str!("../input/day_13.txt"));
//     b.iter(|| exercise_1(&input));
// }

// #[bench]
// fn d13_simd_exercise_2_bench(b: &mut Bencher) {
//     let input = input(include_str!("../input/day_13.txt"));
//     b.iter(|| exercise_2(&input));
// }