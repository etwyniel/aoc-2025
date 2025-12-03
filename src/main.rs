use aoc_framework::Day;
use std::env::args;

mod day01;
mod day02;
mod day03;

fn main() {
    let days = [day01::Day01::run, day02::Day02::run, day03::Day03::run];

    let token = std::env::var("AOC_TOKEN").ok();

    if let Some(day) = args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .and_then(|day| days.get(day - 1))
    {
        day(token.as_deref());
        return;
    }

    for day in days {
        day(token.as_deref());
    }
}
