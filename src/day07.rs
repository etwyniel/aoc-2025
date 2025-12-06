use std::collections::HashMap;

use aoc_framework::*;

pub struct Day07;

impl_day!(Day07::{part1, part2}: 2025[7], r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
");

#[aoc(part = 1, example = 21)]
fn part1(mut input: impl Iterator<Item = String>) -> u64 {
    let Some(first) = input.next() else { return 0 };
    let mut positions: Vec<bool> = first.bytes().map(|b| b == b'S').collect();
    let mut splits = 0;
    for ln in input {
        for (i, b) in ln.bytes().enumerate() {
            if !positions[i] || b != b'^' {
                continue;
            }
            splits += 1;
            positions[i] = false;
            positions[i - 1] = true;
            positions[i + 1] = true;
        }
    }
    splits
}

#[aoc(part = 2, example = 40)]
fn part2(mut input: impl Iterator<Item = String>) -> u64 {
    let Some(first) = input.next() else { return 0 };
    let Some((start, _)) = first.as_bytes().iter().find_position(|&&b| b == b'S') else {
        return 0;
    };
    let w = first.len();
    let mut grid = Vec::new();
    input.for_each(|ln| grid.extend(ln.bytes().map(|b| b == b'^')));
    let h = grid.len() / w;
    let mut positions = vec![(0, start)];
    let mut memo = HashMap::new();
    while let Some((mut y, x)) = positions.last().copied() {
        let start_y = y;
        while y < h && !grid[y * w + x] {
            y += 1;
        }
        if y == h {
            memo.insert((start_y, x), 1);
            positions.pop();
            continue;
        }
        let l = (y + 1, x - 1);
        let r = (y + 1, x + 1);
        let left = memo.get(&l);
        if left.is_none() {
            positions.push(l);
        }
        let right = memo.get(&r);
        if right.is_none() {
            positions.push(r);
        }
        if let Some(left) = left
            && let Some(right) = right
        {
            memo.insert((start_y, x), left + right);
            positions.pop();
        }
        continue;
    }
    memo[&(0, start)]
}
