use aoc_2024::Runner;

#[derive(Debug, Default)]
pub struct Day1 {
    list1: Vec<i64>,
    list2: Vec<i64>,
}

impl Day1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day1 {
    fn parse(&mut self) {
        let lines = aoc_2024::read_lines("input/day1/day1.txt");

        for line in lines {
            let mut entry = line.split("   ");
            self.list1
                .push(entry.next().unwrap().parse::<i64>().unwrap());
            self.list2
                .push(entry.next().unwrap().parse::<i64>().unwrap());
        }
    }

    fn part1(&mut self) {
        println!("Day 1:");
        self.list1.sort();
        self.list2.sort();
        let sum: i64 = self
            .list1
            .iter()
            .zip(self.list2.clone())
            .map(|(left, right)| (left - right).abs())
            .sum::<i64>();
        println!("\tPart 1: {sum}");
    }

    fn part2(&mut self) {
        self.list1.sort();
        self.list2.sort();
        let sum: i64 = self
            .list1
            .iter()
            .map(|left| {
                let count = self.list2.iter().filter(|right| *right == left).count() as i64;
                left * count
            })
            .sum::<i64>();
        println!("\tPart 2: {sum}");
    }
}
