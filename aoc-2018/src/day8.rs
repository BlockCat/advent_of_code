

pub fn execute_exercises() {
    println!("Order: {}", exercise_1(read_input()));
    println!("Root value: {}", exercise_2(read_input()));
}

fn read_input() -> impl Iterator<Item = u8> {
    include_str!("../input/day8_in.txt").split(' ').map(|c| c.parse::<u8>().unwrap())
}

struct Node {
    parent: u8,
    children_count: u8
}

fn exercise_1(mut input: impl Iterator<Item = u8>) -> u32 {
    let mut childrens = [0u8; 2000];//Vec::with_capacity(90);
    let mut metadatas = [0u8; 2000];//Vec::with_capacity(90);
    let mut parents = [0usize; 2000];

    let mut current_node;
    let mut ids = 0;    
    let mut sum = 0u32;
    while let Some(n) = input.next() {
        
        ids+=1;
        current_node = ids;
        childrens[current_node] = n;
        metadatas[current_node] = input.next().unwrap();        
        
        if childrens[current_node] == 0 && metadatas[current_node] > 0 {
            // No children and there is metadata
            sum += (0..metadatas[current_node])
            .map(|_| u32::from(input.next().unwrap()))
            .sum::<u32>();            

            // go up again
            current_node = parents[current_node];                    
            while current_node > 0 {
                if childrens[current_node] > 0 {
                    // There are children, therefore we handle these children
                    parents[ids + 1] = current_node;
                    childrens[current_node] -= 1;
                    break;
                } else if metadatas[current_node] > 0 {
                    // There are no children left and there is metadata
                    sum += (0..metadatas[current_node]).map(|_|u32::from(input.next().unwrap())).sum::<u32>();                   
                    current_node = parents[current_node];
                } else {
                    // There are no children and no metadata
                    current_node = parents[current_node];
                }
            }
        } else {
            // There are children
            // The value becomes the sum of the values referenced by metadata
            parents[ids + 1] = current_node;
            childrens[current_node] -= 1;
        }
    }

    sum
}

fn exercise_2(mut input: impl Iterator<Item = u8>) -> u32 {
    let mut childrens = [0u8; 1756];//Vec::with_capacity(90);
    let mut metadatas = [0u8; 1756];//Vec::with_capacity(90);
    let mut parents = [0usize; 1756];
    let mut values = [0u32; 1756];
    let mut children = Vec::with_capacity(1756);

    children.push(Vec::new());

    let mut current_node;
    let mut ids = 0;        
    while let Some(n) = input.next() {
        
        ids+=1;
        current_node = ids;
        childrens[current_node] = n;
        children.push(Vec::with_capacity(n as usize + 1));
        metadatas[current_node] = input.next().unwrap();
        
        if childrens[current_node] == 0 && metadatas[current_node] > 0 {
            // No children and there is metadata                    
            values[current_node] = (0..metadatas[current_node]).map(|_| u32::from(input.next().unwrap())).sum();            
            
            current_node = parents[current_node];

            while current_node > 0 {
                if childrens[current_node] > 0 {
                    // There are children, therefore we handle these children
                    parents[ids + 1] = current_node;
                    children[current_node].push(ids + 1);
                    childrens[current_node] -= 1;
                    break;
                } else {
                    if metadatas[current_node] > 0 {
                        // There are no children left and there is metadata
                        values[current_node] = (0..metadatas[current_node])
                            .map(|_| input.next().unwrap() as usize)
                            .fold(0u32, |acc, r| {
                                let increase = match children[current_node].get(&r - 1) {
                                    Some(id) => values[*id],
                                    None => 0
                                };
                                acc + increase
                            });
                    }                
                    current_node = parents[current_node];
                }
            }
        } else {
            // There are children
            // The value becomes the sum of the values referenced by metadata
            parents[ids + 1] = current_node;
            children[current_node].push(ids + 1);
            childrens[current_node] -= 1;
        }
    }
    values[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d8_ex1_s1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".split(' ').map(|c| c.parse::<u8>().unwrap());
        assert_eq!(exercise_1(input), 138);
    }

    #[test]
    fn quick() {
        let mut s = vec![1, 2, 3, 4, 5, 6];
        let (a_sl, b_sl) = s.split_at_mut(3);

        for s in a_sl {
            *s -= 1;
        }
        for s in b_sl {
            *s += 1;
        }
    }

    #[test]
    fn d8_ex1_s2() {
        let input = "2 3 0 3 10 11 12 1 0 0 1 99 1 1 2".split(' ').map(|c| c.parse::<u8>().unwrap());
        assert_eq!(exercise_1(input), 136);
    }

    #[test]
    fn d8_ex1_s3() {        
        assert_eq!(exercise_1(read_input()), 41760);
    }

    #[test]
    fn d8_ex2_s1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".split(' ').map(|c| c.parse::<u8>().unwrap());
        assert_eq!(exercise_2(input), 66);
    }

    #[test]
    fn d8_ex2_s2() {
        let input = "2 3 0 3 10 11 12 1 0 0 1 99 1 1 2".split(' ').map(|c| c.parse::<u8>().unwrap());
        assert_eq!(exercise_2(input), 66);       
    }

    #[test]
    fn d8_ex2_s3() {
        assert_eq!(exercise_2(read_input()), 25737);
    }

    #[bench]
    fn d8_bench_ex1(b: &mut Bencher) {        
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d8_bench_ex2(b: &mut Bencher) {        
        b.iter(|| exercise_2(read_input()));
    }

}