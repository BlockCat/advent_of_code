use crate::test::Bencher;

#[test]
pub fn run() {
    println!("{}", exercise_1(128392, 643281)); //1378 too low, 1417 too high
    println!("{}", exercise_2(128392, 643281));
}

fn exercise_1(min: usize, max: usize) -> usize {
    let mut count = 0;
    for a1 in 1..10 {
        for a2 in a1..10 {
            for a3 in a2..10 {
                for a4 in a3..10 {
                    for a5 in a4..10 {
                        for a6 in a5..10 {
                            let num = a1 * 100_000 + a2 * 10_000 + a3 * 1_000 + a4 * 100 + a5 * 10 + a6;
                            if num > max {
                                return count;
                            }
                            if num > min && is_valid(&num) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn exercise_2(min: usize, max: usize) -> usize {
    let mut count = 0;
    for a1 in 1..10 {
        for a2 in a1..10 {
            for a3 in a2..10 {
                for a4 in a3..10 {
                    for a5 in a4..10 {
                        for a6 in a5..10 {
                            let num = a1 * 100_000 + a2 * 10_000 + a3 * 1_000 + a4 * 100 + a5 * 10 + a6;
                            if num > max {
                                return count;
                            }
                            if num > min && is_valid_2(&num) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn is_valid(pass: &usize) -> bool {
    let mut pass = *pass;
    // check if increasing
    let mut last = 10;
    let mut double = false;
    for i in 0..6 {
        let c = pass % 10;
        if c == last {
            double = true;
        }
        last = c;
        pass /= 10;
    }
    double
}

fn is_valid_2(opass: &usize) -> bool {
    let mut pass = *opass;
    // check if increasing
    let mut last = 10;
    let mut match_count = 1;
    let mut double = false;
    for _ in 0..6 {
        let c = pass % 10;
        if c == last {   //1234(5)5          
            match_count += 1;            
        } else { // c < last
            if match_count == 2 {                
                double = true;
            }
            match_count = 1;
        }
        last = c;
        pass /= 10;
    }
    if match_count == 2 {                
        double = true;
    }

    double
}

#[test]
fn d4_test() {
    assert_eq!(is_valid(&111111), true);
    assert_eq!(is_valid(&223450), false);
    assert_eq!(is_valid(&123789), false);
    

    assert_eq!(is_valid_2(&112233), true);
    assert_eq!(is_valid_2(&123444), false);
    assert_eq!(is_valid_2(&111122), true);
    assert_eq!(is_valid_2(&223589), true);
    assert_eq!(is_valid_2(&334589), true);
    assert_eq!(is_valid_2(&556666), true);
}
#[bench]
fn d4_bench_ex1(b: &mut Bencher) {    
    b.iter(|| exercise_1(128392, 643281));
}

#[bench]
fn d4_bench_ex2(b: &mut Bencher) {
    
    b.iter(|| exercise_2(128392, 643281));
}

