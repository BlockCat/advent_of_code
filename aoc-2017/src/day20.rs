use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;

// a = v / s 
// v = m /s
// =>
// v = a*s
//

// a_1(x)=p_1 + ((a_1x^2 + a_1x)/2 + v_1x)
// b_1(x)=p_2 + ((a_2x^2 + a_2x)/2 + v_2x)
// c_1(x)=p_3 + ((a_3x^2 + a_3x)/2 + v_3x)


// a_2(x)=p_4 + ((a_4x^2 + a_4x)/2 + v_4x)
// b_2(x)=p_5 + ((a_5x^2 + a_5x)/2 + v_5x)
// c_2(x)=p_6 + ((a_6x^2 + a_6x)/2 + v_6x)

// p_1 +(a_1/2)x^2 + a_1/2x + v_1x = p_4 + (a_4/2)x^2 + a_4/2x + v_4x
// (p_1 - p_4) + (a_1/2)x^2 - (a_4/2)x^2 + v_1x - v_4x = 0
// (a_1 - a_4)x^2 + 2(v_1 - v_4)x + 2(p_1-p_4) = 0

// x = (-2(v_1-v_4) +- sqrt(4(v_1-v_4)^2-4*(a_1 - a_4)*(p_1-p_4)) / (2 * (a_1 - a_4))
// x = (-2(v_1-v_4) +- sqrt(4(v_1-v_4)^2-4*(a_1 - a_4)*(p_1-p_4)) / (2 * (a_1 - a_4)) // y


// ax^2 + bx + c = 0
// x = (-b +- sqrt(b^2 - 4ac)) / 2a

// v(x)= (a_1x^2)^2 + (a_2x^2)^2 + (a_3x^2)^2
// a_1^2x^4+a_2^2x^4+a_3^2x^4 = (a_1^2 + a_2^2 + a_3^2) x^4
// Therefore find the one with the biggest (a_1^2 + a_2^2 + a_3^2)

#[derive(PartialEq, Clone, Copy, Debug)]
struct Vector(i64, i64, i64);

struct Particle {
    pub location: Vector,
    pub speed: Vector,
    pub acceleration: Vector
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl std::ops::Mul<i64> for Vector {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Vector(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl std::ops::Div<i64> for Vector {
    type Output = Self;

    fn div(self, other: i64) -> Self {
        Vector(self.0 / other, self.1 / other, self.2 / other)
    }
}


impl Particle {

    fn rising(&self) -> i64 {
        self.acceleration.0.pow(2) + self.acceleration.1.pow(2) + self.acceleration.2.pow(2)
    }

    // a_1(x)=p_1 + ((a_1x + a_1)x/2 + v_1x)
    // b_1(x)=p_2 + ((a_2x^2 + a_2x)/2 + v_2x)
    // c_1(x)=p_3 + ((a_3x^2 + a_3x)/2 + v_3x)
    fn ft(&self, time: i64) -> Vector {
        self.location + ((self.acceleration * time + self.acceleration) * time) / 2 + self.speed * time
    }
}

impl FromStr for Particle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // p=<255,-544,641>, v=<-21,93,-83>, a=<-1,-7,4>
        // p,255,-544,641, v,-21,93,-83, a,-1,-7,4
        let s = s.replace("=<", ",").replace('>', "");
        let mut split = s.split(',');
        //println!("{}", s);
        
        let x  = split.nth(1).unwrap().parse()?;
        let y  = split.nth(0).unwrap().parse()?;
        let z  = split.nth(0).unwrap().parse()?;
        let vx = split.nth(1).unwrap().parse()?;
        let vy = split.nth(0).unwrap().parse()?;
        let vz = split.nth(0).unwrap().parse()?;
        let ax = split.nth(1).unwrap().parse()?;
        let ay = split.nth(0).unwrap().parse()?;
        let az = split.nth(0).unwrap().parse()?;

        Ok(Particle { 
            location: Vector(x, y, z),
            speed: Vector(vx, vy, vz),
            acceleration: Vector(ax, ay, az)
        })
    }
}

fn read_input(input: &str) -> Vec<Particle> {
    input.lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn algorthm1(input: &Vec<Particle>) -> usize {
    input.iter().enumerate()
        .map(|(index, particle)| (particle.rising(), index))        
        .min()
        .unwrap().1
}

fn algorthm2(input: &Vec<Particle>) -> usize {
    let mut grid = vec![vec![None; input.len()]; input.len()];
    let mut shortest: Vec<Option<usize>> = vec![None; input.len()];

    for i in 0..grid.len() {
        for j in (i+1)..grid.len() {            
            if let Some(time) = collision_time(&input[i], &input[j]) {
                grid[i][j] = Some(time);
                grid[j][i] = Some(time);
                shortest[i] = if let Some(si) = shortest[i] {
                    Some(std::cmp::min(si, time))
                } else {
                    Some(time)
                };
                shortest[j] = if let Some(si) = shortest[j] {
                    Some(std::cmp::min(si, time))
                } else {
                    Some(time)
                };
            }            
        }
    }
/*
    for l in &grid {
        println!("{:?}", l);
    }*/

    let mut removed = vec![false; input.len()];
    let mut collisions = 0;
    let mut found = false;
    for i in 0..grid.len() {
        found = false;
        if !removed[i] {
            if let Some(si) = shortest[i] {
                for j in (i+1)..grid.len() {
                    if let Some(sj) = shortest[j] {
                        if si == sj {
                            removed[j] = true;
                            collisions += 1;  
                            found = true;                      
                        }
                    }
                }
            }
        }
        if found {
            collisions += 1;
        }
    }

    input.len() - collisions
}

fn collision_time(pa: &Particle, pb: &Particle) -> Option<usize> {    
    for i in 0..1000 {
        if pa.ft(i) == pb.ft(i) {
            return Some(i as usize);
        }
    }
    None
    /*let a = pa.acceleration.0 - pb.acceleration.0;
    let b = (2 * (pa.speed.0 - pb.speed.0));
    let c = (2 * (pa.location.0 - pb.location.0));

    let D = b*b - 4 * a * c;

    if a == 0 {
        if b == 0 {
            None 
        } else {
            let x = -c / b;
            if x >= 0 && pa.ft(x) == pb.ft(x) {
                Some(x as usize)
            } else {
                None
            }

        }
    } else if D >= 0 {
        let dsq = ((D as f64).sqrt()) as i64;        
        let x1 = (-b - dsq) / (2 * a);
        let x2 = (-b + dsq) / (2 * a);
        if x1 - 1 >= 0 && pa.ft(x1 - 1) == pb.ft(x1 - 1) {
            Some(x1 as usize - 1)
        } else if x1 >= 0 && pa.ft(x1) == pb.ft(x1) {
            Some(x1 as usize)
        } else if x1 + 1 >= 0 && pa.ft(x1 + 1) == pb.ft(x1 + 1) {
            Some(x1 as usize + 1)
        } else if x2 - 1 >= 0 && pa.ft(x2 - 1) == pb.ft(x2 - 1) {
            Some(x2 as usize)
        } else if x2 >= 0 && pa.ft(x2) == pb.ft(x2) {
            Some(x2 as usize)
        } else if x2 + 1 >= 0 && pa.ft(x2 + 1) == pb.ft(x2 + 1) {
            Some(x2 as usize)
        } else {
            None
        }
    } else {
        return None;
    }*/
}


#[test]
fn test_examples() {
    let input = read_input(include_str!("input/day20test.txt"));
    let input2 = read_input(include_str!("input/day20test2.txt"));
    assert_eq!(algorthm1(&input), 0);
    assert_eq!(algorthm2(&input2), 1);
}


#[test]
fn run20() {
    let input = read_input(include_str!("input/day20.txt"));
    println!("Closest {}", algorthm1(&input));
    println!("Left {}", algorthm2(&input)); //should be 477
}