// http://practice.geeksforgeeks.org/problems/c-hello-world/0

#![cfg_attr(not(debug_assertions), no_main)]

extern crate porus;
use porus::io::*;


#[cfg_attr(not(debug_assertions), no_mangle)]
pub fn main() {
    let stdout = &mut stdout(1024);
    write(stdout, "Hello World\n");
}
