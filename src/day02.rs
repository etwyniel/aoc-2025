use std::ops::RangeInclusive;
use rayon::prelude::*;

use aoc_framework::*;

pub struct Day02;

impl_day!(Day02::{part1, part2}: 2025[2], r"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
");

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input
        .split(',')
        .filter_map(|range| {
            range
                .trim()
                .split('-')
                .filter_map(|s| s.parse().ok())
                .tuples()
                .next()
        })
        .map(|(start, end)| start..=end)
}

fn decimal_len(mut n: u64) -> u32 {
    let mut len = 0;
    while n > 0 {
        len += 1;
        n /= 10;
    }
    len
}

#[aoc(part = 1, example = 1227775554)]
fn part1(input: &str) -> u64 {
    parse(input)
        .collect::<Vec<_>>().into_par_iter()
        .map(|range| {
            let start_len = decimal_len(*range.start());
            let end_len = decimal_len(*range.end());
            (start_len..=end_len)
                .filter(|len| len % 2 == 0)
                .flat_map(|len| {
                    let check_range = (10u64.pow(len - 1).max(*range.start()))
                        ..(10u64.pow(len).min(*range.end() + 1));
                    check_range.map(move |n| (len, n))
                })
                .filter(|&(len, n)| {
                    let mask = 10u64.pow(len / 2);
                    let half = n % mask;
                    n == half + half * mask
                })
                .map(|(_, n)| n)
                .sum::<u64>()
        })
        .sum()
}

#[aoc(part = 2, example = 4174379265)]
fn part2(input: &str) -> u64 {
    parse(input)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|range| {
            let start_len = decimal_len(*range.start());
            let end_len = decimal_len(*range.end());
            (start_len..=end_len)
                .flat_map(|len| {
                    let check_range = (10u64.pow(len - 1).max(*range.start()))
                        ..(10u64.pow(len).min(*range.end() + 1));
                    check_range.map(move |n| (len, n))
                })
                .filter(|&(len, n)| {
                    for hl in 1..=(len / 2) {
                        if len % hl != 0 {
                            continue;
                        }
                        let mask = 10u64.pow(hl);
                        let part = n % mask;
                        let mut res = 0;
                        for _ in 0..(len / hl) {
                            res = res * mask + part;
                        }
                        if n == res {
                            return true;
                        }
                    }
                    false
                })
                .map(|(_, n)| n)
                .sum::<u64>()
        })
        .sum()
}
