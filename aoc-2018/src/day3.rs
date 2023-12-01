use std::collections::BTreeSet;

use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Area {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Area {
    fn new(id: i32, x: i32, y: i32, width: i32, height: i32) -> Area {
        Area { id, x, y, width, height }
    }

    fn intersects(&self, other: &Area) -> bool {
        self.id != other.id 
        && !(self.x >= other.x + other.width || other.x >= self.x + self.width || self.y >= other.y + other.height || other.y >= self.y + self.height)
    }
}


impl FromStr for Area {
    type Err = ParseIntError;    

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        
        //let parts: Vec<&str> = source.split(' ').collect();
        /*let id = parts[0].replace('#', "").parse::<i32>()?;
        let loc: Vec<i32> = parts[2].replace(':', "").split(',').map(|n| n.parse().unwrap()).collect();
        let size: Vec<i32> = parts[3].split('x').map(|n| n.parse().unwrap()).collect();*/
        let id = source[0..4].parse()?;
        let (x, y) = (source[5..9].parse()?, source[10..14].parse()?);
        let (w, h) = (source[15..19].parse()?, source[20..24].parse()?);
        
        Ok(Area::new(
            id,
            x,
            y,
            w,
            h
        ))        
    }
}

mod sweepline {
    use super::Area;

    #[derive(Eq, PartialEq, Debug)]
    pub enum SweepEvent<'a> {
        StartArea(&'a Area, i32),
        EndArea(&'a Area, i32)
    }

    impl<'a> std::cmp::Ord for SweepEvent<'a> {

        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let sy = match self {
                SweepEvent::StartArea(_, y) => y,
                SweepEvent::EndArea(_, y) => y
            };
            let oy = match other {
                SweepEvent::StartArea(_, y) => y,
                SweepEvent::EndArea(_, y) => y
            };

            sy.cmp(oy)
        }
    }

    impl<'a> std::cmp::PartialOrd for SweepEvent<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, PartialEq, PartialOrd, Eq)]
    pub enum HorizontalSwipe {
        Start(i32, i32),
        End(i32, i32)
    }

    impl std::cmp::Ord for HorizontalSwipe {

        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let sy = match self {
                HorizontalSwipe::Start(id, y) => (y, 0, id),
                HorizontalSwipe::End(id, y) => (y, 1, id)
            };
            let oy = match other {
                HorizontalSwipe::Start(id, y) => (y, 0, id),
                HorizontalSwipe::End(id, y) => (y, 1, id)
            };

            sy.cmp(&oy)
        }
    }
}

use self::sweepline::*;

pub fn execute_exercises() {        
    println!("Overlapping inches: {}", exercise_1_sl(read_input()));
    println!("Non overlapping: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<Area> {    
    include_str!("../input/day3_preprocessed.txt").lines().map(|l| l.parse::<Area>().unwrap()).collect()
}

fn exercise_1_sl(input: Vec<Area>) -> i32 {
    
    let mut event_vector: Vec<SweepEvent> = input.iter()
        .map(|s| SweepEvent::StartArea(s, s.y))        
        .chain(input.iter().map(|s| SweepEvent::EndArea(s, s.y + s.height)))
        .collect();

    event_vector.sort();
    let mut state: BTreeSet<HorizontalSwipe> = BTreeSet::new(); // Should be some ordered lists of

    event_vector.iter().fold((0i32, 0i32, 0i32), |(prev_y, prev_overlap, result), event| {
        match event {
            SweepEvent::StartArea(b, y) => {
                let n_result = result + (y - prev_y) * prev_overlap; // Increment minutes with overlapped
                // Add x and x+width to state.
                state.insert(HorizontalSwipe::Start(b.id, b.x));                
                state.insert(HorizontalSwipe::End(b.id, b.x + b.width));                
                
                // Calculate overlapping
                let overlapped = count_overlapped(&state);
                (*y, overlapped, n_result)
            },
            SweepEvent::EndArea(b, y) => {
                let n_result = result + (y - prev_y) * prev_overlap; // Increment minutes with overlapped
                // Remove x and x+width to state.
                state.remove(&HorizontalSwipe::Start(b.id, b.x));
                state.remove(&HorizontalSwipe::End(b.id, b.x + b.width));
                
                // Calculate overlapping
                let overlapped = count_overlapped(&state);
                (*y, overlapped, n_result)
            }
        }        
    }).2
}

fn count_overlapped(line: &BTreeSet<HorizontalSwipe>) -> i32 {

    line.iter().fold((0i32, 0i32, 0i32), |(overlap, prev_x, counter), event| {        
        match event {
            HorizontalSwipe::Start(_, x) => {
                let increase = if counter > 1 {
                    *x - prev_x
                } else {
                    0
                };

                (overlap + increase , *x, counter + 1)
            },
            HorizontalSwipe::End(_, x) => {
                let increase = if counter > 1 {
                    *x - prev_x
                } else {
                    0
                };

                (overlap + increase , *x, counter - 1)
            }
        }        
    }).0
    
}

fn exercise_2(input: Vec<Area>) -> i32 {
    input.iter().find(|x| !input.iter().any(|y| x.intersects(y))).unwrap().id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Bencher;

    #[test]
    fn d3_ex1_s1() {
        let inputs = vec!(Area::new(1, 1, 3, 4, 4), Area::new(2, 3, 1, 4, 4), Area::new(3, 5, 5, 2, 2));
        assert_eq!(exercise_1_sl(inputs), 4);
    }
    
    #[test]
    fn d3_ex1_s2() {
        let inputs = vec!(Area::new(1, 0, 0, 4, 4), Area::new(2, 3, 0, 4, 4), Area::new(3, 4, 4, 2, 2));
        assert_eq!(exercise_1_sl(inputs), 4);
    }

    #[test]
    fn d3_ex1_s3() {
        assert_eq!(exercise_1_sl(read_input()), 118322);
    }

    #[test]
    fn d3_ex1_s4() {
        let inputs = vec!(Area::new(1, 1, 3, 4, 4), Area::new(2, 3, 1, 4, 4), Area::new(3, 5, 5, 2, 2));
        assert_eq!(exercise_1_sl(inputs), 4);
    }


    #[test]
    fn d3_ex1_s5() {
        assert_eq!(exercise_1_sl(read_input()), 118322);
    }

    #[test]
    fn d3_ex2_s1() {
        let inputs = vec!(Area::new(1, 1, 3, 4, 4), Area::new(2, 3, 1, 4, 4), Area::new(3, 5, 5, 2, 2));
        assert_eq!(exercise_2(inputs), 3);
    }

    #[bench]
    fn d3_read(b: &mut Bencher) {
        b.iter(|| read_input());
    }   

    #[bench]
    fn d3_ex2_bench(b: &mut Bencher) {
        b.iter(|| exercise_2(read_input()));
    }

    #[bench]
    fn d3_ex1_bench_sl(b: &mut Bencher) {
        b.iter(|| exercise_1_sl(read_input()));
    }
}