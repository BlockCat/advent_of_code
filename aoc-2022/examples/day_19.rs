use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

type InputType = Vec<Blueprint>;

pub fn main() {
    let numbers = input();

    println!("Exercise 1: {}", exercise_1(numbers.clone()));
    println!("Exercise 2: {}", exercise_2(numbers));
}

fn input() -> InputType {
    include_str!("../input/day_19.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Blueprint {
    let mut parts = line.split(' ');

    let id = parts.nth(1).unwrap().replace(":", "").parse().unwrap();
    let ore_robot = parts.nth(4).unwrap().parse().unwrap();
    let clay_robot = parts.nth(5).unwrap().parse().unwrap();
    let obsidian_robot_ore = parts.nth(5).unwrap().parse().unwrap();
    let obsidian_robot_clay = parts.nth(2).unwrap().parse().unwrap();
    let geode_robot_ore = parts.nth(5).unwrap().parse().unwrap();
    let geode_robot_obsidian = parts.nth(2).unwrap().parse().unwrap();

    Blueprint {
        id,
        ore_robot,
        clay_robot,
        obsidian_robot_ore,
        obsidian_robot_clay,
        geode_robot_ore,
        geode_robot_obsidian,
    }
}

fn exercise_1(input: InputType) -> usize {
    input
        .into_par_iter()
        .map(|bp| bp.id * blue_print_quality(bp, 24))
        .sum()
}
fn exercise_2(input: InputType) -> usize {
    input
        .into_par_iter()
        .take(3)
        .map(|bp| blue_print_quality(bp, 32))
        .product()
}

fn blue_print_quality(bp: Blueprint, time: u8) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back(QueueEntry::new(1, 0, 0, 0, time, 0, 0, 0, 0));

    let mut max = 0;

    let mut visited: HashSet<QueueEntry> = HashSet::new();

    let max_ore_cost = bp
        .ore_robot
        .max(bp.clay_robot)
        .max(bp.obsidian_robot_ore)
        .max(bp.geode_robot_ore);

    let max_clay_cost = bp.obsidian_robot_clay;

    let max_obs_cost = bp.geode_robot_obsidian;

    while let Some(entry) = queue.pop_back() {
        if entry.time == 0 {
            max = max.max(entry.geode);
            continue;
        }

        if !visited.insert(entry.clone()) {
            continue;
        }

        if entry.ore >= bp.geode_robot_ore && entry.obsidian >= bp.geode_robot_obsidian {
            queue.push_back(
                entry
                    .clone()
                    .tick()
                    .buy_geode(bp.geode_robot_ore, bp.geode_robot_obsidian),
            );
        } else if entry.ore >= bp.obsidian_robot_ore
            && entry.clay >= bp.obsidian_robot_clay
            && entry.obsidian_collection < max_obs_cost
        {
            queue.push_back(
                entry
                    .clone()
                    .tick()
                    .buy_obsidian(bp.obsidian_robot_ore, bp.obsidian_robot_clay),
            );
        } else {
            if entry.ore >= bp.ore_robot && entry.ore_collection < max_ore_cost {
                queue.push_back(entry.clone().tick().buy_ore(bp.ore_robot));
            }

            if entry.ore >= bp.clay_robot && entry.clay_collection < max_clay_cost {
                queue.push_back(entry.clone().tick().buy_clay(bp.clay_robot));
            }

            if entry.ore < bp.ore_robot || entry.ore < bp.clay_robot {
                queue.push_back(entry.clone().tick());
            }
        }
    }

    max as usize
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
struct QueueEntry {
    geode_collection: u16,
    ore_collection: u16,
    clay_collection: u16,
    obsidian_collection: u16,

    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    time: u8,
}

impl QueueEntry {
    pub fn new(
        ore_collection: u16,
        clay_collection: u16,
        obsidian_collection: u16,
        geode_collection: u16,
        t: u8,
        ore: u16,
        clay: u16,
        obsidian: u16,
        geode: u16,
    ) -> Self {
        Self {
            ore_collection,
            clay_collection,
            obsidian_collection,
            geode_collection,
            ore,
            clay,
            obsidian,
            geode,
            time: t,
        }
    }

    pub fn tick(self) -> Self {
        Self {
            ore: self.ore + self.ore_collection,
            clay: self.clay + self.clay_collection,
            obsidian: self.obsidian + self.obsidian_collection,
            geode: self.geode + self.geode_collection,

            ore_collection: self.ore_collection,
            clay_collection: self.clay_collection,
            geode_collection: self.geode_collection,
            obsidian_collection: self.obsidian_collection,

            time: self.time - 1,
        }
    }

    pub fn buy_ore(self, ore_cost: u16) -> Self {
        Self {
            ore: self.ore - ore_cost,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,

            ore_collection: self.ore_collection + 1,
            clay_collection: self.clay_collection,
            geode_collection: self.geode_collection,
            obsidian_collection: self.obsidian_collection,

            time: self.time,
        }
    }

    pub fn buy_clay(self, ore_cost: u16) -> Self {
        Self {
            ore: self.ore - ore_cost,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,

            ore_collection: self.ore_collection,
            clay_collection: self.clay_collection + 1,
            geode_collection: self.geode_collection,
            obsidian_collection: self.obsidian_collection,

            time: self.time,
        }
    }

    pub fn buy_obsidian(self, ore_cost: u16, clay_cost: u16) -> Self {
        Self {
            ore: self.ore - ore_cost,
            clay: self.clay - clay_cost,
            obsidian: self.obsidian,
            geode: self.geode,

            ore_collection: self.ore_collection,
            clay_collection: self.clay_collection,
            geode_collection: self.geode_collection,
            obsidian_collection: self.obsidian_collection + 1,

            time: self.time,
        }
    }

    pub fn buy_geode(self, ore_cost: u16, obsidian_cost: u16) -> Self {
        Self {
            ore: self.ore - ore_cost,
            clay: self.clay,
            obsidian: self.obsidian - obsidian_cost,
            geode: self.geode,

            ore_collection: self.ore_collection,
            clay_collection: self.clay_collection,
            geode_collection: self.geode_collection + 1,
            obsidian_collection: self.obsidian_collection,

            time: self.time,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: usize,
    ore_robot: u16,
    clay_robot: u16,
    obsidian_robot_ore: u16,
    obsidian_robot_clay: u16,
    geode_robot_ore: u16,
    geode_robot_obsidian: u16,
}
