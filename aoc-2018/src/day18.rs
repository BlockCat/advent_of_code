use ocl::{ ProQue, Buffer, Kernel};
use hashbrown::HashMap;


pub fn execute_exercises() {
    let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));
    println!("10 minutes: {}", exercise_1(map.clone(), (w, h)));
    println!("may more minutes: {}", exercise_2_cpu(map.clone(), (w, h)));
}

const PROGRAM: &str = r#"
    __kernel void exercise1(__global read_only char* buffer, __global char* output, int width) {
        int i = get_global_id(0) + 1;
        int j = get_global_id(1) + 1;
        int id = i + j * width;
        
        int c_trees = 0;
        int c_lumbers = 0;        

        for (int x = i - 1; x<=i+1; x++) {
            for (int y = j - 1; y<=j+1; y++) {
                if (x == i && y == j) continue;
                int o_id = x + y * width;
                c_trees += (int)(buffer[o_id] == '|');
                c_lumbers += (int)(buffer[o_id] == '#');
            }
        }

        //printf("trees=%d, lumbers=%d (%d, %d) \n", c_trees, c_lumbers, i, j);

        if (buffer[id] == '#' && (c_trees == 0 || c_lumbers == 0)) {            
            output[id] = '.';
        } else if (buffer[id] == '|' && c_lumbers >= 3) {            
            output[id] = '#';            
        } else if (buffer[id] == '.' && c_trees >= 3) {
            output[id] = '|';
        } else {
            output[id] = buffer[id];
        }        
    }
"#;


fn parse_input(input: &str) -> (Vec<u8>, usize, usize) {
    let width = input.lines().next().unwrap().len() + 2;
    let height = input.lines().count() + 2;
    let mut ccc = vec!(0u8; width);
    ccc.extend(input.lines().flat_map(|s| {
        vec!(0u8).into_iter().chain(s.bytes()).chain(vec!(0u8).into_iter())
    }));
    ccc.extend(vec!(0u8; width));
    

    (ccc, width, height)
}

fn build_program(map: Vec<u8>, (w, h): (usize, usize)) -> (Buffer<u8>, Buffer<u8>, Kernel) {
    let pro_que = ProQue::builder()
        .src(PROGRAM)
        .dims((w -2, h-2))
        .build().unwrap();

    let buffer = Buffer::builder()
        .queue(pro_que.queue().clone())
        .copy_host_slice(&map)        
        .len((w, h))
        .build().unwrap();


    let output = Buffer::builder()
        .queue(pro_que.queue().clone())
        .len((w, h))
        .build().unwrap();

    

    let kernel = pro_que.kernel_builder("exercise1")
        .arg(&buffer)        
        .arg(&output)
        .arg(w as i32)        
        .build().unwrap();
    (buffer, output, kernel)
}


fn do_cycle(map: Vec<u8>, (w, h): (usize, usize)) -> Vec<u8> {
    let mut output = vec!(0u8; map.len());
     for x in 1..w-1 {
        for y in 1..h-1 {
            let id = x + y * w;
            let mut c_trees = 0;
            let mut c_lumbers = 0;

            for y_1 in y-1..=y+1 {
                for x_1 in x-1..=x+1 {
                    if x_1 == x && y_1 == y {
                        continue;
                    }
                    let o_id = x_1 + y_1 * w;
                    c_trees += (map[o_id] == b'|') as usize;
                    c_lumbers += (map[o_id] == b'#') as usize;
                }
            }

            if map[id] == b'#' && (c_trees == 0 || c_lumbers == 0) {
                output[id] = b'.';
            } else if map[id] == b'|' && c_lumbers >= 3 {
                output[id] = b'#';            
            } else if map[id] == b'.' && c_trees >= 3 {
                output[id] = b'|';
            } else {
                output[id] = map[id];
            }
        }
    }

    output
}

fn exercise_1_cpu(mut map: Vec<u8>, (w, h): (usize, usize)) -> usize {
    let mut output = map.clone();
    for i in 0..10 {        
        map = do_cycle(map, (w, h));
    }
    
    let trees = map.iter().filter(|&s| *s == b'|').count();
    let lumbers = map.iter().filter(|&s| *s == b'#').count();

    trees * lumbers
}


fn exercise_2_cpu(mut map: Vec<u8>, (w, h): (usize, usize)) -> usize {
    
    let mut ccc: HashMap<Vec<u8>, usize> = HashMap::new();
    let cycles = 1000000000;

    for i in 0..cycles {               
        map = do_cycle(map, (w, h));

        //pretty_print(&map, (w, h));        
        //use std::{thread, time};
        //thread::sleep(time::Duration::from_millis(10));
        if let Some(old) = ccc.insert(map.clone(), i) {
            println!("cycle found: {} -> {}", old, i);
        
            let remaining = cycles - i - 1;
            let cycle_size = i - old;
            let cycles = remaining % cycle_size;
            for _ in 0..cycles {
                map = do_cycle(map, (w, h));                
            }
            break;
        }
    }
    
    let trees = map.iter().filter(|&s| *s == b'|').count();
    let lumbers = map.iter().filter(|&s| *s == b'#').count();

    trees * lumbers
}

fn exercise_1(map: Vec<u8>, dim: (usize, usize)) -> usize {
    let (buffer, output, kernel) = build_program(map, dim);

    for i in 0..10 {
        unsafe {kernel.enq().unwrap();}
        output.copy(&buffer, None, None).enq().unwrap();
    }

    let mut m: Vec<u8> = vec!(0; buffer.len());
    output.read(&mut m).enq().unwrap();
    
    let trees = m.iter().filter(|&s| *s == b'|').count();
    let lumbers = m.iter().filter(|&s| *s == b'#').count();

    trees * lumbers
}
fn exercise_2(map: Vec<u8>, dim: (usize, usize)) -> usize {
    let (buffer, output, kernel) = build_program(map, dim);
    let mut ccc: HashMap<Vec<u8>, usize> = HashMap::new();

    let cycles = 1000000000;

    for i in 0..cycles  {
        unsafe {kernel.enq().unwrap();}    
        output.copy(&buffer, None, None).enq().unwrap();
        let mut m: Vec<u8> = vec!(0; buffer.len());
        output.read(&mut m).enq().unwrap();
        
        if let Some(old) = ccc.insert(m, i) {
            //println!("cycle found: {} -> {}", old, i);
            let cycles = (cycles - i - 1) % (i - old);
            for _ in 0..cycles {
                unsafe {kernel.enq().unwrap();}    
                output.copy(&buffer, None, None).enq().unwrap();                
            }
            break;
        }
    }

    let mut m: Vec<u8> = vec!(0; buffer.len());
    output.read(&mut m).enq().unwrap();

    let trees = m.iter().filter(|&s| *s == b'|').count();
    let lumbers = m.iter().filter(|&s| *s == b'#').count();

    trees * lumbers
}

fn pretty_print(map: &Vec<u8>, (width, height): (usize, usize)) {
    let grid = (1..height-1).flat_map(|y| {
        let begin = y * width;
        (&map[begin..begin+width]).iter().map(|u| *u as char).chain(vec!('\n'))
    }).collect::<String>();

    println!("{}", grid);
    /*
    for y in 1..height-1 {
        for x in 1..width-1 {
            match map[x + y * width] {
                b'#' => print!("#"),
                b'|' => print!("|"),
                _ => print!("{}", map[x + y * width] as char)
            }
        }
        println!();
    }*/
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

   #[test]
   fn day18_ex1_s1() {
       let input = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let (map, w, h) = parse_input(input);
        println!("{:?}, {}, {}", map.len(), w, h);

        //exercise_1(map, (w, h));
        assert_eq!(exercise_1_cpu(map, (w, h)), 1147);
   }
    #[test]
    fn day18_ex1_s2() {
        let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));
        assert_eq!(exercise_1_cpu(map, (w, h)), 355918);
    }

    #[test]
    fn day18_ex2_s1() {
        let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));
        assert_eq!(exercise_2_cpu(map, (w, h)), 202806);
    }

    #[bench]
    fn day18_bench_ex1(b: &mut Bencher) {
        let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));        
        b.iter(move || exercise_1(map.clone(), (w, h)));
    }   

    #[bench]
    fn day18_bench_ex1_cpu(b: &mut Bencher) {
        let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));        
        b.iter(move || exercise_1_cpu(map.clone(), (w, h)));
    }   

    #[bench]
    fn day18_bench_ex2(b: &mut Bencher) {
        let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));        
        b.iter(move || exercise_2(map.clone(), (w, h)));
    }   

    #[bench]
    fn day18_bench_ex2_cpu(b: &mut Bencher) {
        let (map, w, h) = parse_input(include_str!("../input/day18_in.txt"));        
        b.iter(move || exercise_2_cpu(map.clone(), (w, h)));
    }   


}