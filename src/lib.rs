#![feature(specialization)]
#![feature(core_intrinsics)]
#![no_std]

#[macro_use]
pub mod libc;
pub mod ctype;
pub mod traits;
pub mod storage;
pub mod iter;

#[macro_use]
pub mod string;
pub mod io;
pub mod collections;
