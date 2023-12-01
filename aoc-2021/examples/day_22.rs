use aoc_2021::{stopwatch, vector::Vector3};
use std::collections::{HashMap, HashSet, BinaryHeap};

type Input = Vec<Cuboid>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cuboid {
    id: usize,
    on: bool,
    min: Vector3,
    max: Vector3,
}

impl Cuboid {
    pub fn overlaps(&self, other: &Self) -> bool {
        let no_x_overlap = self.max[0] < other.min[0] || self.min[0] > other.max[0];
        let no_y_overlap = self.max[1] < other.min[1] || self.min[1] > other.max[1];
        let no_z_overlap = self.max[2] < other.min[2] || self.min[2] > other.max[2];

        !(no_x_overlap || no_y_overlap || no_z_overlap)
    }

    pub fn magnitude(&self) -> isize {
        let x = self.max[0] - self.min[0];
        let y = self.max[1] - self.min[1];
        let z = self.max[2] - self.min[2];

        x * y * z
    }
}

// Left to right
#[derive(Debug, PartialEq, PartialOrd)]
enum SweepPlaneEvent {
    Start(Vector3, Vector3, usize, bool),
    End(isize, usize),
}

#[derive(Debug, PartialEq)]
enum SweepLineEvent {
    Start((isize, isize), isize, usize, bool),
    End(isize, usize),
}

#[derive(Debug, PartialEq)]
enum SweepPointEvent {
    Start(isize, usize, bool),
    End(isize, usize),
}

pub fn main() {
    let input = parse_input(include_str!("../input/test.txt"));
    println!(
        "e1: {:?}",
        stopwatch(|| {
            println!("Ex1: {}", exercise_1(&input));
        })
    );
    println!(
        "e2: {:?}",
        stopwatch(|| {
            println!("Ex1: {}", exercise_2(&input));
        })
    );
}

fn parse_input(input: &str) -> Input {
    input.lines().enumerate().map(parse_line).collect()
}

fn parse_line((id, line): (usize, &str)) -> Cuboid {
    let mut split = line.split(' ');
    let onoff = split.next();
    let coords = split.next().unwrap();

    let on = match onoff {
        Some("on") => true,
        Some("off") => false,
        _ => unreachable!(),
    };

    let mut split = coords.split(',');
    let x = split.next().unwrap();
    let y = split.next().unwrap();
    let z = split.next().unwrap();
    let mut x = x[2..].split("..");
    let mut y = y[2..].split("..");
    let mut z = z[2..].split("..");

    let minx: isize = x.next().unwrap().parse().unwrap();
    let miny = y.next().unwrap().parse().unwrap();
    let minz = z.next().unwrap().parse().unwrap();

    let maxx: isize = x.next().unwrap().parse::<isize>().unwrap() + 1;
    let maxy = y.next().unwrap().parse::<isize>().unwrap() + 1;
    let maxz = z.next().unwrap().parse::<isize>().unwrap() + 1;

    let min = Vector3::new([minx, miny, minz]);
    let max = Vector3::new([maxx, maxy, maxz]);

    Cuboid {
        id: id + 1,
        on,
        min,
        max,
    }
}

fn exercise_1(input: &Input) -> usize {
    let check = Cuboid {
        id: 0,
        on: false,
        min: Vector3::new([-50, -50, -50]),
        max: Vector3::new([50, 50, 50]),
    };

    calculate_for(&check, input).len()
}

fn calculate_for(check: &Cuboid, input: &Vec<Cuboid>) -> HashSet<(isize, isize, isize)> {
    let mut set = HashSet::with_capacity(check.magnitude() as usize / 2);
    for cuboid in input {
        if !cuboid.overlaps(&check) && check.id > cuboid.id {
            continue;
        }

        let minx = cuboid.min[0].max(check.min[0]);
        let miny = cuboid.min[1].max(check.min[1]);
        let minz = cuboid.min[2].max(check.min[2]);

        let maxx = cuboid.max[0].min(check.max[0]);
        let maxy = cuboid.max[1].min(check.max[1]);
        let maxz = cuboid.max[2].min(check.max[2]);

        for x in minx..maxx {
            for y in miny..maxy {
                for z in minz..maxz {
                    if cuboid.on {
                        set.insert((x, y, z));
                    } else {
                        set.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    set
}

fn exercise_2(input: &Input) -> usize {
    collect_plane_events(input)
        .into_iter()
        .fold(
            (SweepPlaneInfo::default(), isize::MIN, 0),
            |(mut spi, old_x, counter), e| match e {
                SweepPlaneEvent::Start(min, max, id, on) => {
                    let size = spi.calculate_size((min[0] - old_x) as usize);

                    spi.add(Cuboid { id, min, max, on });

                    (spi, min[0], counter + size)
                }
                SweepPlaneEvent::End(new_x, id) => {
                    let size = spi.calculate_size((new_x - old_x) as usize);

                    spi.remove(id);

                    (spi, new_x, counter + size)
                }
            },
        )
        .2
}

fn collect_plane_events(input: &[Cuboid]) -> Vec<SweepPlaneEvent> {
    let mut events: Vec<SweepPlaneEvent> = input
        .iter()
        .flat_map(|a| {
            let min_start = a.min;
            let max_start = Vector3::new([a.min[0], a.max[1], a.max[2]]);

            [
                SweepPlaneEvent::Start(min_start, max_start, a.id, a.on),
                SweepPlaneEvent::End(a.max[0], a.id),
            ]
        })
        .collect();

    events.sort_by_key(|a| match a {
        SweepPlaneEvent::Start(a, _, c, _) => (a[0], *c),
        SweepPlaneEvent::End(a, b) => (*a, *b),
    });

    events
}

#[derive(Debug, Default)]
struct SweepPlaneInfo {
    cubes: Vec<Cuboid>,
}

impl SweepPlaneInfo {
    pub fn add(&mut self, cube: Cuboid) {
        self.cubes.push(cube);
        self.cubes.sort_by_key(|x| x.id);
    }

    pub fn remove(&mut self, cube: usize) {
        self.cubes = self
            .cubes
            .iter()
            .filter(|x| x.id != cube)
            .cloned()
            .collect();
    }

    pub fn calculate_size(&self, dif: usize) -> usize {
        if dif == 0 {
            return 0;
        }

        return collect_line_events(&self.cubes)
            .into_iter()
            .fold(
                (SweepLineInfo::default(), isize::MIN, 0),
                |(mut sli, old_y, counter), e| match e {
                    SweepLineEvent::Start(range, y, id, on) => {
                        let size = sli.calculate_size((y - old_y) as usize);

                        sli.add(range, id, on);

                        (sli, y, size + counter)
                    }
                    SweepLineEvent::End(y, id) => {
                        let size = sli.calculate_size((y - old_y) as usize);

                        sli.remove(id);

                        (sli, y, size + counter)
                    }
                },
            )
            .2
            * dif;
    }
}

fn collect_line_events(input: &[Cuboid]) -> Vec<SweepLineEvent> {
    let mut events: Vec<SweepLineEvent> = input
        .iter()
        .flat_map(|c| {
            let x_range = (c.min[1], c.max[1]);
            [
                SweepLineEvent::Start(x_range, c.min[2], c.id, c.on),
                SweepLineEvent::End(c.max[2], c.id),
            ]
        })
        .collect();

    events.sort_by_key(|x| match x {
        SweepLineEvent::Start(_, y, id, _) => (*y, *id),
        SweepLineEvent::End(y, id) => (*y, *id),
    });

    events
}

#[derive(Debug, Default)]
struct SweepLineInfo {
    map: HashMap<usize, ((isize, isize), usize, bool)>,
}

impl SweepLineInfo {
    fn calculate_size(&self, dif: usize) -> usize {
        if dif == 0 {
            return 0;
        }

        collect_point_events(self.map.values())
            .into_iter()
            .fold(
                (SweepPointInfo::default(), isize::MIN, 0),
                |(mut spi, old_x, counter), e| match e {
                    SweepPointEvent::Start(x, id, on) => {
                        let size = spi.calculate_size((x - old_x) as usize);

                        spi.add(id, on);

                        (spi, x, size + counter)
                    }
                    SweepPointEvent::End(x, id) => {
                        let size = spi.calculate_size((x - old_x) as usize);

                        spi.remove(id);

                        (spi, x, size + counter)
                    }
                },
            )
            .2
            * dif
    }

    fn add(&mut self, range: (isize, isize), id: usize, on: bool) {
        self.map.insert(id, (range, id, on));
    }

    fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }
}

fn collect_point_events(
    values: std::collections::hash_map::Values<usize, ((isize, isize), usize, bool)>,
) -> Vec<SweepPointEvent> {
    let mut events = values
        .flat_map(|x| {
            [
                SweepPointEvent::Start(x.0 .0, x.1, x.2),
                SweepPointEvent::End(x.0 .1, x.1),
            ]
        })
        .collect::<Vec<_>>();

    events.sort_by_key(|a| match a {
        SweepPointEvent::Start(y, id, _) => (*y, *id),
        SweepPointEvent::End(y, id) => (*y, *id),
    });

    events
}

#[derive(Debug, Default)]
struct SweepPointInfo {
    map: HashMap<usize, (usize, bool)>,
}

impl SweepPointInfo {
    fn calculate_size(&self, diff: usize) -> usize {
        if diff == 0 {
            return 0;
        }

        let f = self.map.values().max_by_key(|p| p.0);

        match f {
            Some((_, true)) => diff,
            _ => 0,
        }
    }

    fn add(&mut self, id: usize, on: bool) {
        self.map.insert(id, (id, on));
    }

    fn remove(&mut self, id: usize) {        
        self.map.remove(&id);
    }
}
