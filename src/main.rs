use aoc_2024::Runner;

/*
  Call function to run the all AOC exercises.
  The function should automaticlly determine the day and read the input file.
  Then it should print the results for all parts.
*/
mod day1;
mod day2;
mod day3;

fn main() {
    day1::Day1::run(&mut day1::Day1::new());
    day2::Day2::run(&mut day2::Day2::new());
    day3::Day3::run(&mut day3::Day3::new());
}
