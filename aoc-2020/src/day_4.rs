use hashbrown::HashSet;
use regex::Regex;
use std::iter::FromIterator;

pub fn run() {
    let input = include_str!("input/day4bigboi.txt").trim();
    println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

fn exercise_1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut valid = 0;

    let mut current = 0;
    let mut cid_found = false;

    while let Some(start) = lines.next() {
        if start.trim().is_empty() {
            if current == 8 || (current == 7 && !cid_found) {
                valid += 1;
            }
            cid_found = false;
            current = 0;
        } else {
            cid_found |= start.contains("cid:");
            current += start.split(' ').count();
        }
    }
    if current == 8 || (current == 7 && !cid_found) {
        valid += 1;
    }
    valid
}

fn exercise_2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut valid = 0;

    let mut current = 0;
    let mut cid_found = false;

    let eycs = HashSet::<&str>::from_iter(
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter(),
    );

    while let Some(start) = lines.next() {
        if start.trim().is_empty() {
            if current == 8 || (current == 7 && !cid_found) {
                valid += 1;
            }
            cid_found = false;
            current = 0;
        } else {
            for data in start.split(' ') {
                let key = &data[0..3];
                let value = &data[4..];
                if match key {
                    "byr" => check_year(value, 1920, 2002) && value.len() == 4,
                    "iyr" => check_year(value, 2010, 2020) && value.len() == 4,
                    "eyr" => check_year(value, 2020, 2030) && value.len() == 4,
                    "hgt" => check_height(value),
                    "hcl" => check_haircolour(value),
                    "ecl" => eycs.contains(value),
                    "pid" => value.len() == 9 && value.parse::<u32>().is_ok(),
                    "cid" => {
                        cid_found = true;
                        true
                    }
                    _ => unreachable!(key),
                } {
                    current += 1;
                }
            }
        }
    }
    if current == 8 || (current == 7 && !cid_found) {
        valid += 1;
    }
    valid
}

fn check_year(d: &str, low: u16, up: u16) -> bool {
    d.parse::<u16>().map(|x| (low..=up).contains(&x)).unwrap_or(false)
}

fn check_height(d: &str) -> bool {    
    if d.ends_with("cm") {
        check_year(&d.replace("cm", ""), 150, 193)
    } else if d.ends_with("in") {
        check_year(&d.replace("in", ""), 59, 76)
    } else {
        false
    }
}

fn check_haircolour(d: &str) -> bool {
    if d.starts_with('#') && d.len() == 7 {
        for c in d.replace("#", "").chars() {
            if !('0'..='9').contains(&c) && !('a'..='f').contains(&c) {
                return false;
            }
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{exercise_1, exercise_2};
    use crate::test::Bencher;

    #[test]
    fn d4t1() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        assert_eq!(0, exercise_2(&input));
    }

    #[test]
    fn d4t12() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(4, exercise_2(&input));
    }

    #[test]
    fn d4p1_test() {
        let input = include_str!("input/day4.txt");
        assert_eq!(254, exercise_1(&input));
    }

    #[test]
    fn d4p2_test() {
        let input = include_str!("input/day4.txt");
        assert_eq!(184, exercise_2(&input));
    }

    #[bench]
    fn d4_bench_ex1(b: &mut Bencher) {
        let input = include_str!("input/day4.txt");
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d4_bench_ex2(b: &mut Bencher) {
        let input = include_str!("input/day4.txt");
        b.iter(|| exercise_2(&input));
    }

    #[bench]
    fn d4_bench_ex1bigboi(b: &mut Bencher) {
        let input = include_str!("input/day4bigboi.txt");
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d4_bench_ex2bigboi(b: &mut Bencher) {
        let input = include_str!("input/day4bigboi.txt");
        b.iter(|| exercise_2(&input));
    }
}
