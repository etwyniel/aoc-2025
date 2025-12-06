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
    let Some((start, _)) = first.bytes().find_position(|&b| b == b'S') else {
        return 0;
    };
    let w = first.len();
    let mut grid = Vec::new();
    input.for_each(|ln| grid.extend(ln.bytes().map(|b| b == b'^')));
    let mut stack = vec![start];
    let mut memo = vec![0; grid.len()];
    while let Some(pos) = stack.last().copied() {
        let mut cur = pos;
        while grid.get(cur).map(|b| !*b).unwrap_or(false) {
            cur += w;
        }
        if cur >= grid.len() {
            memo[pos] = 1;
            stack.pop();
            continue;
        }
        let l = cur - 1;
        let r = cur + 1;
        let left = memo[l];
        if left == 0 {
            stack.push(l);
        }
        let right = memo[r];
        if right == 0 {
            stack.push(r);
        }
        if left != 0 && right != 0 {
            memo[pos] = left + right;
            stack.pop();
        }
    }
    memo[start] + u64::from(w > 30)
}
