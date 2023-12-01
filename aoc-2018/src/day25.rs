type Location = (isize, isize, isize, isize);

#[derive(Debug, Clone)]
struct Star {
    location: Location,
    origin_star: Option<usize>
}

pub fn execute_exercises() {
    let input = parse_input(include_str!("../input/day25_in.txt"));

    println!("Constelations: {}", exercise_1(input)); 
}

fn parse_input(input: &str) -> Vec<Star> {
    input.lines().map(|l| {
        let mut iter = l.split(',');        
        let w = iter.next().unwrap().parse().unwrap();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        let z = iter.next().unwrap().parse().unwrap();

        Star {
            location: (w, x, y, z),
            origin_star: None
        }
    }).collect()
}

fn exercise_1(mut input: Vec<Star>) -> usize {

    input.sort_unstable_by_key(|Star { location: (a, b, c, d), origin_star: _}| {        
        a + b + c + d
    });

    for i in 0..input.len()-1 {
        let (a, b, c, d) = input[i].location;
        let mapped_4d = a + b + c + d;       

        let mut i_ancestor = find_ancestor(&mut input, i);

        for j in (i+1)..input.len() {
            let (a, b, c, d) = input[j].location;
            if a + b + c + d - mapped_4d > 3 { //Early out
                break;
            }
            if distance(&input[i].location, &input[j].location) <= 3 {
                
                let j_ancestor = find_ancestor(&mut input, j);                
                if i_ancestor == j_ancestor {
                    continue;
                } else {
                    input[i_ancestor].origin_star = Some(j_ancestor);
                    i_ancestor = j_ancestor;
                }
            }
        }        
    }

    input.iter().filter(|s| match s.origin_star {
        Some(_) => false,
        None => true
    }).count()
}

fn distance((w1, x1, y1, z1): &Location, (w2, x2, y2, z2): &Location) -> isize {
    (w1-w2).abs() + (x1-x2).abs() + (y1-y2).abs() + (z1-z2).abs()
}

fn find_ancestor(input: &mut Vec<Star>, mut origin: usize) -> usize {
    let mut start = origin;    
    while let Some(new_origin) = input[origin].origin_star {        
        origin = new_origin;        
    }
    
    // Now flatten the tree such that every child has a direct link to its ancestor
    while let Some(new_origin) = input[start].origin_star {
        input[start].origin_star = Some(origin);
        start = new_origin;
    }

    origin
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day25_ex1_s1() {
        let input = r"0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0";
        let input = parse_input(input);
        let result = exercise_1(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn day25_ex1_s2() {
        let input = r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        let input = parse_input(input);
        let result = exercise_1(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn day25_ex1_s3() {
        let input = r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        let input = parse_input(input);
        let result = exercise_1(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn day25_ex1_s4() {
        let input = r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        let input = parse_input(input);
        let result = exercise_1(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn day25_ex1_s5() {
        let input = r"9,0,0,0
6,0,0,0
0,0,0,0
3,0,0,0
21,0,0,0
18,0,0,0
15,0,0,0
12,0,0,0
30,0,0,0
27,0,0,0
24,0,0,0";
        let input = parse_input(input);
        let result = exercise_1(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn day25_ex1_s6() {
        let input = parse_input(include_str!("../input/day25_in.txt"));
        assert_eq!(exercise_1(input), 352);
    }
    
    #[bench]
    fn day25_bench_ex1(b: &mut Bencher) {
        let input = parse_input(include_str!("../input/day25_in.txt"));
        b.iter(move || exercise_1(input.clone())); 
    }

}