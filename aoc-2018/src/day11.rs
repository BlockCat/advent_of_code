use rayon::prelude::*;


pub fn execute_exercises() {
    println!("Max 3x3 grid: {:?}",exercise_1(9995));
    println!("max: {:?}", exercise_2(9995));
}

fn power_level(input: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 11;
    ((((rack_id * (y+1) + input) * rack_id) / 100) % 10) - 5
}

fn build_prefix_sum_fuel_cells(input: i32) -> [[i32; 300]; 300] {
    let mut fuel_cells = [[0i32; 300]; 300];    

    fuel_cells[0][0] = power_level(input, 0, 0);    
    for i in 1..300 {        
        fuel_cells[i][0] = fuel_cells[i-1][0] + power_level(input, i as i32, 0);
        fuel_cells[0][i] = fuel_cells[0][i-1] + power_level(input, 0, i as i32);
    }
    for y in 1..300 {        
        for x in 1..300 {            
            fuel_cells[x][y] = fuel_cells[x-1][y] + fuel_cells[x][y-1] - fuel_cells[x-1][y-1] + power_level(input, x as i32, y as i32);
        }
    }

    fuel_cells    
}

fn exercise_1(input: i32) -> (i32, (usize, usize)) {
    let mut fuel_cells = [[0i32; 300]; 300];    

    // first phase    
    for x in 0..300 {
        for y in 0..300 {
            let power_level = power_level(input, x, y);
            fuel_cells[x as usize][y as usize] = power_level;
        }
    }

    let mut max = (-1000, (0, 0));

    for x in 0..298 {
        for y in 0..298 {
            let conv = 
                fuel_cells[x][y] + fuel_cells[x + 1][y] + fuel_cells[x + 2][y] +
                fuel_cells[x][y + 1] + fuel_cells[x + 1][y + 1] + fuel_cells[x + 2][y + 1] +
                fuel_cells[x][y + 2] + fuel_cells[x + 1][y + 2] + fuel_cells[x + 2][y + 2];

            max = std::cmp::max(max, (conv, (x + 1, y + 1)));
        }
    }

    max
}


fn exercise_2(input: i32) -> (i32, (usize, usize), usize) {

    let fuel_cells = build_prefix_sum_fuel_cells(input);
    
    let mut max = (-1000, (0, 0), 0);

    for i in 0..300 {
        max = std::cmp::max(max, (fuel_cells[i][i], (1, 1), i + 1));
    }
    
    for xy in 1..300 {
        for i in 0..(300 - xy) {            
            let x_surface = fuel_cells[xy + i][i] - fuel_cells[xy-1][i];
            let y_surface = fuel_cells[i][xy + i] - fuel_cells[i][xy - 1];
            max = std::cmp::max(max, (x_surface, (xy+1, 1), i+1));            
            max = std::cmp::max(max, (y_surface, (1, xy+1), i+1));            
        }
    }

    for y in 1..300 {
        for x in 1..300 {
            for i in 0..(300 - std::cmp::max(x, y)) {                
                let surface = fuel_cells[x - 1][y - 1] + fuel_cells[x + i][y + i] - fuel_cells[x + i][y-1]-fuel_cells[x-1][y + i];
                max = std::cmp::max(max, (surface, (x+1, y+1), i+1));
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn day11_ex1_s1() {
        let (_, (x, y)) = exercise_1(42);
        assert_eq!((x, y), (21, 61));
    }

    #[test]
    fn day11_ex1_s2() {
        let (_, (x, y)) = exercise_1(18);
        assert_eq!((x, y), (33, 45));
    }

    #[test]
    fn day11_ex1_s3() {
        assert_eq!(exercise_1(9995), (29, (33, 45)));
    }

    #[test]
    fn day11_ex2_s1() {
        let (_, (x, y), size) = exercise_2(18);
        assert_eq!((x, y, size), (90, 269, 16));
    }

    #[test]
    fn day11_ex2_s2() {
        let (_, (x, y), size) = exercise_2(42);
        assert_eq!((x, y, size), (232, 251, 12));
    }

    #[test]
    fn day11_ex2_s3() {
        let (_, (x, y), size) = exercise_2(9995);
        assert_eq!((x, y, size), (233, 116, 15));
    }

    #[bench]
    fn day11_bench_ex1(b: &mut Bencher) {
        b.iter(|| exercise_1(9995));
    }

    #[bench]
    fn day11_bench_ex2(b: &mut Bencher) {
        b.iter(|| exercise_2(9995));
    }
}