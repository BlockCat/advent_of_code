// #![feature(test, portable_simd)]
// include!("../examples/day_03_simd.rs");
// extern crate test;
// use self::test::Bencher;


// #[bench]
// fn d03_input(b: &mut Bencher) {
//     b.iter(|| {
//         include_str!("../input/day03.txt")
//             .lines()
//             .map(decode_binary)
//             .collect::<Vec<_>>()
//     });
// }

// #[bench]
// fn d03_ex1_simd(b: &mut Bencher) {
//     let numbers = include_str!("../input/day03.txt")
//         .lines()
//         .map(decode_binary)
//         .collect::<Vec<_>>();
//     b.iter(|| exercise_1(&numbers));
// }

// #[bench]
// fn d03_ex2_simd(b: &mut Bencher) {
//     let numbers = include_str!("../input/day03.txt")
//         .lines()
//         .map(decode_binary)
//         .collect::<Vec<_>>();
//         b.iter(|| exercise_2(&numbers));
// }
