use aoc_framework::{grid::Grid, *};

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

#[aoc(part = 1, example = 13)]
fn part1(input: Vec<String>) -> u64 {
    let g = Grid::from_lines(input, |b| b == b'@');
    g.points_iter()
        .filter(|&p| g[p])
        .filter(|p| {
            p.neighbors_diag()
                .filter(|&n| g.get(n) == Some(&true))
                .count()
                < 4
        })
        .count() as u64
}

#[aoc(part = 2, example = 43)]
fn part2(input: Vec<String>) -> u64 {
    let mut g = Grid::from_lines(input, |b| b == b'@');
    let mut removed = 0;
    loop {
        let removable = g
            .points_iter()
            .filter(|&p| g[p])
            .filter(|p| {
                p.neighbors_diag()
                    .filter(|&n| g.get(n) == Some(&true))
                    .count()
                    < 4
            })
            .collect::<Vec<_>>();
        if removable.is_empty() {
            break;
        }
        removed += removable.len() as u64;
        removable.into_iter().for_each(|p| {
            g.set(p, false);
        })
    }
    removed
}
