use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

pub fn main() {
    let input = parse(include_str!("../input/day_05_big.txt"));
    println!("Exercise 1: {}", exercise_1(&input));
    println!("Exercise 2: {}", exercise_2(&input));
}

fn parse<'a>(input: &'a str) -> InputType {
    let mut lines = input.lines();

    let seeds = lines.next().unwrap()[7..]
        .split_whitespace()
        .map(|s| (s).parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut category = Vec::new();

    let mut lines = lines.skip(1);

    while let Some(_) = lines.next() {
        let mut ce = lines
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .take(3)
                    .map(|s| s.parse::<usize>().expect("Could not parse"))
                    .collect::<Vec<usize>>()
            })
            .map(|numbers| Mapping {
                source: numbers[1],
                offset: numbers[0] as isize - numbers[1] as isize,
                range: numbers[2],
            })
            .collect::<Vec<_>>();

        ce.sort_by_key(|s| s.source);

        category.push(ce);
    }

    InputType { seeds, category }
}

fn exercise_1(input: &InputType) -> usize {
    input
        .seeds
        .iter()
        .map(|seed| input.category.iter().fold(*seed, map_to_target))
        .min()
        .unwrap()
}

fn map_to_target(number: usize, mappings: &Vec<Mapping>) -> usize {
    mappings
        .iter()
        .find(|m| m.contains(number))
        .map(|f| f.offset(number))
        .unwrap_or(number)
}

fn exercise_2(input: &InputType) -> usize {
    input
        .seeds
        .chunks_exact(2)
        .par_bridge()
        .map(|chunk| Mapping {
            source: chunk[0],
            range: chunk[1],
            offset: 0,
        })
        .map(|mapping| {
            input
                .category
                .iter()
                .fold(vec![mapping], |mut acc, mappings| {
                    acc.sort_by_key(|s| s.source);
                    acc.into_par_iter()
                        .flat_map(|map| overlapping(map, mappings))
                        .collect()
                })
                .into_iter()
                .map(|x| x.start_offset())
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn overlapping(base_mapping: Mapping, mappings: &Vec<Mapping>) -> Vec<Mapping> {
    let mut result = Vec::new();

    let low = base_mapping.start_offset();
    let up = base_mapping.end_offset();

    let mut last_range = 0;

    for mapping in mappings
        .iter()
        .skip_while(|mapping| mapping.end() < low)
        .take_while(|mapping| mapping.start() < up)
    {
        if mapping.source >= last_range {
            if let Some(overlap) = get_overlap(
                low,
                up,
                Mapping::new(last_range, 0, mapping.source - last_range),
            ) {
                result.push(overlap);
            }
        }

        if let Some(overlap) = get_overlap(low, up, mapping.clone()) {
            result.push(Mapping {
                offset: mapping.offset,
                ..overlap
            });
        }

        last_range = mapping.end();
    }

    if last_range < up {
        if let Some(overlap) = get_overlap(low, up, Mapping::new(last_range, 0, up - last_range)) {
            result.push(overlap);
        }
    }
    result
}

fn get_overlap(low: usize, up: usize, mapping: Mapping) -> Option<Mapping> {
    if mapping.start() > up || mapping.end() < low {
        return None;
    }

    let up = up.min(mapping.end());
    let low = low.max(mapping.start());

    if up == low {
        return None;
    }
    return Some(Mapping {
        source: low,
        offset: mapping.offset,
        range: up - low,
    });
}

struct InputType {
    seeds: Vec<usize>,
    category: Vec<Vec<Mapping>>,
}

#[derive(Debug, Clone)]
struct Mapping {
    source: usize,
    offset: isize,
    range: usize,
}

impl Mapping {
    fn new(source: usize, offset: isize, range: usize) -> Self {
        Self {
            source,
            offset,
            range,
        }
    }
    fn contains(&self, number: usize) -> bool {
        (self.source..(self.source + self.range)).contains(&number)
    }

    fn offset(&self, number: usize) -> usize {
        (number as isize + self.offset) as usize
    }

    fn start(&self) -> usize {
        self.source
    }

    fn end(&self) -> usize {
        self.source + self.range
    }

    fn start_offset(&self) -> usize {
        self.offset(self.source)
    }

    fn end_offset(&self) -> usize {
        self.offset(self.source + self.range)
    }
}
