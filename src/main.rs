use aoc_framework::Checker;
use std::env::args;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let token = std::env::var("AOC_TOKEN").ok();
    Checker::new(token, &args().nth(1).unwrap_or_default())
        .unwrap()
        .run::<day01::Day01>()
        .run::<day02::Day02>()
        .run::<day03::Day03>()
        .run::<day04::Day04>()
        .run::<day05::Day05>()
        .run::<day06::Day06>()
        .run::<day07::Day07>();
}
