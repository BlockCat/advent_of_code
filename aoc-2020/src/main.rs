#![feature(test)]
#![feature(iterator_fold_self)]
#![feature(min_const_generics)]
#![feature(is_sorted)]
extern crate hashbrown;
extern crate test;
extern crate utils;

#[allow(dead_code)]
mod day_24;
// mod day_9;
// mod day_8;
// mod day_7;
// mod day_6;
// mod day_5;
// mod day_4;
// mod day_3;
// mod day_2;
// mod day_1;

fn main() {
    day_24::run();
}
