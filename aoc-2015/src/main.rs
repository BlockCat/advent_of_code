macro_rules! iter {
    ($item:expr) => (std::iter::once($item));
    ($item:expr, $($rest:tt)*) => (std::iter::once($item).chain(iter!($($rest)*)));
}
use std::collections::*;
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Unit {
    Elf,
    Gnome,
}

fn main() {
    println!("{:?}", part2(r"################################
#######################.########
######################....######
#######################.....####
##################..##......####
###################.##.....#####
###################.....G..#####
##################.....G...#####
############.....GG.G...#..#####
##############...##....##.######
############...#..G............#
###########......E.............#
###########...#####..E........##
#...#######..#######.......#####
#..#..G....G#########.........##
#..#....G...#########..#....####
##.....G....#########.E......###
#####G.....G#########..E.....###
#####.......#########....#.....#
#####G#G....G#######.......#..E#
###.....G.....#####....#.#######
###......G.....G.G.......#######
###..................#..########
#####...................########
#####..............#...#########
####......G........#.E.#E..#####
####.###.........E...#E...######
####..##........#...##.....#####
########.#......######.....#####
########...E....#######....#####
#########...##..########...#####
################################"));
}

pub fn part2(input: &str) -> usize {
    let mut map = vec![];
    let mut width = 0;
    let mut units = vec![];
    let mut elf_attack_power = 25;
    for (y, line) in input.lines().enumerate() {
        width = line.len();
        for (x, &ch) in line.as_bytes().iter().enumerate() {
            map.push(ch);
            match ch {
                b'G' => {
                    units.push((y, x, Unit::Gnome, 200));
                }
                b'E' => {
                    units.push((y, x, Unit::Elf, 200));
                }
                _ => (),
            }
        }
    }
    let mut round;
    let units = 'a: loop {
        println!("elf_attack_power = {}", elf_attack_power);
        round = 0;
        let mut units = units.clone();
        let mut map = map.clone();
        'l: loop {
            units.sort_unstable();
            println!("{}", round);
            pretty_print(width, &map, &units);
            
            let mut unit_inde: isize = 0;
            while unit_inde < units.len() as isize {
                let unit_index = unit_inde as usize;
                let gnomes: Vec<_> = units.iter().cloned().filter(|v| v.2 == Unit::Gnome).collect();
                let elves: Vec<_> = units.iter().cloned().filter(|v| v.2 == Unit::Elf).collect();
                if gnomes.len() == 0 || elves.len() == 0 {
                    break 'l;
                }
                let unit = units[unit_index];
                let mut unit = unit.0*width+unit.1;
                let mut spots_in_range: Vec<_> = match units[unit_index].2 {
                    Unit::Elf => &gnomes,
                    Unit::Gnome => &elves,
                }.iter()
                .map(|v| v.0*width+v.1)
                .flat_map(|i| iter![i+1, i-1, i+width, i-width])
                .filter(|&i| map[i] == b'.' || i == unit)
                .map(|i| (i, ((i%width) as isize-(unit%width) as isize).abs()+((i/width) as isize - (unit/width) as isize).abs()))
                .collect();
                spots_in_range.sort_unstable_by_key(|v| v.1);
                if spots_in_range.get(0).map(|v| v.0) != Some(unit) {
                    // breadth first flood fill and dijkstra's alg
                    let mut queue = VecDeque::new();
                    //           distance, current, previous
                    queue.push_back((1u64, unit+1, vec![unit]));
                    queue.push_back((1u64, unit-1, vec![unit]));
                    queue.push_back((1u64, unit+width, vec![unit]));
                    queue.push_back((1u64, unit-width, vec![unit]));
                    let mut hashmap: HashMap<usize, (u64, Vec<usize>)> = HashMap::new();
                    while queue.len() != 0 {
                        let c = queue.pop_front().unwrap();
                        if map[c.1] != b'.' { continue; }
                        match hashmap.get_mut(&c.1) {
                            Some(v) => if c.0 < v.0 {
                                *v = (c.0, c.2);
                            } else if c.0 == v.0 {
                                v.1.extend(c.2.iter());
                                continue;
                            } else {
                                continue;
                            }
                            None => {
                                hashmap.insert(c.1, (c.0, c.2));
                            }
                        }
                        queue.push_back((c.0+1, c.1+1, vec![c.1]));
                        queue.push_back((c.0+1, c.1-1, vec![c.1]));
                        queue.push_back((c.0+1, c.1+width, vec![c.1]));
                        queue.push_back((c.0+1, c.1-width, vec![c.1]));
                    }
                    // println!("{:?}", hashmap);
                    let mut sorted: Vec<_> = spots_in_range
                        .iter()
                        .filter_map(|v| hashmap.get(&v.0).map(|d| (v.0, d)))
                        .collect();
                    sorted.sort_unstable_by_key(|v| (v.1 .0, v.0));
                    // println!("{}: {:?}", map[unit] as char, spots_in_range.iter().map(|v| (v.0%width, v.0/width, v.1)).collect::<Vec<_>>());
                    // println!("{}: {:?}", map[unit] as char, sorted.iter().map(|v| (v.0%width, v.0/width, v.1)).collect::<Vec<_>>());
                    // let stdout = std::io::stdout();
                    // let mut stdout = stdout.lock();
                    // for y in 0..map.len()/width {
                    //     for x in 0..width {
                    //         if spots_in_range.iter().any(|v| v.0 == y*width+x) {
                    //             stdout.write(b"?");
                    //         } else {
                    //             stdout.write(&[map[y*width+x]]);
                    //         }
                    //     }
                    //     stdout.write(b"\n");
                    // }
                    sorted.first()
                        .map(|(c, (_, p))| {
                            let c = p.iter().cloned().map(|mut p| {
                                let mut c = *c;
                                while p != unit {
                                    c = p;
                                    p = *hashmap.get(&p).unwrap().1.iter().min().unwrap();
                                }
                                c
                            }).min().unwrap();
                            units[unit_index].0 = c/width;
                            units[unit_index].1 = c%width;
                            map[unit] = b'.';
                            map[c] = match units[unit_index].2 {
                                Unit::Gnome => b'G',
                                Unit::Elf => b'E',
                            };
                            unit = c;
                        });
                }
                // check for adjacent and attacc
                let mut adjacent = match units[unit_index].2 {
                    Unit::Elf => &gnomes,
                    Unit::Gnome => &elves,
                }.iter().filter(|v| (v.0 as isize - units[unit_index].0 as isize).abs() == 1
                                && (v.1 as isize - units[unit_index].1 as isize).abs() == 0
                                || (v.1 as isize - units[unit_index].1 as isize).abs() == 1
                                && (v.0 as isize - units[unit_index].0 as isize).abs() == 0).collect::<Vec<_>>();
                adjacent.sort_unstable_by_key(|v| (v.3, v.0*width+v.1));
                if let Some(v) = adjacent.first() {
                    let it = {
                        let mut it = units.iter_mut().enumerate().find(|(_, x)| x == v).unwrap();
                        match it.1 .2 {
                            Unit::Gnome => it.1 .3 -= elf_attack_power,
                            Unit::Elf => it.1 .3 -= 3,
                        }
                        (it.0, *it.1)
                    };
                    // println!("unit at {},{} attacks unit at {},{}", units[unit_index].1, units[unit_index].0, it.1 .1, it.1 .0);
                    if it.1 .3 <= 0 {
                        if it.1 .2 == Unit::Elf {
                            elf_attack_power += 1;
                            continue 'a;
                        }
                        units.remove(it.0);
                        if it.0 < unit_index { unit_inde -= 1; }
                        map[it.1 .0*width+it.1 .1] = b'.';
                    }
                }
                // println!("{}: {:?}", map[unit] as char, adjacent);
                unit_inde += 1;
            }
            // println!("{:?}", units);
            // let stdout = std::io::stdout();
            // let mut stdout = stdout.lock();
            // for y in 0..map.len()/width {
            //     for x in 0..width {
            //         stdout.write(&[map[y*width+x]]);
            //     }
            //     stdout.write(b"\n");
            // }
            round += 1;            
        }
        break units;
    };
    println!("{} rounds, {:?}", round, units);
    pretty_print(width, &map, &units);
    

    round * units.iter().map(|v| v.3).sum::<i32>() as usize
}

fn pretty_print(width: usize, map: &Vec<u8>, units: &Vec<(usize, usize, Unit, i32)>) {
    use std::collections::HashMap;
    let m: HashMap<(usize, usize), Unit> = units.iter().map(|(x, y, uni, _)| {
        ((*y, *x), *uni)
    }).collect();

    println!("{:?}", m);
    for y in 0..(map.len() / width) {        
        for x in 0..width {
            if m.contains_key(&(x, y)) {
                match m[&(x, y)] {
                    Unit::Elf => print!("E"),
                    Unit::Gnome => print!("G"),                    
                }
            } else {
                match map[x + y * width] {
                    b'#' => print!("#"),
                    _ => print!(".")
                }
                
            }
        }
        println!();
    }
}
