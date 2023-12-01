use hashbrown::HashMap;

mod preprocess {
    use std::str::FromStr;
    use std::num::ParseIntError;    

    #[derive(Debug, Copy, Clone)]
    struct Date {
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32
    }


    impl FromStr for Date {
        type Err = ParseIntError;

        fn from_str(source: &str) -> Result<Self, Self::Err> {
            Ok(Date {
                year: source[1..5].parse().unwrap(),
                month: source[6..8].parse().unwrap(),
                day: source[9..11].parse().unwrap(),
                hour: source[12..14].parse().unwrap(),
                minute: source[15..17].parse().unwrap()
            })            
        }
    }

    #[derive(Debug)]
    enum GuardEvent {
        StartShift(Date, i32),
        StartSleep(Date),
        EndSleep(Date)
    }

    impl FromStr for GuardEvent {
        type Err = ();
        
        fn from_str(source: &str) -> Result<Self, ()> {
            let date = source.parse::<Date>().unwrap();
            let significant_char = &source[19..20];       

            match significant_char {
                "G" => {                    
                    let guard = source[26..source.find(" b").unwrap()].parse().unwrap();
                    Ok(GuardEvent::StartShift(date, guard))
                },
                "f" => Ok(GuardEvent::StartSleep(date)),
                "w" => Ok(GuardEvent::EndSleep(date)),
                _ => Err(())
            }
        }
    }
    
    pub fn pre_process(input: &str) -> Vec<(i32, i32, i32)> {
            
        #[derive(Debug, PartialEq)] enum State {
            Sleep {guard: i32, sleep: i32, wake: i32},
            Wait
        };
        let mut v: Vec<GuardEvent> = input.lines().map(|l| { l.parse::<GuardEvent>().unwrap()}).collect();
        
        v.sort_by(|a, b| {
            let ad = match a {
                GuardEvent::StartShift(d, _) => d,
                GuardEvent::StartSleep(d) => d,
                GuardEvent::EndSleep(d) => d,
            };

            let bd = match b {
                GuardEvent::StartShift(d,_) => d,
                GuardEvent::StartSleep(d) => d,
                GuardEvent::EndSleep(d) => d,
            };

            let aa = [ad.year, ad.month, ad.day, ad.hour, ad.minute];
            let ba = [bd.year, bd.month, bd.day, bd.hour, bd.minute];

            aa.cmp(&ba)
        });

        v.into_iter().scan((0i32, 0i32), |(guard, sleep), event| {
            match event {
                GuardEvent::StartShift(_, g) => {
                    *guard = g;
                    Some(State::Wait)}
                GuardEvent::StartSleep(d) => { 
                    *sleep = d.minute; 
                    Some(State::Wait)}
                GuardEvent::EndSleep(d) => {
                    Some(State::Sleep {
                        guard: *guard, 
                        sleep: *sleep, 
                        wake: d.minute
                    })                
                }
            }        
        }).filter_map(|s| match s {
            State::Sleep { guard, sleep, wake} => Some((guard, sleep, wake)),
            _ => None
        }).collect()    
    }
}


pub fn execute_exercises() {
    //preprocess::pre_process(include_str!("../input/day4_in.txt")).into_iter().for_each(|(guard, sleep, wake)| println!("{} {} {}", guard, sleep, wake));
    println!("strat 1: {}", exercise_1(read_input()));
    println!("strat 2: {}", exercise_2(read_input()));    
}

fn read_input() -> Vec<(i32, i32, i32)> {  
    read_input_str(include_str!("../input/day4_preprocessed.txt"))    
}

fn read_input_str(input:  &str) -> Vec<(i32, i32, i32)> {
    input.lines().map(|l| { 
        let spl: Vec<&str> = l.split(' ').collect();        
        (spl[0].parse::<i32>().unwrap(), spl[1].parse::<i32>().unwrap(), spl[2].parse::<i32>().unwrap())
    }).collect()
}


fn exercise_1(input: Vec<(i32, i32, i32)>) -> i32 {    
    let guard_sleep = input.iter()
        .fold(HashMap::with_capacity(100), |mut acc, &(guard, sleep, wake)| {
            *acc.entry(guard).or_insert(0) += wake - sleep;
            acc
        });

    let (sleepy_guard, _) = guard_sleep.into_iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    let minutes = input.into_iter()
        .filter(|(guard, _, _)| *guard == sleepy_guard)        
        .fold([0u8; 60], |mut acc, (_, sleep, wake)| {            
            for m in sleep..wake {
                acc[m as usize] += 1;
            }            
            acc
        });
        
    let (minute, _) = minutes.into_iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))        
        .unwrap();        
    

    sleepy_guard * (minute as i32)
}

fn exercise_2(input: Vec<(i32, i32, i32)>) -> i32 {    

    let guard_minutes = input.into_iter().fold(HashMap::with_capacity(200), |mut acc, (guard, sleep, wake)| {
        let minutes = acc.entry(guard).or_insert([0u8; 60]);
        for m in sleep..wake {
            minutes[m as usize] += 1;            
        }        
        
        acc
    });
    
    let (guard, (minute, _)) = guard_minutes.into_iter().map(|(guard, minutes)| {
        // (guard, (minute, sleeps))        
        (guard, minutes.into_iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(a, &b)| (a, b))
            .unwrap())
    }).max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
    .unwrap();

    guard * (minute as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::preprocess::*;
    use crate::test::Bencher;


    #[test]
    fn d4_ex1_s1() {
        let c = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let input = pre_process(c);
        assert_eq!(exercise_1(input), 240);
    }
    
    #[test]
    fn d4_ex1_s2() {     
        assert_eq!(exercise_1(read_input()), 36898);   
    }

    #[test]
    fn d4_ex2_s1() {
      let c = 
r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let input = pre_process(c);
        assert_eq!(exercise_2(input), 4455);
    }

    #[test]
    fn d4_ex2_s2() {
        assert_eq!(exercise_2(read_input()), 80711);
    }

    #[test]
    fn d4_ex2_s3() {        
    }

    #[bench]
    fn d4_preprocess(b: &mut Bencher) {
        b.iter(|| super::preprocess::pre_process(include_str!("../input/day4_in.txt")));
    }

    #[bench]
    fn d4_read(b: &mut Bencher) {
       b.iter(|| read_input());
    }

    #[bench]
    fn d4_ex1_bench(b: &mut Bencher) {
        b.iter(|| exercise_1(read_input()));
    }

    #[bench]
    fn d4_ex2_bench(b: &mut Bencher) {
       b.iter(|| exercise_2(read_input()));
    }
}