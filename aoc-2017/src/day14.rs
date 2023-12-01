use crate::day10;

fn algorithm1(input: &str) -> (usize, [u128; 128]) {
    let mut counter = 0;
    let mut grid = [0u128; 128];
    for i in 0..128 {
        let hash = format!("{}-{}", input, i);
        let parse = day10::knot_hash_2(hash.bytes(), 256, 64);
        assert_eq!(parse.len(), 32);
        let parse = u128::from_str_radix(&parse, 16).unwrap();
        grid[i] = parse;
        for j in 0..128 {
            counter += (parse >> j) & 1;
            /*if (parse >> j) & 1 == 1 {
                print!("#");
            } else {
                print!(".");
            }*/
        }
        //println!();
    }
    (counter as usize, grid)
}

fn count_regions(input: &str) -> usize {    
    let mut grid = [[false; 128]; 128];

    for i in 0..128 {
        let hash = format!("{}-{}", input, i);
        let parse = day10::knot_hash_2(hash.bytes(), 256, 64);        
        let parse = u128::from_str_radix(&parse, 16).unwrap();        
        for j in 0..128 {
            if (parse >> (127-j)) & 1 == 1 {
                grid[i][j] = true;
            }
        }        
    }

    let mut groups = 0;
    let mut visited = [[false; 128]; 128];
    let mut queue = Vec::new();    

    for i in 0..128usize {
        for j in 0..128usize {
            if grid[i][j] && !visited[i][j] {
                groups += 1;
                queue.push((i, j));

                while let Some((x, y)) = queue.pop() {
                    if grid[x][y] && !visited[x][y] {
                        visited[x][y] = true;
                        if x > 0 { queue.push((x -1, y)); }
                        if x < 127 { queue.push((x + 1, y)); }
                        if y > 0 { queue.push((x, y - 1)); }
                        if y < 127 { queue.push((x, y + 1)); }
                    }
                }
            }
        }
    }
    
    groups
}


#[test]
fn test_example() {
    assert_eq!(algorithm1("flqrgnkx").0, 8108);
    assert_eq!(count_regions("flqrgnkx"), 1242);
}


#[test]
fn test_parse() {
    let parse = u128::from_str_radix("a0c2017", 16).unwrap();

    assert_eq!(format!("{:b}", parse), "1010000011000010000000010111");
}

#[test]
fn run14() {
    println!("used: {}, regions: {}", algorithm1("uugsqrei").0, count_regions("uugsqrei"));
}