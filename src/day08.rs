use std::collections::HashSet;

use aoc_framework::*;

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

#[aoc(part = 1, example = 40)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let points: Vec<(i64, i64, i64)> = input
        .map(|ln| {
            ln.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .collect_vec();
    let mut pc: Vec<Option<usize>> = vec![None; points.len()];
    let target = if points.len() < 100 { 10 } else { 1000 };
    let mut chains = Vec::new();
    let len = points.len();
    let mut pairs = HashSet::new();
    for _ in 0..target {
        let (i, j) = (0..len)
            .flat_map(|i| ((i + 1)..len).filter(move |&j| j != i).map(move |j| (i, j)))
            .filter(|pair| !pairs.contains(pair))
            .min_by_key(|&(i, j)| {
                ((points[i].0 - points[j].0).pow(2)
                    + (points[i].1 - points[j].1).pow(2)
                    + (points[i].2 - points[j].2).pow(2))
                .isqrt()
            })
            .unwrap();
        pairs.insert((i, j));
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
            continue;
        }
        let c = pc[i].or(pc[j]).unwrap_or_else(|| {
            let c = chains.len();
            chains.push(0);
            c
        });
        chains[c] += pc[i].is_none() as usize + pc[j].is_none() as usize;
        pc[i] = Some(c);
        pc[j] = Some(c);
    }
    chains.sort_unstable();
    chains.into_iter().rev().take(3).product::<usize>() as u64
}

#[aoc(part = 2, example = 25272)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let points: Vec<(i64, i64, i64)> = input
        .map(|ln| {
            ln.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .tuples()
                .next()
                .unwrap()
        })
        .collect_vec();
    let mut pc: Vec<Option<usize>> = vec![None; points.len()];
    let mut chains = Vec::new();
    let len = points.len();
    let mut pairs = vec![false; len * len];
    loop {
        let (i, j) = (0..len)
            .flat_map(|i| ((i + 1)..len).filter(move |&j| j != i).map(move |j| (i, j)))
            .filter(|&(i, j)| !pairs[i * len + j])
            .min_by_key(|&(i, j)| {
                (points[i].0 - points[j].0).pow(2)
                    + (points[i].1 - points[j].1).pow(2)
                    + (points[i].2 - points[j].2).pow(2)
            })
            .unwrap();
        pairs[i * len + j] = true;
        if let (Some(c1), Some(c2)) = (pc[i], pc[j]) {
            if c1 != c2 {
                let c = c1.min(c2);
                let replaced = c1.max(c2);
                pc.iter_mut()
                    .filter(|p| **p == Some(replaced))
                    .for_each(|p| *p = Some(c));
                chains[c] += chains[replaced];
                chains[replaced] = 0;
                if c == 0 {
                    eprintln!("{}/{}", chains[0], points.len());
                }
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
        if chains[0] == points.len() {
            return (points[i].0 * points[j].0) as u64;
        }
    }
}
