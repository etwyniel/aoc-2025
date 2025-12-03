#[cfg(use_c)]
use std::ffi::CString;

use aoc_framework::*;
#[cfg(use_c)]
use libc::{c_char, c_int};

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

#[cfg(use_c)]
unsafe extern "C" {
    fn day01_part1(input: *const c_char) -> c_int;
    fn day01_part2(input: *const c_char) -> c_int;
}

#[cfg(not(use_c))]
fn parse_ln(ln: &[u8]) -> i64 {
    let Some((c, dist)) = ln.split_at_checked(1) else {
        return 0;
    };
    let sign = if c == b"L" { -1 } else { 1 };
    sign * dist
        .iter()
        .fold(0, |acc, b| acc * 10 + i64::from(b - b'0'))
}

#[cfg(not(use_c))]
fn parse(input: &[u8]) -> impl Iterator<Item = i64> {
    input.split(|&b| b == b'\n').map(parse_ln)
}

#[cfg(use_c)]
#[aoc(part = 1, example = 3)]
fn part1(input: &str) -> u64 {
    let input_c = CString::new(input).unwrap();
    let res = unsafe { day01_part1(input_c.as_ptr()) };
    res as u64
}

#[cfg(not(use_c))]
#[aoc(part = 1, example = 3)]
fn part1(input: &str) -> u64 {
    let mut angle: i64 = 50;
    parse(input.as_bytes())
        .filter(|&dist| {
            angle = (angle + dist).rem_euclid(100);
            angle == 0
        })
        .count() as u64
}

#[cfg(use_c)]
#[aoc(part = 2, example = 6)]
fn part2(input: &str) -> u64 {
    let input_c = CString::new(input).unwrap();
    let res = unsafe { day01_part2(input_c.as_ptr()) };
    res as u64
}

#[cfg(not(use_c))]
#[aoc(part = 2, example = 6)]
fn part2(input: &str) -> u64 {
    let mut angle: i64 = 50;
    parse(input.as_bytes())
        .map(|dist| {
            let start = angle;
            angle = (angle + dist).rem_euclid(100);
            (dist / 100).unsigned_abs() + u64::from(start != 0 && !(1..100).contains(&(start + dist % 100)))
        })
        .sum::<u64>()
}
