use std::{collections::BTreeSet, ops::RangeInclusive};

use aoc_framework::bcd::Bcd;
use aoc_framework::*;

pub struct Day02;

impl_day!(Day02::{part1, part2}: 2025[2], r"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
");

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<Bcd>> {
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

fn prefix(start: Bcd, end: Bcd) -> Bcd {
    for i in (0..u64::BITS / 4).rev().skip(1) {
        if start >> i != end >> i {
            return start >> (i + 1);
        }
    }
    start
}

fn solve<const PART2: bool>(input: &str) -> u64 {
    parse(input)
        .map(|range| {
            let start_len = range.start().len();
            let end_len = range.end().len();
            (start_len..=end_len)
                .map(|len| {
                    let check_range = (Bcd::new(1) << (len - 1)).max(*range.start())
                        ..=(Bcd::new(9).repeat(len).min(*range.end()));
                    (len, check_range)
                })
                .map(|(len, check_range)| {
                    let mut ids = BTreeSet::<u64>::new();
                    let start = *check_range.start();
                    let end = *check_range.end();
                    let start_bin: u64 = start.into();
                    let end_bin: u64 = end.into();

                    let prefix = prefix(start, end);
                    let prefix_len = prefix.len();

                    let first_size = if PART2 { 1 } else { len / 2 };
                    for size in first_size..=(len / 2) {
                        if !(PART2 || len % 2 == 0) || len % size != 0 {
                            continue;
                        }

                        let mask: u64 = Bcd::new(1).repeat_len(size, len / size).into();

                        let remaining_digits = size.saturating_sub(prefix_len);
                        let prefix_part: u64 = if prefix_len > size {
                            prefix >> (prefix_len - size)
                        } else {
                            prefix << remaining_digits
                        }
                        .into();
                        for n in 0..(10u32.pow(remaining_digits)) {
                            let candidate = (prefix_part + n as u64) * mask;
                            if candidate < start_bin {
                                continue;
                            }
                            if candidate <= end_bin {
                                ids.insert(candidate);
                            } else {
                                break;
                            }
                        }
                    }
                    ids.into_iter().sum::<u64>()
                })
                .sum::<u64>()
        })
        .sum()
}

#[aoc(part = 1, example = 1227775554)]
fn part1(input: &str) -> u64 {
    solve::<false>(input)
}

#[aoc(part = 2, example = 4174379265)]
fn part2(input: &str) -> u64 {
    solve::<true>(input)
}
