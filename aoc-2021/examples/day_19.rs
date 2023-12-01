use aoc_2021::vector::Vector3;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::{HashMap, HashSet, VecDeque};

type Input = Vec<Scanner>;

const MATCHING_SIZE: usize = 12;
pub fn main() {
    let input = parse_input(include_str!("../input/day19.txt"));

    let duration = aoc_2021::stopwatch(|| {
        println!("Ex1: {:?}", exercise_12(&input));
    });

    println!("duration: {:?}", duration);
}

fn exercise_12(scanners: &Input) -> (usize, usize) {
    // Create a growing space from the first scanner
    let mut growth = GrowResult::new(&scanners[0]);

    let mut to_search = scanners[1..].iter().collect::<VecDeque<_>>();

    while let Some(x) = to_search.pop_front() {
        // Try adding a new scanner to the space, if that's not possible add it back to the queue so it can try again
        if !growth.try_add(x) {
            to_search.push_back(x);
        }
    }

    (growth.different_beacons(), growth.max_scanner_distance())
}

struct DirectionMatchInfo {
    source_beacon: Vector3,
    target_beacon: Vector3,
}

#[derive(Debug, Clone, Default)]
struct BeaconSet(HashSet<Vector3>);

impl BeaconSet {
    fn find_match(&self, beacons: &[BeaconSet]) -> Option<(usize, Vector3)> {
        let (rotation, info) = (0..24)
            .par_bridge()
            .find_map_any(|r| is_directional_match(self, &beacons[r]).map(|x| (r, x)))?;

        Some((rotation, info.source_beacon - info.target_beacon))
    }

    fn add_space(&mut self, offset: Vector3, space: &BeaconSet) {
        for beacon in &space.0 {
            self.0.insert(offset + *beacon);
        }
    }
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<BeaconSet>,
}

#[derive(Debug, Clone)]
struct GrowResult {
    scanners: HashMap<usize, (Vector3, BeaconSet)>,
}

impl GrowResult {
    pub fn new(scanner: &Scanner) -> Self {
        let mut scanners = HashMap::new();
        scanners.insert(scanner.id, (Vector3::zero(), scanner.beacons[0].clone()));
        GrowResult { scanners }
    }
    pub fn try_add(&mut self, scanner: &Scanner) -> bool {
        let match_result = self
            .scanners
            .values()
            .par_bridge()
            .find_map_any(|(pos, space)| {
                space
                    .find_match(&scanner.beacons)
                    .map(|result| (*pos, result.0, result.1))
            });

        if let Some((pos, rotation, relative_scanner_position)) = match_result {
            self.scanners.insert(
                scanner.id,
                (
                    pos + relative_scanner_position,
                    scanner.beacons[rotation].clone(),
                ),
            );
            true
        } else {
            false
        }
    }

    pub fn different_beacons(&self) -> usize {
        let mut set = BeaconSet::default();
        for (pos, space) in self.scanners.values() {
            set.add_space(*pos, space);
        }
        set.0.len()
    }

    pub fn max_scanner_distance(&self) -> usize {
        self.scanners
            .values()
            .map(|(a, _)| {
                self.scanners
                    .values()
                    .map(|(b, _)| Vector3::manhattan(a, b))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

fn is_directional_match(
    space: &BeaconSet,
    beacon_rotations: &BeaconSet,
) -> Option<DirectionMatchInfo> {
    for source_beacon in &space.0 {
        for matched_beacon in &beacon_rotations.0 {
            let offset = *source_beacon - *matched_beacon;
            let matching = beacon_rotations
                .0
                .iter()
                .filter(|x| space.0.contains(&(**x + offset)))
                .take(MATCHING_SIZE)
                .count();

            if matching >= MATCHING_SIZE {
                return Some(DirectionMatchInfo {
                    source_beacon: *source_beacon,
                    target_beacon: *matched_beacon,
                });
            }
        }
    }
    None
}

fn all_rotations(a: Vector3) -> Vec<Vector3> {
    let b = rotate_x_clockwise(a);
    let c = rotate_x_clockwise(b);
    let d = rotate_x_clockwise(c);
    let e = rotate_y_clockwise(a);
    let f = rotate_y_clockwise(e);
    let g = rotate_y_clockwise(f);

    vec![a, b, c, d, e, g]
        .into_iter()
        .map(z_rotations)
        .flatten()
        .collect()
}

// n, 1, 2
// n, 2, -1
fn rotate_x_clockwise(a: Vector3) -> Vector3 {
    Vector3::new([a[0], a[2], -a[1]])
}

// 1, n, 2
// 2, n, -1
fn rotate_y_clockwise(a: Vector3) -> Vector3 {
    Vector3::new([a[2], a[1], -a[0]])
}

// 1, 2, n
// 2, -1, n
fn rotate_z_clockwise(a: Vector3) -> Vector3 {
    Vector3::new([a[1], -a[0], a[2]])
}

fn z_rotations(a: Vector3) -> [Vector3; 4] {
    let b = rotate_z_clockwise(a);
    let c = rotate_z_clockwise(b);
    let d = rotate_z_clockwise(c);
    [a, b, c, d]
}

fn parse_input(input: &str) -> Input {
    let mut scanners = Vec::new();
    let mut lines = input.lines();
    let mut counter = 0;

    while let Some(_) = lines.next() {
        let beacons = lines
            .by_ref()
            .take_while(|x| !x.is_empty())
            .map(|line| {
                let mut split = line.split(',');
                let x = split.next().and_then(|a| a.parse::<isize>().ok()).unwrap();
                let y = split.next().and_then(|a| a.parse::<isize>().ok()).unwrap();
                let z = split.next().and_then(|a| a.parse::<isize>().ok()).unwrap();

                Vector3::new([x, y, z])
            })
            .collect::<Vec<_>>();

        let mut rekt = vec![BeaconSet::default(); 24];

        for beacon in beacons {
            let rotations = all_rotations(beacon);
            for rotation in 0..24 {
                rekt[rotation].0.insert(rotations[rotation]);
            }
        }

        scanners.push(Scanner {
            id: counter,
            beacons: rekt,
        });

        counter += 1;
    }

    scanners
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use aoc_2021::vector::Vector3;

    use crate::all_rotations;

    #[test]
    fn d19_test_rotations() {
        let rots = all_rotations(Vector3::new([1, 2, 3]))
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(24, rots.len());
    }
}
