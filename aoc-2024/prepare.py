import requests
import os
import sys

# from dotenv import load_dotenv
from os.path import exists
from datetime import datetime, time

# load_dotenv()

year = 2024

if len(sys.argv) > 1:
    day = sys.argv[1]
else:
    print("Enter day:")
    day = input()

url = "https://adventofcode.com/{year}/day/{day}/input".format(year=year, day=day)

example_path = "examples/day_{}.rs".format(day.zfill(2))
input_path = "input/day_{}.txt".format(day.zfill(2))


if not exists(example_path):
    print("created: " + example_path)
    file = open(example_path, "x")
    rust = """use aoc_2024::stopwatch;

type Input = Vec<u32>;

pub fn main() {
    let numbers = input(include_str!("../input/day_{day}.txt"));
    // let numbers = input(include_str!("../input/test.txt"));

    let time = stopwatch(|| {
        println!("Exercise 1: {}", exercise_1(&numbers));
        // println!("Exercise 2: {}", exercise_2(&numbers));
    });

    println!("time: {:?}", time);
}

fn input(input: &str) -> Input {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> usize {

}

fn exercise_1(input: &Input) -> usize {
    unimplemented!()
}
fn exercise_2(input: &Input) -> usize {
    unimplemented!()    
}""".replace(
        "{day}", day.zfill(2)
    )
    file.write(rust)
    file.close()


if not exists(input_path):
    # cookies = {'session': os.environ["COOKIE"]}
    # headers = {'User-Agent': 'https://github.com/BlockCat/advent_of_code_2024 by BlockCat'}
    # response = requests.get(url = url, cookies=cookies, headers=headers)

    # if not response.text.__contains__("Please don't repeatedly request this endpoint before it unlocks! The calendar countdown is synchronized with the server "):
    print("created:" + input_path)
    file = open(input_path, "x")
    # file.write(response.text)
    file.close()
