use std::{collections::HashMap, fmt::Debug};

use aoc_framework::*;

pub struct Day11;

impl_day!(Day11::{part1, part2}: 2025[11], r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
", r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
");

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Label(u32);

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            let c = ((self.0 >> (8 * (2 - i))) & 0xff) as u8 as char;
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl From<&str> for Label {
    fn from(s: &str) -> Label {
        Label(s.bytes().take(3).fold(0, |acc, b| (acc << 8) | (b as u32)))
    }
}

#[aoc(part = 1, example = 5)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let graph: HashMap<Label, Vec<Label>> = input
        .map(|ln| {
            let (src, dsts) = ln.split_once(": ").unwrap();
            let src = Label::from(src);
            let dsts = dsts.split(' ').map(Label::from).collect_vec();
            (src, dsts)
        })
        .collect();
    let out = Label::from("out");
    let mut stack = vec![Label::from("you")];
    let mut paths = 0;
    while let Some(cur) = stack.pop() {
        if cur == out {
            paths += 1;
            continue;
        }
        let Some(next) = graph.get(&cur) else {
            continue;
        };
        stack.extend(next);
    }
    paths as u64
}

#[aoc(part = 2, example = 2)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut labels = HashMap::new();
    let mut get_label = |label: Label| {
        let len = labels.len();
        *labels.entry(label).or_insert(len)
    };
    let nodes = input
        .map(|ln| {
            let (src, dsts) = ln.split_once(": ").unwrap();
            let src = get_label(src.into());
            (src, dsts.to_string())
        })
        .collect_vec();
    let graph: Vec<Vec<usize>> = nodes
        .iter()
        .map(|(_, dsts)| {
            dsts.split(' ')
                .map(|dst| get_label(dst.into()))
                .collect_vec()
        })
        .collect();
    let svr = get_label("svr".into());
    let out = get_label("out".into());
    let fft = get_label("fft".into());
    let dac = get_label("dac".into());
    let mut memo = vec![None; labels.len()];
    memo[out] = Some((1, 0, 0, 0));
    let mut stack = vec![svr];
    while let Some(&cur) = stack.last() {
        let Some(next) = graph.get(cur) else {
            continue;
        };
        let mut total = 0;
        let mut with_fft = 0;
        let mut with_dac = 0;
        let mut valid = 0;
        let mut all = true;
        for &n in next {
            if let Some((ntotal, nwith_fft, nwith_dac, nvalid)) = memo[n] {
                total += ntotal;
                with_fft += nwith_fft;
                with_dac += nwith_dac;
                valid += nvalid;
            } else {
                stack.push(n);
                all = false;
            }
        }
        if !all {
            continue;
        }
        stack.pop();
        if cur == fft {
            valid += with_dac;
            with_fft = total;
        } else if cur == dac {
            valid += with_fft;
            with_dac = total;
        }
        memo[cur] = Some((total, with_fft, with_dac, valid));
    }
    memo[svr].unwrap().3
}
