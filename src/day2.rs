use aoc_2024::Runner;

#[derive(Debug, Default)]
pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day2 {
    fn parse(&mut self) {
        self.input = std::fs::read_to_string("input/day2/day2.txt").unwrap();
    }

    fn part1(&mut self) {
        println!(
            "Part 1: {}",
            self.input
                .lines()
                .map(|line| {
                    let nums: Vec<i32> = line
                        .split_whitespace()
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();
                    safe1(&nums)
                })
                .sum::<u32>()
        );
    }

    fn part2(&mut self) {
        print!(
            "Part 2: {}",
            self.input
                .lines()
                .map(|line| {
                    let nums: Vec<i32> = line
                        .split_whitespace()
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect();
                    safe2(&nums)
                })
                .sum::<u32>()
        );
    }
}

fn safe1(nums: &[i32]) -> u32 {
    let ascending = nums[0] < nums[1];

    let mut prev = nums[0];
    for &n in &nums[1..] {
        let diff = (prev - n).abs();

        let no_diff = diff < 1;
        let too_big_gap = diff > 3;
        let wrong_direction = (ascending && prev > n) || (!ascending && prev < n);

        if no_diff || too_big_gap || wrong_direction {
            return 0;
        }
        prev = n;
    }
    1
}

fn safe2(nums: &[i32]) -> u32 {
    if safe1(nums) == 1 {
        return 1;
    }

    for skip_idx in 0..nums.len() {
        let subset: Vec<i32> = nums
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &n)| n)
            .collect();

        if safe1(&subset) == 1 {
            return 1;
        }
    }
    0
}
