use aoc_2024::Runner;
use regex::Regex;

#[derive(Debug, Default)]
pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day3 {
    fn parse(&mut self) {}

    fn part1(&mut self) {
        println!("Day 3:");
        let input: &str = include_str!("../input/day3/day3.txt");
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut sum = 0;
        for (_, [a, b]) in re.captures_iter(input).map(|cap| cap.extract()) {
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();
            sum += a * b;
        }

        println!("\tPart 1: {sum}");
    }

    fn part2(&mut self) {
        let input: &str = include_str!("../input/day3/day3.txt");
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        let mut sum = 0;
        let mut add_up = true;
        for m in re.captures_iter(input) {
            let instruction = m.get(0).unwrap().as_str();

            match instruction {
                "do()" => add_up = true,
                "don't()" => add_up = false,
                _ => {
                    if add_up {
                        let a: u64 = m.get(1).unwrap().as_str().parse().unwrap();
                        let b: u64 = m.get(2).unwrap().as_str().parse().unwrap();
                        sum += a * b;
                    }
                }
            }
        }

        println!("\tPart 2: {sum}");
    }
}
