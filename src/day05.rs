use std::{cmp::Ordering, collections::VecDeque, ops::RangeInclusive};

use aoc_framework::*;

pub struct Day05;

impl_day!(Day05::{part1, part2}: 2025[5], r"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
");

fn parse(input: impl Iterator<Item = String>) -> VecDeque<RangeInclusive<u64>> {
    let range_it = input.take_while(|ln| !ln.is_empty()).map(|ln| {
        ln.split('-')
            .flat_map(|s| s.parse::<u64>().ok())
            .tuples()
            .next()
            .unwrap()
    });
    let mut ranges: VecDeque<RangeInclusive<u64>> = VecDeque::new();
    for (mut start, mut end) in range_it {
        match ranges.binary_search_by_key(&start, |range| *range.end() + 1) {
            Ok(ndx) => ranges[ndx] = *ranges[ndx].start()..=end,
            Err(ndx) => {
                ranges.insert(ndx, start..=end);
                while let Some(next) = ranges
                    .get(ndx + 1)
                    .filter(|next| *next.start() <= (end + 1))
                {
                    start = start.min(*next.start());
                    end = end.max(*next.end());
                    ranges[ndx] = start..=end;
                    ranges.remove(ndx + 1);
                }
            }
        }
    }
    ranges
}

#[aoc(part = 1, example = 3)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let ranges = parse(&mut input);
    input
        .map(|ln| ln.trim().parse::<u64>().unwrap())
        .filter(|num| {
            ranges
                .binary_search_by(|range| {
                    if range.contains(&num) {
                        Ordering::Equal
                    } else {
                        range.start().cmp(&num)
                    }
                })
                .is_ok()
        })
        .count() as u64
}

#[aoc(part = 2, example = 14)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    parse(input)
        .into_iter()
        .map(|range| range.size_hint().0 as u64)
        .sum()
}
