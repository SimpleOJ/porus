#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let a: usize = read!();
    let b: usize = read!();
    let c: usize = read!();
    writelnf!("{:d}", (a..=b).filter(|x| (&c) % x == 0).count());
}
