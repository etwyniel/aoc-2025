use std::{collections::VecDeque, ffi::c_void, fmt::Write};

use aoc_framework::*;
use libc::{c_double, c_int};

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

fn print_value(v: u128, len: usize) -> String {
    let mut out = String::new();
    for i in 0..len {
        if i > 0 {
            out.write_str(", ").unwrap();
        }
        let off = len - i - 1;
        out.write_str(&format!("{:3}", (v >> (9 * off)) & 0x1ff))
            .unwrap();
    }
    out
}
#[repr(C)]
struct GlpProb(c_void);

#[repr(transparent)]
struct FooBar([c_double; 33]);

impl Default for FooBar {
    fn default() -> Self {
        FooBar([0.; 33])
    }
}

#[derive(Default)]
#[repr(C)]
struct GlpSmcp {
    msg_lev: c_int,
    meth: c_int,
    pricing: c_int,
    r_test: c_int,
    tol_bnd: c_double,
    tol_dj: c_double,
    tol_piv: c_double,
    obj_ll: c_double,
    obj_ul: c_double,
    it_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    presolve: c_int,
    excl: c_int,
    shift: c_int,
    aorn: c_int,
    foo_bar: FooBar,
}

unsafe extern "C" {
    fn glp_create_prob() -> *mut GlpProb;
    fn glp_set_obj_dir(_: *mut GlpProb, _: c_int);
    fn glp_add_rows(_: *mut GlpProb, _: c_int) -> c_int;
    fn glp_add_cols(_: *mut GlpProb, _: c_int) -> c_int;
    fn glp_set_row_bnds(_: *mut GlpProb, _: c_int, _: c_int, _: c_double, _: c_double);
    fn glp_set_col_bnds(_: *mut GlpProb, _: c_int, _: c_int, _: c_double, _: c_double);
    fn glp_set_col_kind(_: *mut GlpProb, _: c_int, _: c_int);
    fn glp_mip_col_val(_: *mut GlpProb, _: c_int) -> c_double;
    fn glp_set_obj_coef(_: *mut GlpProb, _: c_int, _: c_double);
    fn glp_load_matrix(
        _: *mut GlpProb,
        _: c_int,
        _: *const c_int,
        _: *const c_int,
        _: *const c_double,
    );
    fn glp_init_smcp(_: *mut GlpSmcp);
    fn glp_simplex(_: *mut GlpProb, _: *const GlpSmcp) -> c_int;
    fn glp_intopt(_: *mut GlpProb, _: *const c_void) -> c_int;
    fn glp_mip_obj_val(_: *mut GlpProb) -> c_double;
    fn glp_term_out(_: c_int);
    fn glp_delete_prob(_: *mut GlpProb);
}

fn solve(buttons: &[u128], len: usize, target: u128) -> u64 {
    let len = len as c_int;
    unsafe {
        glp_term_out(0);
        let p = glp_create_prob();
        glp_set_obj_dir(p, 1);

        glp_add_rows(p, len as c_int);
        for i in (0..len).rev() {
            let tgt = (target >> (i * 9)) & 0x1ff;
            glp_set_row_bnds(p, len - i as c_int, 5, tgt as c_double, 0.);
        }

        glp_add_cols(p, buttons.len() as c_int);
        let mut ia = vec![0];
        let mut ja = vec![0];
        let mut ar = vec![0.];
        for (i, &b) in buttons.iter().enumerate() {
            let ndx = i as c_int + 1;
            glp_set_col_bnds(p, ndx, 2, 0., 0.);
            glp_set_col_kind(p, ndx, 2);
            glp_set_obj_coef(p, ndx, 1.);

            for j in 0..len {
                if (b >> ((len - j - 1) * 9)) & 0x1ff != 0 {
                    ia.push(j as c_int + 1);
                    ja.push(ndx);
                    ar.push(1.);
                }
            }
        }

        glp_load_matrix(
            p,
            ia.len() as c_int - 1,
            ia.as_ptr(),
            ja.as_ptr(),
            ar.as_ptr(),
        );
        let mut params = GlpSmcp {
            meth: 3,
            ..Default::default()
        };
        glp_init_smcp(&mut params);
        glp_simplex(p, &params);
        glp_intopt(p, std::ptr::null());
        let res = glp_mip_obj_val(p);
        let mut total = 0;
        for (i, &b) in buttons.iter().enumerate() {
            let col = glp_mip_col_val(p, i as c_int + 1);
            total += col as u128 * b;
        }
        assert_eq!(
            print_value(target, len as usize),
            print_value(total, len as usize)
        );
        glp_delete_prob(p);
        res.round() as u64
    }
}

#[aoc(part = 2, example = 33)]
fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|ln| parse_p2(&ln))
        .map(|(target, len, buttons)| solve(&buttons, len, target))
        .sum()
}
