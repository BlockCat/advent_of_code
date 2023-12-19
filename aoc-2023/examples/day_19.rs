use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;

type InputType = (HashMap<String, Workflow>, Vec<Score>);

// 130273941158780
// 130262715574114
pub fn main() {
    let input = parse(include_str!("../input/day_19.txt"));

    println!("Exercise 1: {}", exercise_1(input.clone()));
    println!("Exercise 2: {}", exercise_2(input));
}

#[test]
fn test() {
    let input = parse(include_str!("../input/test.txt"));

    let res = exercise_1(input.clone());
    println!("Exercise 1: {}", res);
    assert_eq!(res, 19114);

    let res = exercise_2(input);
    println!("Exercise 2: {}", res);
    assert_eq!(res, 167409079868000);
}

fn parse<'a>(input: &'a str) -> InputType {
    let mut lines = input.lines();

    let workflow = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(parse_workflow)
        .map(|s| (s.name.clone(), s))
        .collect();
    let scores = lines.map(parse_line).collect::<Vec<_>>();

    (workflow, scores)
}

fn parse_workflow(line: &str) -> Workflow {
    let pos = line.find("{").unwrap();
    let name = &line[0..pos];
    let l = line[pos..].replace("{", "").replace("}", "");
    let l = l.split(",");

    let steps = l
        .map(|s| {
            if let Some((le, name)) = s.split_once(":") {
                let match_part = &le[0..1];
                let instruction = &le[1..2];
                let rest = le[2..].parse::<usize>().unwrap();

                let match_part = match match_part {
                    "x" => ScoreStep::Cool,
                    "m" => ScoreStep::Musical,
                    "a" => ScoreStep::Aero,
                    "s" => ScoreStep::Shiny,
                    _ => unreachable!(),
                };
                let goto = match name {
                    "A" => Move::Accept,
                    "R" => Move::Reject,
                    _ => Move::Location(name.to_string()),
                };
                match instruction {
                    ">" => WorkflowStep::Gt(match_part, rest, goto),
                    "<" => WorkflowStep::Lt(match_part, rest, goto),
                    _ => unreachable!(),
                }
            } else {
                match s {
                    "A" => WorkflowStep::Accept,
                    "R" => WorkflowStep::Reject,
                    _ => WorkflowStep::Goto(s.to_string()),
                }
            }
        })
        .collect::<Vec<_>>();

    Workflow {
        name: name.to_string(),
        steps,
    }
}

fn parse_line(line: &str) -> Score {
    let mut l = line.split(",");

    let cool = l
        .next()
        .unwrap()
        .replace("{x=", "")
        .parse::<usize>()
        .unwrap();
    let musical = l
        .next()
        .unwrap()
        .replace("m=", "")
        .parse::<usize>()
        .unwrap();
    let aero = l
        .next()
        .unwrap()
        .replace("a=", "")
        .parse::<usize>()
        .unwrap();
    let shiny = l
        .next()
        .unwrap()
        .replace("s=", "")
        .replace("}", "")
        .parse::<usize>()
        .unwrap();

    [cool, musical, aero, shiny]
}

fn exercise_1(input: InputType) -> usize {
    input
        .1
        .par_iter()
        .filter(|s| is_accepted(s, &input.0, "in"))
        .map(|s| s.iter().sum::<usize>())
        .sum()
}

fn exercise_2(input: InputType) -> usize {
    traversal(&input.0, "in", [(1, 4001); 4])
}

fn combinations(ranges: &[(usize, usize); 4]) -> usize {
    ranges.iter().map(|(a, b)| b - a).product()
}

fn traversal(
    map: &HashMap<String, Workflow>,
    part: &str,
    mut ranges: [(usize, usize); 4],
) -> usize {
    let wf = &map[part];

    let mut counter = 0;

    for step in &wf.steps {
        match step {
            WorkflowStep::Accept => {
                return counter + combinations(&ranges);
            }
            WorkflowStep::Reject => return counter,
            WorkflowStep::Goto(part) => return counter + traversal(map, part, ranges),
            WorkflowStep::Gt(ss, n, m) => {
                let ss_index = ss.index();
                let (lower, upper) = ranges[ss_index];

                let n = n + 1;

                if lower > n {
                    return counter
                        + match m {
                            Move::Accept => combinations(&ranges),
                            Move::Reject => 0,
                            Move::Location(part) => traversal(map, part, ranges),
                        };
                } else if upper < n {
                    continue;
                } else {
                    let new_lower_range = (ranges[ss_index].0, n);
                    let new_upper_range = (n, ranges[ss_index].1);

                    let mut next_ranges = ranges.clone();
                    next_ranges[ss_index] = new_upper_range;
                    ranges[ss_index] = new_lower_range;
                    counter += match m {
                        Move::Accept => combinations(&next_ranges),
                        Move::Reject => 0,
                        Move::Location(part) => traversal(map, part, next_ranges),
                    };
                }
            }
            WorkflowStep::Lt(ss, n, m) => {
                let ss_index = ss.index();
                let (lower, upper) = &ranges[ss_index];

                if lower > n {
                    continue;
                } else if upper <= n {
                    return counter
                        + match m {
                            Move::Accept => combinations(&ranges),
                            Move::Reject => 0,
                            Move::Location(part) => traversal(map, part, ranges),
                        };
                } else {
                    let new_lower_range = (ranges[ss_index].0, *n);
                    let new_upper_range = (*n, ranges[ss_index].1);

                    let mut next_ranges = ranges.clone();
                    next_ranges[ss_index] = new_lower_range;
                    ranges[ss_index] = new_upper_range;

                    counter += match m {
                        Move::Accept => combinations(&next_ranges),
                        Move::Reject => 0,
                        Move::Location(part) => traversal(map, part, next_ranges),
                    };
                }
            }
        }
    }

    unreachable!()
}

fn is_accepted(score: &Score, map: &HashMap<String, Workflow>, current: &str) -> bool {
    let cw = map.get(current).unwrap();

    for step in &cw.steps {
        match step {
            WorkflowStep::Accept => return true,
            WorkflowStep::Reject => return false,
            WorkflowStep::Goto(wf) => return is_accepted(score, map, wf),
            WorkflowStep::Gt(r, s, m) => {
                let number = score[r.index()];
                if number > *s {
                    return match m {
                        Move::Accept => true,
                        Move::Reject => false,
                        Move::Location(wf) => is_accepted(score, map, wf),
                    };
                }
            }
            WorkflowStep::Lt(r, s, m) => {
                let number = score[r.index()];
                if number < *s {
                    return match m {
                        Move::Accept => true,
                        Move::Reject => false,
                        Move::Location(wf) => is_accepted(score, map, wf),
                    };
                }
            }
        }
    }

    unreachable!()
}

type Score = [usize; 4];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Workflow {
    name: String,
    steps: Vec<WorkflowStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum WorkflowStep {
    Accept,
    Reject,
    Goto(String),
    Gt(ScoreStep, usize, Move),
    Lt(ScoreStep, usize, Move),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Move {
    Accept,
    Reject,
    Location(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ScoreStep {
    Cool = 0,
    Musical,
    Aero,
    Shiny,
}

impl ScoreStep {
    fn index(&self) -> usize {
        match self {
            ScoreStep::Cool => 0,
            ScoreStep::Musical => 1,
            ScoreStep::Aero => 2,
            ScoreStep::Shiny => 3,
        }
    }
}
