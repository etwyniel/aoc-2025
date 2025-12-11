use aoc_framework::*;

pub struct Day06;

impl_day!(Day06::{part1, part2}: 2025[6], r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
");

fn parse_line(ln: &str) -> impl Iterator<Item = u64> {
    ln.split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
}

#[aoc(part = 1, example = 4277556)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let mut totals = parse_line(&input.next().unwrap())
        .map(|n| (n, n))
        .collect_vec();
    for ln in input {
        if !ln.trim().as_bytes()[0].is_ascii_digit() {
            return ln
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, op)| {
                    let (sum, prod) = totals[i];
                    if op == "+" { sum } else { prod }
                })
                .sum();
        }
        parse_line(&ln).enumerate().for_each(|(i, d)| {
            totals[i].0 += d;
            totals[i].1 *= d;
        });
    }
    0
}

#[aoc(part = 2, example = 3263827)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut nums: Vec<u64> = Vec::new();
    for ln in input {
        if !ln.trim().as_bytes()[0].is_ascii_digit() {
            return ln
                .bytes()
                .enumerate()
                .filter(|(_, b)| !b.is_ascii_whitespace())
                .map(|(i, op)| {
                    let numbers = nums[i..].iter().take_while(|&&n| n > 0);
                    if op == b'+' {
                        numbers.sum()
                    } else {
                        numbers.product::<u64>()
                    }
                })
                .sum();
        }
        for (i, b) in ln.bytes().enumerate() {
            if b == b' ' {
                continue;
            }

            while nums.len() <= i {
                nums.push(0);
            }
            let number = &mut nums[i];
            *number = *number * 10 + (b - b'0') as u64;
        }
    }
    0
}
