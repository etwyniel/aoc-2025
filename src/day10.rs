use std::{
    collections::{HashSet, VecDeque},
    fmt::Write,
    u128,
};

use aoc_framework::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Day10;

impl_day!(Day10::{part1, part2}: 2025[10], r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
");

fn parse_p1(ln: &str) -> (u16, Vec<u16>) {
    let mut it = ln.split_ascii_whitespace();
    let target_pattern = it
        .next()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']');
    let len = target_pattern.len() as u8;
    let target = target_pattern
        .bytes()
        .fold(0, |acc, b| (acc << 1) | u16::from(b == b'#'));
    let buttons = it
        .take_while(|s| s.starts_with('('))
        .map(|s| {
            s.trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .fold(0, |acc, v| acc | (1 << (len - v - 1)))
        })
        .collect();
    (target, buttons)
}

fn parse_p2(ln: &str) -> (u128, usize, Vec<u128>) {
    let (_, ln) = ln.split_once(' ').unwrap();
    let (buttons, target_pattern) = ln.rsplit_once(' ').unwrap();
    let mut len = 0;
    let target = target_pattern
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|s| s.parse::<u16>().unwrap())
        .inspect(|_| len += 1)
        .fold(0u128, |acc, v| (acc << 9) | v as u128);
    let buttons = buttons
        .split(' ')
        .map(|s| {
            s.trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .fold(0, |acc, v| acc | (1 << (9 * (len - v - 1))))
        })
        .collect();
    (target, len.into(), buttons)
}

#[aoc(part = 1, example = 7)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let mut checked = [false; 1024];
    let mut stack = VecDeque::new();
    input
        .map(|ln| parse_p1(&ln))
        .map(|(target, buttons)| {
            checked.iter_mut().for_each(|c| *c = false);
            checked[0] = true;
            stack.push_back((0, 0));
            let mut min = u64::MAX;
            while let Some((presses, cur)) = stack.pop_front() {
                let presses = presses + 1;
                for &button in &buttons {
                    let next = cur ^ button;
                    if next == target {
                        if presses < min {
                            min = presses
                        }
                    } else if !checked[next as usize] {
                        stack.push_back((presses, next));
                        checked[next as usize] = true;
                    }
                }
            }
            if min == u64::MAX {
                panic!("no sequence found for {target:b}")
            }
            min
        })
        .sum()
}

fn is_over_target(mut target: u128, mut cur: u128) -> bool {
    while target > 0 {
        if (cur & 0xff) > (target & 0xff) {
            return true;
        }
        cur >>= 9;
        target >>= 9;
    }
    false
}

fn print_value(v: u128, len: usize) -> String {
    // let mut len = 0;
    // let mut temp = v;
    // while temp > 0 {
    //     len += 1;
    //     temp >>= 9;
    // }
    let mut out = String::new();
    // dbg!(v);
    for i in 0..len {
        if i > 0 {
            out.write_str(", ");
        }
        let off = len - i - 1;
        // dbg!((v >> (9 * off)) & 0x1ff);
        out.write_str(&format!("{:3}", (v >> (9 * off)) & 0x1ff));
    }
    out
}

fn solve(target: u128, len: usize, buttons: &[u128]) -> Result<u64, u128> {
    let Some(b) = buttons.first().copied() else {
        eprintln!("last button");
        return if target == 0 { Ok(0) } else { Err(target) };
    };
    println!(
        "target: {}\nb:      {}\n",
        print_value(target, len),
        print_value(b, len)
    );
    let mut min = u16::MAX;
    let mut cur = target;
    let mut temp = b;
    while temp > 0 {
        if temp & 0x1ff > 0 {
            let v = (cur & 0x1ff) as u16;
            if v < min {
                min = v;
            }
        }
        cur >>= 9;
        temp >>= 9;
    }
    let mut n = 0;
    loop {
        let presses = min - n;
        let rem_target = target - presses as u128 * b;
        // eprintln!("presses: {presses} -> -{:x}", presses as u128 * b);
        // eprintln!("{rem_target:x}");
        if rem_target == 0 {
            eprintln!("res: {presses} * {}", print_value(b, len));
            return Ok(presses as u64);
        }
        let t = match solve(rem_target, len, &buttons[1..]) {
            Ok(p) => {
                eprintln!("res: {presses} * {}", print_value(b, len));
                return Ok(p + presses as u64);
            }
            Err(t) => t,
        };
        let mut max = 0;
        let mut cur = t;
        let mut temp = b;
        while temp > 0 {
            if temp & 0x1ff != 0 {
                let v = (cur & 0x1ff) as u16;
                if v > max {
                    max = v;
                }
            }
            cur >>= 9;
            temp >>= 9;
        }
        if max > presses || max <= n {
            return Err(t);
        }
        n = max;
        // let presses = min - n - max;
        // return solve(target - presses as u128 * b, &buttons[1..]).map(|p| presses as u64 + p);
    }
    // Err(target)
}

#[aoc(part = 2, example = 33)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    // let mut checked = HashSet::new();
    // let mut stack = VecDeque::new();
    input
        // .into_par_iter()
        .map(|ln| parse_p2(&ln))
        .map(|(target, len, mut buttons)| {
            buttons.sort_unstable_by_key(|&b| std::cmp::Reverse(b));
            dbg!(solve(target, len, &buttons).unwrap())
            // let mut max_presses = vec![0u8; buttons.len()];
            // loop {
            //     let mut rem_target = target;
            //     for (i, mut b) in buttons.iter().copied().enumerate() {
            //         let mut min = u128::MAX;
            //         let mut cur = rem_target;
            //         while b > 0 {
            //             if b & 0xff > 0 {
            //                 let v = cur & 0xff;
            //                 if v < min {
            //                     min = v;
            //                 }
            //             }
            //             cur >>= 8;
            //             b >>= 8;
            //         }
            //         rem_target -= cur * b;
            //         max_presses[i] = min as u8;
            //     }
            //     if rem_target == 0 {
            //         break;
            //     }
            // }
            // max_presses.into_iter().map(u64::from).sum::<u64>()

            // eprintln!("target: {target:x}");
            // checked.clear();
            // checked.insert(0);
            // stack.push_back((0, 0));
            // let mut min = u64::MAX;
            // while let Some((presses, cur)) = stack.pop_front() {
            //     let presses = presses + 1;
            //     for &button in &buttons {
            //         let next = cur + button;
            //         // eprintln!("{cur:x} + {button:x} = {next:x}");
            //         if next == target {
            //             if presses < min {
            //                 min = presses
            //             }
            //         } else if is_over_target(target, next) {
            //             continue;
            //         } else if !checked.contains(&next) {
            //             stack.push_back((presses, next));
            //             checked.insert(next);
            //         }
            //     }
            // }
            // if min == u64::MAX {
            //     for c in &checked {
            //         eprintln!("checked {c:x}");
            //     }
            //     panic!("no sequence found for {target:x}")
            // }
            // dbg!(min);
            // min
        })
        .sum()
}
