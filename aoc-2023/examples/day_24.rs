// use std::ops::Add;

// use aoc_2023::vector::{Vector2, Vector3};
// use rayon::iter::{ParallelBridge, ParallelIterator};
// use z3::{
//     ast::{Ast, Int},
//     Config, Context, Solver,
// };

// type InputType = Vec<Stone>;

// const LOWER_BOUND: usize = 200000000000000;
// const UPPER_BOUND: usize = 400000000000000;

// pub fn main() {
//     let input = parse(include_str!("../input/day_24.txt"));

//     println!(
//         "Exercise 1: {}",
//         exercise_1(input.clone(), LOWER_BOUND, UPPER_BOUND)
//     );
//     println!("Exercise 2: {}", exercise_2(input));
// }

// #[test]
// fn test() {
//     let input = parse(include_str!("../input/test.txt"));

//     // println!("Exercise 1: {}", exercise_1(input.clone(), 7, 27));
//     println!("Exercise 2: {}", exercise_2(input));
//     // assert_eq!(62, exercise_1(input.clone()));
//     // assert_eq!(952408144115, exercise_2(input));
// }

// fn parse<'a>(input: &'a str) -> InputType {
//     input.lines().map(parse_line).collect()
// }

// fn parse_line(line: &str) -> Stone {
//     let (pos, vel) = line.split_once(" @ ").unwrap();

//     let pos = pos
//         .split(", ")
//         .map(|s| s.parse::<isize>().unwrap())
//         .collect::<Vec<_>>();
//     let vel = vel
//         .split(", ")
//         .map(|s| s.trim().parse::<isize>().unwrap())
//         .collect::<Vec<_>>();

//     Stone {
//         position: Vector3::new([pos[0], pos[1], pos[2]]),
//         velocity: Vector3::new([vel[0], vel[1], vel[2]]),
//     }
// }

// fn exercise_1(input: InputType, lower_bound: usize, upper_bound: usize) -> usize {
//     let mut counter = 0;
//     for i in 0..input.len() {
//         let stone_1 = &input[i];
//         for j in i + 1..input.len() {
//             let stone_2 = &input[j];

//             if let Some(_) = calculate_2d_collission(
//                 stone_1,
//                 stone_2,
//                 lower_bound as isize,
//                 upper_bound as isize,
//             ) {
//                 counter += 1;
//             }
//         }
//     }

//     counter
// }

// fn exercise_2(input: InputType) -> isize {
//     // help(&input);
//     // calculate_axis(&input)

//     let cfg = Config::new();
//     let ctx = Context::new(&cfg);

//     let mut solver = Solver::new(&ctx);

//     let search_x_0 = Int::new_const(&ctx, "search_x_0");
//     let search_y_0 = Int::new_const(&ctx, "search_y_0");

//     let search_x_1 = Int::new_const(&ctx, "search_x_1");
//     let search_y_1 = Int::new_const(&ctx, "search_y_1");

//     let search_x_2 = Int::new_const(&ctx, "search_x_2");
//     let search_y_2 = Int::new_const(&ctx, "search_y_2");

//     for (i, stone) in input.iter().enumerate() {
//         let a_0 = Int::from_i64(&ctx, stone.position[0] as i64);
//         let a_1 = Int::from_i64(&ctx, stone.position[1] as i64);
//         let a_2 = Int::from_i64(&ctx, stone.position[2] as i64);

//         let c_0 = Int::new_const(&ctx, format!("c_{}", i));

//         let b_0 = Int::from_i64(&ctx, stone.velocity[0] as i64);
//         let b_1 = Int::from_i64(&ctx, stone.velocity[1] as i64);
//         let b_2 = Int::from_i64(&ctx, stone.velocity[2] as i64);

//         let left_part_0 = &search_x_0 + &search_y_0 * &c_0;
//         let right_part_0 = &a_0 + &b_0 * &c_0;

//         let left_part_1 = &search_x_1 + &search_y_1 * &c_0;
//         let right_part_1 = &a_1 + &b_1 * &c_0;

//         let left_part_2 = &search_x_2 + &search_y_2 * &c_0;
//         let right_part_2 = &a_2 + &b_2 * &c_0;

//         solver.assert(&left_part_0._eq(&right_part_0));
//         solver.assert(&left_part_1._eq(&right_part_1));
//         solver.assert(&left_part_2._eq(&right_part_2));
//     }

//     solver.check();

//     let model = solver.get_model().unwrap();

//     let x = model.eval(&search_x_0, true).unwrap().as_i64().unwrap();
//     let y = model.eval(&search_x_1, true).unwrap().as_i64().unwrap();
//     let z = model.eval(&search_x_2, true).unwrap().as_i64().unwrap();

//     x as isize + y as isize + z as isize
//     // let v = model.eval(&search_y_0, true).unwrap().as_i64().unwrap();

//     // println!("solved: {} {}", pos, v);

//     // search_pos(&input, pos as isize, v as isize).unwrap()
// }

// fn calculate_2d_collission(
//     stone_1: &Stone,
//     stone_2: &Stone,
//     lower_bound: isize,
//     upper_bound: isize,
// ) -> Option<usize> {
//     let pos_1 = Vector2::new([stone_1.position[0], stone_1.position[1]]);
//     let pos_2 = Vector2::new([stone_2.position[0], stone_2.position[1]]);
//     let vel_1 = Vector2::new([stone_1.velocity[0], stone_1.velocity[1]]);
//     let vel_2 = Vector2::new([stone_2.velocity[0], stone_2.velocity[1]]);

//     let b_1 = vel_1[1] as f64 / vel_1[0] as f64;
//     let a_1 = pos_1[1] as f64 - b_1 * pos_1[0] as f64;

//     let b_2 = vel_2[1] as f64 / vel_2[0] as f64;
//     let a_2 = pos_2[1] as f64 - b_2 * pos_2[0] as f64;

//     // find intersection
//     let x = (a_2 - a_1) / (b_1 - b_2);
//     let y = b_1 * x + a_1;

//     let t_1 = if x > pos_1[0] as f64 && vel_1[0] >= 0 {
//         true
//     } else if x < pos_1[0] as f64 && vel_1[0] <= 0 {
//         true
//     } else {
//         false
//     };
//     let t_2 = if x > pos_2[0] as f64 && vel_2[0] >= 0 {
//         true
//     } else if x < pos_2[0] as f64 && vel_2[0] <= 0 {
//         true
//     } else {
//         false
//     };

//     if x >= lower_bound as f64 && x <= upper_bound as f64 && t_1 && t_2 {
//         if y >= lower_bound as f64 && y <= upper_bound as f64 {
//             return Some(x as usize);
//         }
//     }

//     None
// }

// fn search_pos(input: &InputType, pos: isize, v: isize) -> Option<usize> {
//     let stone_0 = &input[0];
//     let stone_1 = &input[1];

//     let col_0 = calculate_collide(pos, v, stone_0.position[0], stone_0.velocity[0])?;
//     let col_1 = calculate_collide(pos, v, stone_1.position[0], stone_1.velocity[0])?;
//     let diff_p = col_0 - col_1;

//     let pos_0 = [
//         stone_0.position[0] as f64 + stone_0.velocity[0] as f64 * col_0,
//         stone_0.position[1] as f64 + stone_0.velocity[1] as f64 * col_0,
//         stone_0.position[2] as f64 + stone_0.velocity[2] as f64 * col_0,
//     ];

//     let pos_1 = [
//         stone_1.position[0] as f64 + stone_1.velocity[0] as f64 * col_1,
//         stone_1.position[1] as f64 + stone_1.velocity[1] as f64 * col_1,
//         stone_1.position[2] as f64 + stone_1.velocity[2] as f64 * col_1,
//     ];

//     let target_diff = [
//         (pos_0[0] - pos_1[0]) / diff_p,
//         (pos_0[1] - pos_1[1]) / diff_p,
//         (pos_0[2] - pos_1[2]) / diff_p,
//     ];

//     if input.iter().all(|stone| {
//         if let Some(p) = calculate_collide(pos, v, stone.position[0], stone.velocity[0]) {
//             let diff_p = (p - col_0) as f64;
//             let spos = [
//                 stone.position[0] as f64 + stone.velocity[0] as f64 * p - diff_p * target_diff[0],
//                 stone.position[1] as f64 + stone.velocity[1] as f64 * p - diff_p * target_diff[1],
//                 stone.position[2] as f64 + stone.velocity[2] as f64 * p - diff_p * target_diff[2],
//             ];
//             spos == pos_0
//         } else {
//             false
//         }
//     }) {
//         let pos_a = pos_0[0] - col_0 * target_diff[0];
//         let pos_b = pos_0[1] - col_0 * target_diff[1];
//         let pos_c = pos_0[2] - col_0 * target_diff[2];

//         println!("pos: {:?}", (pos_a, pos_b, pos_c));
//         println!("vel: {:?}", target_diff);

//         return Some((pos_a + pos_b + pos_c) as usize);
//     }

//     // let r = input
//     //     .iter()
//     //     .filter_map(|stone| {
//     //         if let Some(p) = calculate_collide(pos, v, stone.position[0], stone.velocity[0]) {
//     //             let pos_0 = stone.position[0] as f64 + stone.velocity[0] as f64 * p;
//     //             let pos_1 = stone.position[1] as f64 + stone.velocity[1] as f64 * p;
//     //             let pos_2 = stone.position[2] as f64 + stone.velocity[2] as f64 * p;
//     //             Some((pos_0, pos_1, pos_2, p))
//     //         } else {
//     //             None
//     //         }
//     //     })
//     //     .collect::<Vec<_>>();

//     // if r.len() == input.len() {
//     //     let mut found = true;
//     //     for i in 0..r.len() {
//     //         let (x_0, y_0, z_0, p_0) = r[i];
//     //         for j in i + 1..r.len() {
//     //             let (x_1, y_1, z_1, p_1) = r[j];
//     //             let diff_p = (p_0 - p_1);
//     //             let diff_x = (x_0 - x_1) / diff_p;
//     //             let diff_y = (y_0 - y_1) / diff_p;
//     //             let diff_z = (z_0 - z_1) / diff_p;

//     //             if diff_x != target_diff_x || diff_y != target_diff_y || diff_z != target_diff_z {
//     //                 found = false;
//     //             }
//     //         }
//     //     }

//     //     if found {
//     //         println!("{} {}", pos, v);
//     //         let reee = r[0];
//     //         println!(
//     //             "{} {} ({}, {}, {})",
//     //             pos, v, target_diff_x, target_diff_y, target_diff_z
//     //         );

//     //         let pos_0 = reee.0 - reee.3 * target_diff_x;
//     //         let pos_1 = reee.1 - reee.3 * target_diff_y;
//     //         let pos_2 = reee.2 - reee.3 * target_diff_z;

//     //         println!("ree: {:?}", (pos_0, pos_1, pos_2));

//     //         return Some(pos as usize);
//     //     }
//     // }

//     None
// }

// fn calculate_axis(input: &InputType) -> usize {
//     for pos in 0.. {
//         let res = (0..10000isize).par_bridge().find_map_any(|v| {
//             if let Some(r) = search_pos(input, pos, v) {
//                 return Some(r);
//             }
//             if let Some(r) = search_pos(input, -pos, v) {
//                 return Some(r);
//             }
//             if let Some(r) = search_pos(input, pos, -v) {
//                 return Some(r);
//             }
//             if let Some(r) = search_pos(input, -pos, -v) {
//                 return Some(r);
//             }
//             None
//         });

//         if let Some(res) = res {
//             return res;
//         }
//     }

//     unreachable!()
// }

// // fn pos_vel_pairs_x(input: &InputType) -> Vec<(isize, isize)> {
// //     let stone_0 = &input[0];
// //     let stone_1 = &input[1];

// //     let mut a_0 = stone_0.position[0];
// //     let mut a_1 = stone_1.position[0];

// //     let b_0 = stone_0.velocity[0];
// //     let b_1 = stone_1.velocity[0];

// //     let a_d = a_0 - a_1;
// //     let b_d = b_0 - b_1;
// // }

// fn calculate_collide(a0: isize, b0: isize, a1: isize, b1: isize) -> Option<f64> {
//     let t = (a1 - a0) as f64 / (b0 - b1) as f64;

//     if t >= 0.0 && t.is_finite() {
//         // && t as isize as f64 == t {
//         return Some(t);
//     }

//     None
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct Stone {
//     position: Vector3,
//     velocity: Vector3,
// }
