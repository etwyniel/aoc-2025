use aoc_framework::*;

pub struct Day01;

impl_day!(Day01::{part1, part2}: 2025[1], r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
");

fn parse_ln(ln: &str) -> i64 {
    let Some((c, dist)) = ln.split_at_checked(1) else {
        return 0;
    };
    let sign = if c == "L" { -1 } else { 1 };
    sign * dist.parse::<i64>().unwrap_or_default()
}

fn parse(input: &str) -> impl Iterator<Item = i64> {
    input.lines().map(parse_ln)
}

#[aoc(part = 1, example = 3)]
fn part1(input: &str) -> u64 {
    let mut angle: i64 = 50;
    parse(input)
        .filter(|&dist| {
            angle = (angle + dist).rem_euclid(100);
            angle == 0
        })
        .count() as u64
}

#[aoc(part = 2, example = 6)]
fn part2(input: &str) -> u64 {
    let mut angle: i64 = 50;
    parse(input)
        .map(|dist| {
            let start = angle;
            angle = (angle + dist).rem_euclid(100);
            (dist / 100).abs() + (start != 0 && !(1..100).contains(&(start + dist % 100))) as i64
        })
        .sum::<i64>() as u64
}
