use aoc_framework::*;

pub struct Day03;

impl_day!(Day03::{part1, part2}: 2025[3], r"
987654321111111
811111111111119
234234234234278
818181911112111
");

fn solve_line<const SIZE: usize>(ln: impl AsRef<[u8]>) -> u64 {
    let numbers = ln.as_ref();
    let mut total = 0;
    let mut last_index = 0;
    for n in 1..=SIZE {
        let (i, max) = numbers
            .iter()
            .enumerate()
            .skip(last_index)
            .rev()
            .skip(SIZE - n)
            .max_by_key(|(_, n)| *n)
            .unwrap();
        total += u64::from(max - b'0') * 10u64.pow((SIZE - n) as u32);
        last_index = i + 1;
    }
    total
}

#[aoc(part = 1, example = 357)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    input.map(solve_line::<2>).sum()
}

#[aoc(part = 2, example = 3121910778619)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input.map(solve_line::<12>).sum()
}
