use std::collections::HashMap;
use bitvec::prelude::*;

fn next_size(n: usize, a: usize, b: usize) -> usize {
    (n/a) * b
}

type Grid = Vec<BitVec>;

fn hash(grid: Vec<&BitSlice>) -> u16 {
    assert!(grid.len() <= 3 && grid.len() >= 2, "{}", grid.len());
    let mut hash = 0u16;
    for i in 0..grid.len() { // lines
        for j in 0..grid.len() { //pos
            hash |= (grid[i][j] as u16) << (j + i*grid.len());
        }
    }

    hash |= 1 << (grid.len() + 12);

    //println!("{:b}", hash);
    hash
}

fn rotate_270(grid: &Grid) -> Grid {
    rotate_180(&rotate_90(grid))
}

fn rotate_180(grid: &Grid) -> Grid {
    rotate_90(&rotate_90(grid))
}
fn rotate_90(grid: &Grid) -> Grid {
    let mut g = grid.clone();
    let ul = grid[0][0];

    g[0].set(0, grid[grid.len() - 1][0]);
    g[grid.len() - 1].set(0, grid[grid.len() - 1][grid.len() - 1]);
    g[grid.len() - 1].set(grid.len() - 1, grid[0][grid.len() - 1]);
    g[0].set(grid.len() - 1, ul);

    if grid.len() == 3 {
        let ul = g[0][1];
        g[0].set(1, grid[1][0]);
        g[1].set(0, grid[2][1]);
        g[2].set(1, grid[1][2]);
        g[1].set(2, ul);
    }

    g
}
fn flip_v(grid: &Grid) -> Grid {
    let mut g = grid.clone();
    g.reverse();
    g
}

fn flip_h(grid: &Grid) -> Grid {
    grid.clone().into_iter()
        .map(|mut x| {
            x.reverse();
            x
        })
        .collect::<Grid>()
}

fn convert_2(rule2: &HashMap<u16, Grid>, grid: Grid) -> Grid {
    
    let (step_size, ns, input) = if grid.len() % 2 == 0 {
        (2, next_size(grid.len(), 2, 3), rule2)
    } else {
        (3, next_size(grid.len(), 3, 4), rule2)
    };
    let mut ng = vec![BitVec::with_capacity(ns); ns];

    //print_grid(&grid);  

    println!("ss:{}, ns: {}, s: {}", step_size, ns, grid.len());
    // Take the first step_size lines
    for block_y in 0..(grid.len()/step_size) {
        let y = block_y * step_size;
        for block_x in 0..(grid.len()/step_size) {
            let x = block_x * step_size;
            let hash = hash((0..step_size).map(|f| &grid[y + f][x..(x+step_size)]).collect::<Vec<_>>());
            let mini_grid = &input[&hash];            
            for k in 0..(step_size + 1) { // foreach line in ns
                ng[block_y * (step_size + 1) + k].append(&mut (&mini_grid[k]).clone());
            }
        }
    }
    ng
}

fn read_grid(line: &str) -> Grid {
    line.split('/').map(|x| {
        x.chars().map(|c| match c {
            '.' => false,
            '#' => true,
            c => panic!("Invalid char {}", c)
        }).collect::<BitVec>()
    }).collect::<Grid>()
        
}

fn print_grid(grid: &Grid) {
    for line in grid {
        println!("{}", line);
    }
    println!();
}

fn read_input(line: &str) -> HashMap<u16, Grid> {
    let mut hashes = HashMap::new();

    for line in line.lines() {
        read_rule(line, &mut hashes);
    }

    hashes
}
fn read_rule(line: &str, rule: &mut HashMap<u16, Grid>) {
    // ../.# => ##./#../...
    let mut iterator = line.split(" => ");

    let mini_grid_left = read_grid(iterator.next().unwrap());
    let mini_grid_right = read_grid(iterator.next().unwrap());

//    let hash = hash(mini_grid_left.iter().map(BitVec::as_bitslice).collect());

    let a = rotate_90(&mini_grid_left);
    let b = rotate_180(&mini_grid_left);
    let c = rotate_270(&mini_grid_left);
    let d = flip_h(&mini_grid_left);
    let e = flip_v(&mini_grid_left);
    
    let f = flip_h(&a);
    let g = flip_v(&a);
    
    let h = flip_h(&b);
    let i = flip_v(&b);

    let j = flip_h(&c);
    let k = flip_v(&c);
    

    //println!("Start block");
    // print_grid(&mini_grid_right);
    //print_grid(&mini_grid_left);    
    // print_grid(&a);
    // print_grid(&b);
    // print_grid(&c);
    // print_grid(&d);    
    // print_grid(&e);

    rule.insert(hash(mini_grid_left.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(a.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(b.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(c.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(d.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(e.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(f.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(g.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(h.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(i.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(j.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
    rule.insert(hash(k.iter().map(BitVec::as_bitslice).collect()), mini_grid_right.clone());
} 

#[test]
fn test_examples() {
    let mut board: Vec<BitVec> = vec![bitvec![0, 1, 0], bitvec![0, 0, 1], bitvec![1,1,1]];
    let rules = read_input(r"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#");
    for i in 0..2 {
        board = convert_2(&rules, board);
        print_grid(&board);
    }


    assert_eq!(board.iter().map(|x| x.count_ones()).sum::<usize>(), 12);
}

#[test]
fn run21() {
    let mut board: Vec<BitVec> = vec![bitvec![0, 1, 0], bitvec![0, 0, 1], bitvec![1,1,1]];
    let rules = read_input(include_str!("input/day21.txt"));

    for i in 0..5 {
        board = convert_2(&rules, board);
        //print_grid(&board);
    }

    println!("ones: {}", board.iter().map(|x| x.count_ones()).sum::<usize>());
    for i in 0..13 {
        board = convert_2(&rules, board);
        //print_grid(&board);
    }
    println!("ones: {}", board.iter().map(|x| x.count_ones()).sum::<usize>());
}