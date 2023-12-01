use crate::test::Bencher;

use hashbrown::HashMap;
use hashbrown::HashSet;
use utils::Vector3;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Planet {
    id: usize,
    pos: utils::Vector3,
    vel: utils::Vector3,
}

impl Planet {
    fn new(id: usize, x: isize, y: isize, z: isize) -> Planet {
        Planet {
            id,
            pos: Vector3(x, y, z),
            vel: Vector3(0, 0, 0),
        }
    }
}

#[test]
pub fn run() {
    let input = vec![
        Planet::new(0, -3, 15, -11),
        Planet::new(1, 3, 13, -19),
        Planet::new(2, -13, 18, -2),
        Planet::new(3, 6, 0, -1),
    ];

    println!("ex1: {}", exercise_1(input.clone(), 1000));
    println!("ex2: {}", exercise_2(input));
}

fn exercise_1(mut input: Vec<Planet>, steps: usize) -> usize {
    for _ in 0..steps {
        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                let ij = (input[i].pos - input[j].pos).sign();
                let ji = (input[j].pos - input[i].pos).sign();
                input[j].vel += ij;
                input[i].vel += ji;
            }
        }

        for p in input.iter_mut() {
            p.pos += p.vel;
        }
    }

    // energy
    input
        .into_iter()
        .map(|x| {
            Vector3::manhattan(&Vector3(0, 0, 0), &x.pos)
                * Vector3::manhattan(&Vector3(0, 0, 0), &x.vel)
        })
        .sum()
}
fn exercise_2(mut input: Vec<Planet>) -> u128 {
    let mut sets: [HashMap<Vec<(isize, isize)>, i32>; 3] =
        [HashMap::new(), HashMap::new(), HashMap::new()];
    let mut vec_collec = [
        vec![(0, 0); input.len()],
        vec![(0, 0); input.len()],
        vec![(0, 0); input.len()],
    ];

    let mut xc = None;
    let mut yc = None;
    let mut zc = None;
    for step in 0.. {
        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                let ij = (input[i].pos - input[j].pos).sign();
                let ji = (input[j].pos - input[i].pos).sign();
                input[j].vel += ij;
                input[i].vel += ji;
            }
       }

        for p in input.iter_mut() {
            p.pos += p.vel;

            vec_collec[0][p.id] = (p.pos.0, p.vel.0);
            vec_collec[1][p.id] = (p.pos.1, p.vel.1);
            vec_collec[2][p.id] = (p.pos.2, p.vel.2);
        }

        if xc == None {
            if let Some(x_cycle) = sets[0].insert(vec_collec[0].clone(), step) {
                println!("x_c: {} starting at {}", step - x_cycle, x_cycle);
                xc = Some((step - x_cycle, x_cycle));
            }
        }
        if yc == None {            
            if let Some(x_cycle) = sets[1].insert(vec_collec[1].clone(), step) {
                println!("y_c: {} starting at {}", step - x_cycle, x_cycle);
                yc = Some((step - x_cycle, x_cycle));
            }
        }
        if zc == None {
            
            if let Some(x_cycle) = sets[2].insert(vec_collec[2].clone(), step) {
                println!("z_c: {} starting at {}", step - x_cycle, x_cycle);
                zc = Some((step - x_cycle, x_cycle));
            }
        }

        match (xc, yc, zc) {
            (Some((ax, _)), Some((ay, _)), Some((az, _))) => {
                let ax = ax as u128;
                let ay = ay as u128;
                let az = az as u128;
                let lcdxy = ax  / gcd_2(ax, ay) * ay;
                return (lcdxy / gcd_2(lcdxy, az)) * az;
            },
            _ => {}
        }
    }

    0
}

fn gcd_2(mut a: u128, mut b: u128) -> u128 {    
    if a == 0 || b == 0 {
        return std::cmp::max(a, b);
    }
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

#[test]
fn d11_test() {
    let input = vec![
        Planet::new(0, -8, -10, 0),
        Planet::new(1, 5, 5, 10),
        Planet::new(2, 2, -7, 3),
        Planet::new(3, 9, -8, -3),
    ];
    assert_eq!(exercise_1(input.clone(), 100), 1940);

    let input = vec![
        Planet::new(0, -1, 0, 2),
        Planet::new(1, 2, -10, -7),
        Planet::new(2, 4, -8, 8),
        Planet::new(3, 3, 5, -1),
    ];
    assert_eq!(exercise_2(input), 2772);
}

#[bench]
fn d11_bench_ex1(b: &mut Bencher) {
    let input = vec![
        Planet::new(0, -3, 15, -11),
        Planet::new(1, 3, 13, -19),
        Planet::new(2, -13, 18, -2),
        Planet::new(3, 6, 0, -1),
    ];
    b.iter(|| exercise_1(input.clone(), 1000));
}

#[bench]
fn d11_bench_ex2(b: &mut Bencher) {
    let input = vec![
        Planet::new(0, -3, 15, -11),
        Planet::new(1, 3, 13, -19),
        Planet::new(2, -13, 18, -2),
        Planet::new(3, 6, 0, -1),
    ];
    b.iter(|| exercise_2(input.clone()));
}

