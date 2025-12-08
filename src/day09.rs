use std::{cmp::max_by_key, fs::File, io::Write, ops::Add};

use aoc_framework::*;

pub struct Day09;

impl_day!(Day09::{part1, part2}: 2025[9], r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
");

fn parse(input: impl Iterator<Item = String>) -> Vec<(i64, i64)> {
    input
        .flat_map(|ln| {
            ln.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .tuples()
                .next()
        })
        .collect()
}

#[aoc(part = 1, example = 50)]
fn part1(input: impl Iterator<Item = String>) -> u64 {
    let points = parse(input);
    let mut max = 0;
    for (i, (x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in &points[(i + 1)..] {
            let size = x1.abs_diff(x2).add(1) * y1.abs_diff(y2).add(1);
            if size > max {
                max = size;
            }
        }
    }
    max
}

#[allow(unused)]
fn write_to_svg(points: &[(i64, i64)], (x1, y1): (i64, i64), (x2, y2): (i64, i64)) {
    let x = x1.min(x2) as f64 / 100.;
    let w = x1.max(x2) as f64 / 100. - x;
    let y = y1.min(y2) as f64 / 100.;
    let h = y1.max(y2) as f64 / 100. - y;
    let mut out = File::create("day09.svg").unwrap();
    writeln!(
        &mut out,
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="1000" height="1000" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
<polyline stroke="red" fill="green" points="
"#
    )
    .unwrap();
    for (x, y) in points {
        let x = (*x as f64) / 100.;
        let y = (*y as f64) / 100.;
        write!(&mut out, "{x},{y} ").unwrap();
    }
    write!(
        &mut out,
        r#"" /><rect x="{x}" y="{y}" width="{w}" height="{h}" fill="lime" /> </svg>"#
    )
    .unwrap();
}

fn compute_turns(points: &[(i64, i64)]) -> (Vec<bool>, bool) {
    let len = points.len();
    let mut left_turns = 0;
    let mut right_turns = 0;
    let turns = points
        .iter()
        .enumerate()
        .map(|(i, &(xt, yt))| {
            let (x1, y1) = points[(i + len - 1) % len];
            let (x2, y2) = points[(i + 1) % len];
            if x1 != xt {
                (xt > x1) == (y2 > yt)
            } else {
                (yt > y1) != (x2 > xt)
            }
        })
        .inspect(|is_right| {
            if *is_right {
                right_turns += 1
            } else {
                left_turns += 1
            }
        })
        .collect_vec();
    let inside = right_turns > left_turns;
    (turns, inside)
}

#[aoc(part = 2, example = 24)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    let points = parse(input);
    let len = points.len();
    let mut sorted_x = points.iter().enumerate().collect_vec();
    let mut sorted_y = sorted_x.clone();
    sorted_x.sort_unstable_by_key(|(_, (x, _))| *x);
    sorted_y.sort_unstable_by_key(|(_, (_, y))| *y);
    let is_inside_corner = |ndx: usize, (x, y): (i64, i64)| {
        let (px, py) = points[ndx];
        let (x1, y1) = points[(ndx + len - 1) % len];
        let (x2, y2) = points[(ndx + 1) % len];
        let dx = if x1 == px { x2 } else { x1 } - px;
        let dy = if y1 == py { y2 } else { y1 } - py;
        (x - px).signum() == dx.signum() && (y - py).signum() == dy.signum()
    };
    let (turns, inside) = compute_turns(&points);
    let (res, _, _) = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, p2)| (i, p1, j, p2))
        })
        .filter(|&(i, p1 @ (x1, y1), j, p2 @ (x2, y2))| {
            let &minx = x1.min(x2);
            let &maxx = x1.max(x2);
            let &miny = y1.min(y2);
            let &maxy = y1.max(y2);

            if is_inside_corner(i, *p2) != (turns[i] == inside)
                || is_inside_corner(j, *p1) != (turns[j] == inside)
            {
                return false;
            }
            let first = sorted_x
                .binary_search_by_key(&minx, |(_, (x, _))| *x)
                .unwrap();
            let overlaps_vertical = sorted_x
                .iter()
                .skip(first)
                .skip_while(|(_, (x, _))| *x == minx)
                .take_while(|(_, (x, _))| *x < maxx)
                .any(|&(i, &(_, y))| {
                    let (_, ny) = points[(i + 1) % len];
                    (y > miny && y < maxy) || (y < miny && ny > miny) || (y > maxy && ny < maxy)
                });
            if overlaps_vertical {
                return false;
            }

            let first = sorted_y
                .binary_search_by_key(&miny, |(_, (_, y))| *y)
                .unwrap();
            let overlaps_horizontal = sorted_y
                .iter()
                .skip(first)
                .skip_while(|(_, (_, y))| *y == miny)
                .take_while(|(_, (_, y))| *y < maxy)
                .any(|&(i, &(x, _))| {
                    let (nx, _) = points[(i + 1) % len];
                    (x > minx && x < maxx) || (x < minx && nx > minx) || (x > maxx && nx < maxx)
                });
            !(overlaps_horizontal || overlaps_vertical)
        })
        .map(|(_, p1 @ (x1, y1), _, p2 @ (x2, y2))| {
            let size = x1.abs_diff(*x2).add(1) * y1.abs_diff(*y2).add(1);
            (size, p1, p2)
        })
        .max_by_key(|(size, _, _)| *size)
        .unwrap();
    res
}
