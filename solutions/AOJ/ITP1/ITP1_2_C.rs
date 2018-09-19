#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let a: usize = read!();
    let b: usize = read!();
    let c: usize = read!();
    writelnf!(
        "{:d} {:d} {:d}",
        Ord::min(Ord::min(a, b), c),
        Ord::max(Ord::max(Ord::min(a, b), Ord::min(b, c)), Ord::min(a, c)),
        Ord::max(Ord::max(a, b), c)
    );
}
