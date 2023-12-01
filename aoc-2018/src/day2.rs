
#[derive(Debug, PartialEq)]
struct PartialCheckSum {
    is_double: bool,
    is_triple: bool
}

pub fn execute_exercises() {
    println!("Checksum ex1: {}", exercise_1(read_input()));
    println!("Common between boxes: {}", exercise_2(read_input()));
}

fn read_input() -> Vec<String> {
    include_str!("../input/day2_in.txt").lines().map(String::from).collect()
}

fn exercise_1(input: Vec<String>) -> i32 {
    let (doubles, triples) = input
        .into_iter()
        .map(check_id)
        .fold((0i32, 0i32), |(d, t), n| {
            (d + n.is_double as i32, t + n.is_triple as i32)
        });

    doubles * triples
}

fn check_id(id: String) -> PartialCheckSum {
    let grouping = id.bytes().fold([0u8; 26], |mut acc, c| {
        acc[c as usize - ('a' as usize)] += 1;
        acc
    });

    PartialCheckSum {
        is_double: grouping.iter().any(|v| *v == 2),
        is_triple: grouping.iter().any(|v| *v == 3)
    }
}

fn exercise_2(input: Vec<String>) -> String {
    for i in 0..input.len() {        
        for j in i..input.len() {
            if input[i].chars().zip(input[j].chars()).filter(|(a, b)| a != b).count() == 1 {
                return input[i].chars().zip(input[j].chars()).filter(|(a, b)| a == b).map(|(_, b)| b).collect();
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d2_ex1_s1() {
        assert_eq!(check_id("abcdef".to_string()), PartialCheckSum { is_double: false, is_triple: false});
        assert_eq!(check_id("bababc".to_string()), PartialCheckSum { is_double: true, is_triple: true});
        assert_eq!(check_id("abbcde".to_string()), PartialCheckSum { is_double: true, is_triple: false});
        assert_eq!(check_id("abcccd".to_string()), PartialCheckSum { is_double: false, is_triple: true});
        assert_eq!(check_id("aabcdd".to_string()), PartialCheckSum { is_double: true, is_triple: false});
        assert_eq!(check_id("abcdee".to_string()), PartialCheckSum { is_double: true, is_triple: false});
        assert_eq!(check_id("ababab".to_string()), PartialCheckSum { is_double: false, is_triple: true});
    }

    #[test]
    fn d2_ex1_s2() {
        let v = vec!("abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab").into_iter().map(|t| t.to_string()).collect();
        assert_eq!(exercise_1(v), 12);
    }

    #[test]
    fn d2_ex2_s1() {
        let v: Vec<String> = vec!("abcde","fghij","klmno","pqrst","fguij","axcye","wvxyz").into_iter().map(|t| t.to_string()).collect();
        assert_eq!(exercise_2(v), String::from("fgij"));
    }

    #[bench]
    fn d2_bench_read(b: &mut Bencher) {
        b.iter(|| {
            read_input();
        });
    }

    #[bench]
    fn d2_bench_ex1(b: &mut Bencher) {
        b.iter(|| {
            exercise_1(read_input());
        });
    }

    #[bench]
    fn d2_bench_ex2(b: &mut Bencher) {
        b.iter(|| {
            exercise_2(read_input());
        });
    }
}
