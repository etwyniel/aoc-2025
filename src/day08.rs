use std::collections::BTreeMap;

use aoc_framework::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Day08;

impl_day!(Day08::{part1, part2}: 2025[8], r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
");

fn solve<const PART2: bool>(input: impl Iterator<Item = String>) -> u64 {
    let points: Vec<(i64, i64, i64)> = input
        .map(|ln| {
            ln.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .collect_vec();
    let len = points.len();
    let mut pc: Vec<Option<usize>> = vec![None; len];
    let mut chains = Vec::new();
    let mut nearest = (0..len)
        .into_par_iter()
        .map(|i| {
            let mut v = Vec::with_capacity(len);
            let (ix, iy, iz) = points[i];
            for (j, (jx, jy, jz)) in points.iter().enumerate().skip(i + 1).take(len) {
                let dist = (ix - jx).pow(2) + (iy - jy).pow(2) + (iz - jz).pow(2);
                v.push((j, dist));
            }
            v.sort_unstable_by_key(|(_, dist)| -*dist);
            v
        })
        .collect::<Vec<_>>();
    let mut btree = BTreeMap::new();
    for (i, nearest) in nearest.iter_mut().enumerate() {
        if let Some((j, dist)) = nearest.pop() {
            btree.insert(dist, (i, j));
        }
    }
    let target = if PART2 {
        usize::MAX
    } else if points.len() < 100 {
        10
    } else {
        1000
    };
    for _ in 0..target {
        let (_, (i, j)) = btree.pop_first().unwrap();
        if let Some((j, dist)) = nearest[i].pop() {
            btree.insert(dist, (i, j));
        }
        if let (Some(c1), Some(c2)) = (pc[i], pc[j]) {
            if c1 != c2 {
                let c = c1.min(c2);
                let replaced = c1.max(c2);
                pc.iter_mut()
                    .filter(|p| **p == Some(replaced))
                    .for_each(|p| *p = Some(c));
                chains[c] += chains[replaced];
                chains[replaced] = 0;
            }
        } else {
            let c = pc[i].or(pc[j]).unwrap_or_else(|| {
                let c = chains.len();
                chains.push(0);
                c
            });
            chains[c] += pc[i].is_none() as usize + pc[j].is_none() as usize;
            pc[i] = Some(c);
            pc[j] = Some(c);
        }
        if PART2 && chains[0] == points.len() {
            return (points[i].0 * points[j].0) as u64;
        }
    }
    chains.sort_unstable();
    chains.into_iter().rev().take(3).product::<usize>() as u64
}

#[aoc(part = 1, example = 40)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    solve::<false>(input)
}

#[aoc(part = 2, example = 25272)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    solve::<true>(input)
}
