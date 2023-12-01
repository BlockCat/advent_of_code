use std::{collections::VecDeque, print};

use hashbrown::{HashMap, HashSet};
use utils::Grid;

// #[test]
pub fn run() {
    let input = read_input(include_str!("input/day20.txt"));
    // println!("{}", exercise_1(&input));
    println!("{}", exercise_2(&input));
}

type Input = Vec<(usize, Grid<bool>)>;
fn read_input(input: &str) -> Input {
    let mut it = input.lines();

    let mut grids = Vec::new();

    while let Some(line) = it.by_ref().next() {
        let id: usize = line[5..9].parse().unwrap();
        let grid = Grid::from_vec(
            it.by_ref()
                .take_while(|x| !x.is_empty())
                .map(|x| {
                    x.chars()
                        .map(|a| match a {
                            '#' => true,
                            '.' => false,
                            _ => unimplemented!(),
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        );

        grids.push((id, grid))
    }

    grids
}

fn insertion<'a>(
    hashes: (usize, usize),
    grid: (usize, &'a Grid<bool>),
    map: &mut HashMap<usize, Vec<(usize, &'a Grid<bool>)>>,
) {
    map.entry(hashes.0).or_insert_with(|| Vec::new()).push(grid);
    map.entry(hashes.1).or_insert_with(|| Vec::new()).push(grid);
}

fn exercise_1(input: &Input) -> usize {
    let mut map: HashMap<usize, Vec<(usize, &Grid<bool>)>> = HashMap::new();
    let mut tiles = HashMap::new();
    for (tile_id, grid) in input {
        let grid_up = horizontal_hash(grid, 0);
        let grid_down = horizontal_hash(grid, grid.height - 1);
        let grid_left = vertical_hash(grid, 0);
        let grid_right = vertical_hash(grid, grid.width - 1);

        insertion(grid_up, (*tile_id, grid), &mut map);
        insertion(grid_down, (*tile_id, grid), &mut map);
        insertion(grid_left, (*tile_id, grid), &mut map);
        insertion(grid_right, (*tile_id, grid), &mut map);

        tiles.insert(*tile_id, grid);
    }

    let mut neighbours = HashMap::new();

    for (_, ids) in map {
        for (id, _) in &ids {
            for (id2, _) in &ids {
                if id != id2 {
                    neighbours
                        .entry(*id)
                        .or_insert_with(|| HashSet::new())
                        .insert(*id2);
                }
            }
        }
    }

    neighbours
        .iter()
        .filter(|x| x.1.len() == 2)
        .map(|x| x.0)
        .product()
}

fn exercise_2(input: &Input) -> usize {
    let mut map: HashMap<usize, Vec<(usize, &Grid<bool>)>> = HashMap::new();
    let mut tiles = HashMap::new();
    for (tile_id, grid) in input {
        let grid_up = horizontal_hash(grid, 0);
        let grid_down = horizontal_hash(grid, grid.height - 1);
        let grid_left = vertical_hash(grid, 0);
        let grid_right = vertical_hash(grid, grid.width - 1);

        insertion(grid_up, (*tile_id, grid), &mut map);
        insertion(grid_down, (*tile_id, grid), &mut map);
        insertion(grid_left, (*tile_id, grid), &mut map);
        insertion(grid_right, (*tile_id, grid), &mut map);

        tiles.insert(*tile_id, grid.clone());
    }

    let mut neighbours = HashMap::new();

    for (_, ids) in map {
        for (id, _) in &ids {
            for (id2, _) in &ids {
                if id != id2 {
                    neighbours
                        .entry(*id)
                        .or_insert_with(|| HashSet::new())
                        .insert(*id2);
                }
            }
        }
    }

    for x in &neighbours {
        assert!(x.1.len() <= 4, "{:?}", x);
    }

    let mut corners = neighbours
        .iter()
        .filter(|x| x.1.len() == 2)
        .map(|x| *x.0)
        .collect::<Vec<_>>();
    corners.sort();
    let first_id = corners[0];
    let first = tiles[&first_id].clone(); //tiles.iter().map(|x| (*x.0, x.1.clone())).next().unwrap();

    let mut map = HashMap::new();
    map.insert((0isize, 0isize), (first_id, first));

    resolve((first_id, (0, 0)), &mut map, &tiles, &neighbours);

    assert_eq!(tiles.len(), map.len());
    let mut grid = collect(map);

    let seamonster: Grid<Option<()>> = Grid::from_vec(
        "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   "
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Some(()),
                        _ => None,
                    })
                    .collect()
            })
            .collect(),
    );

    for _ in 0..4 {
        grid = rotate_left(grid);
        let a = check_seamonsters(&grid, &seamonster);
        if a.0 {
            return grid.grid.iter().filter(|x| **x).count() - a.1;
        }
    }

    grid = horizontal_flip(grid);

    for _ in 0..4 {
        grid = rotate_left(grid);
        let a = (check_seamonsters(&grid, &seamonster));
        if a.0 {
            return grid.grid.iter().filter(|x| **x).count() - a.1;
        }
    }

    unreachable!()
}

fn check_seamonsters(grid: &Grid<bool>, seamonster: &Grid<Option<()>>) -> (bool, usize) {
    let mut set = HashSet::new();
    let mut found = false;
    for x in 0..(grid.width - seamonster.width) {
        'rip: for y in 0..(grid.height - seamonster.height) {
            let mut pos = Vec::new();
            for sx in 0..seamonster.width {
                for sy in 0..seamonster.height {
                    if seamonster.get(sx, sy).unwrap().is_some() {
                        if grid.get(x + sx, y + sy) == Some(&true) {
                            pos.push((x + sx, y + sy));
                        } else {
                            // continue 'rip;
                        }
                    }
                }
            }
            if (pos.len()) == 15 {
                found = true;
                for pos in pos {
                    set.insert(pos);
                }
            }
        }
    }

    (found, set.len())
}

fn collect(map: HashMap<(isize, isize), (usize, Grid<bool>)>) -> Grid<bool> {
    let left = map.keys().min().unwrap().0;
    let right = map.keys().max().unwrap().0;
    let up = map.keys().map(|x| x.1).min().unwrap();
    let down = map.keys().map(|x| x.1).max().unwrap();

    let width = (right - left) as usize;
    let height = (down - up) as usize;
    let gw = map.get(&(0, 0)).unwrap().1.width - 2;
    let gh = map.get(&(0, 0)).unwrap().1.width - 2;
    let width = gw * (width + 1);
    let height = gh * (height + 1);

    let mut grid = Grid::new(width, height);

    for gx in left..=right {
        let ngx = (gx - left) as usize;

        for gy in up..=down {
            let ngy = (gy - up) as usize;

            for x in 0..gw {
                for y in 0..gh {
                    let nx = ngx * gw + x;
                    let ny = ngy * gh + y;

                    grid.set(nx, ny, *map[&(gx, gy)].1.get(x + 1, y + 1).unwrap())
                }
            }
        }
    }
    grid
}

fn print(grid: &Grid<bool>) {
    for y in 0..grid.height {
        let r = grid.grid[(y * grid.width)..((y + 1) * grid.width)]
            .iter()
            .map(|x| if *x { '#' } else { '_' })
            .collect::<String>();
        println!("{}", r);
    }
}

fn resolve(
    start: (usize, (isize, isize)),
    map: &mut HashMap<(isize, isize), (usize, Grid<bool>)>,
    tiles: &HashMap<usize, Grid<bool>>,
    neighbours: &HashMap<usize, HashSet<usize>>,
) {
    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    stack.push_front(start);

    while let Some((current_id, current_pos)) = stack.pop_front() {
        if !visited.insert(current_id) {
            continue;
        }

        let current_grid = &map.get(&current_pos).unwrap().1;
        let grid_up = horizontal_hash(current_grid, 0).0;
        let grid_down = horizontal_hash(current_grid, current_grid.height - 1).0;
        let grid_left = vertical_hash(current_grid, 0).0;
        let grid_right = vertical_hash(current_grid, current_grid.width - 1).0;

        let up: (isize, isize) = (current_pos.0, current_pos.1 - 1);
        let down: (isize, isize) = (current_pos.0, current_pos.1 + 1);
        let left: (isize, isize) = (current_pos.0 - 1, current_pos.1);
        let right: (isize, isize) = (current_pos.0 + 1, current_pos.1);

        for neighbour in neighbours.get(&current_id).unwrap() {
            if visited.contains(neighbour) {
                continue;
            }
            let neighbour_grid = &tiles[neighbour];
            if !map.contains_key(&up) {
                if let Some(grid) =
                    inspect_horizontal(grid_up, neighbour_grid.clone(), neighbour_grid.height - 1)
                {
                    map.insert(up, (*neighbour, grid));
                    stack.push_front((*neighbour, up));
                }
            }
            if !map.contains_key(&down) {
                if let Some(grid) = inspect_horizontal(grid_down, neighbour_grid.clone(), 0) {
                    map.insert(down, (*neighbour, grid));
                    stack.push_front((*neighbour, down));
                }
            }
            if !map.contains_key(&left) {
                if let Some(grid) =
                    inspect_vertical(grid_left, neighbour_grid.clone(), neighbour_grid.width - 1)
                {
                    map.insert(left, (*neighbour, grid));
                    stack.push_back((*neighbour, left));
                }
            }
            if !map.contains_key(&right) {
                if let Some(grid) = inspect_vertical(grid_right, neighbour_grid.clone(), 0) {
                    map.insert(right, (*neighbour, grid));
                    stack.push_back((*neighbour, right));
                }
            }
        }
    }
}

fn inspect_horizontal(hash: usize, mut neighbour_grid: Grid<bool>, y: usize) -> Option<Grid<bool>> {
    for _ in 0..4 {
        neighbour_grid = rotate_left(neighbour_grid);
        let down_hash = horizontal_hash(&neighbour_grid, y);
        if hash == down_hash.0 {
            return Some(neighbour_grid);
        } else if hash == down_hash.1 {
            return Some(horizontal_flip(neighbour_grid));
        }
    }
    None
}

fn inspect_vertical(hash: usize, mut neighbour_grid: Grid<bool>, x: usize) -> Option<Grid<bool>> {
    for _ in 0..4 {
        neighbour_grid = rotate_left(neighbour_grid);
        let down_hash = vertical_hash(&neighbour_grid, x);
        if hash == down_hash.0 {
            return Some(neighbour_grid);
        } else if hash == down_hash.1 {
            return Some(vertical_flip(neighbour_grid));
        }
    }
    None
}

fn horizontal_flip(grid: Grid<bool>) -> Grid<bool> {
    let mut flipped = Grid::new(grid.width, grid.height);
    for x in 0..grid.width {
        for y in 0..grid.height {
            flipped.set(grid.width - x - 1, y, *grid.get(x, y).unwrap());
        }
    }
    flipped
}

fn vertical_flip(grid: Grid<bool>) -> Grid<bool> {
    let mut flipped = Grid::new(grid.width, grid.height);
    for x in 0..grid.width {
        for y in 0..grid.height {
            flipped.set(x, grid.width - y - 1, *grid.get(x, y).unwrap());
        }
    }
    flipped
}

fn rotate_left(grid: Grid<bool>) -> Grid<bool> {
    let mut flipped = Grid::new(grid.height, grid.width);
    for x in 0..grid.width {
        for y in 0..grid.height {
            flipped.set(y, grid.height - x - 1, *grid.get(x, y).unwrap());
        }
    }
    flipped
}

fn vertical_hash(grid: &Grid<bool>, x: usize) -> (usize, usize) {
    (0..grid.height)
        .map(|y| grid.get(x, y).unwrap())
        .enumerate()
        .fold((0usize, 0usize), |(hash, hash_flip), (index, x)| {
            if *x {
                (
                    hash | (1 << index),
                    hash_flip | (1 << (grid.height - index - 1)),
                )
            } else {
                (hash, hash_flip)
            }
        })
}

fn horizontal_hash(grid: &Grid<bool>, y: usize) -> (usize, usize) {
    (0..grid.width)
        .map(|x| grid.get(x, y).unwrap())
        .enumerate()
        .fold((0usize, 0usize), |(hash, hash_flip), (index, x)| {
            if *x {
                (
                    hash | (1 << index),
                    hash_flip | (1 << (grid.width - index - 1)),
                )
            } else {
                (hash, hash_flip)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d18ex1() {
        let input = read_input(include_str!("input/day20test.txt"));
        assert_eq!(2, exercise_1(&input));
        // assert_eq!(71, exercise_1(&input))
    }

    #[test]
    fn d18ex2() {
        let input = read_input(include_str!("input/day20test.txt"));
        assert_eq!(12, exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex1(b: &mut Bencher) {
        let input = read_input(include_str!("input/day20.txt"));
        b.iter(|| exercise_1(&input));
    }

    #[bench]
    fn d18_bench_ex2(b: &mut Bencher) {
        let input = read_input(include_str!("input/day20.txt"));
        b.iter(|| exercise_2(&input));
    }
}
