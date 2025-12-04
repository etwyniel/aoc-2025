use aoc_framework::*;

pub struct Day04;

impl_day!(Day04::{part1, part2}: 2025[4], r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
");

fn neighbors(w: usize) -> [isize; 8] {
    let mut points = [0; 8];
    let mut n = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            points[n] = y * w as isize + x;
            n += 1;
        }
    }
    points
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    Empty,
    Present,
    Checking,
}

fn parse(input: Vec<String>) -> (Vec<State>, Vec<usize>, [isize; 8]) {
    let w = input[0].len() + 2;
    let h = input.len() + 2;
    let neighbors = neighbors(w);
    let mut grid = vec![State::Empty; h * w];
    let mut points = Vec::new();
    input.iter().enumerate().for_each(|(y, ln)| {
        let y = y + 1;
        ln.bytes()
            .enumerate()
            .filter(|(_, b)| *b == b'@')
            .for_each(|(x, _)| {
                let x = x + 1;
                let ndx = y * h + x;
                grid[ndx] = State::Checking;
                points.push(ndx);
            })
    });
    (grid, points, neighbors)
}

#[aoc(part = 1, example = 13)]
fn part1(input: Vec<String>) -> u64 {
    let (grid, points, neighbors) = parse(input);
    points
        .into_iter()
        .filter(|&p| {
            neighbors
                .iter()
                .map(|&off| (p as isize + off) as usize)
                .filter(|&n| grid[n] != State::Empty)
                .take(4)
                .count()
                < 4
        })
        .count() as u64
}

#[aoc(part = 2, example = 43)]
fn part2(input: Vec<String>) -> u64 {
    let (mut grid, mut stack, neighbors) = parse(input);
    let mut removed = 0;
    let mut neigh = Vec::new();
    'outer: while let Some(p) = stack.pop() {
        grid[p] = State::Present;
        neigh.clear();
        for off in &neighbors {
            let n = (p as isize + off) as usize;
            if grid[n] == State::Empty {
                continue;
            }
            if neigh.len() >= 3 {
                continue 'outer;
            }
            neigh.push(n);
        }

        grid[p] = State::Empty;
        for &n in neigh.iter() {
            if grid[n] == State::Checking {
                continue;
            }
            grid[n] = State::Checking;
            stack.push(n);
        }
        removed += 1;
    }
    removed
}
