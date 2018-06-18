pub use core::cmp::Ordering::Less;
pub use core::cmp::Ordering::Equal;
pub use core::cmp::Ordering::Greater;

pub use super::iter::{Iterator, IterRef, IterRefMut, into_iter};

pub use super::collection;
pub use super::list;
pub use super::list::slice;
pub use super::stack::Stack;
pub use super::deque::Deque;

pub use super::dlist::DoublyLinkedList;

pub use super::io::join;
pub use porus_macros::f;
pub use porus_macros::writef;
pub use porus_macros::writelnf;

pub fn default<T: Default>() -> T {
    Default::default()
}

#[macro_export]
macro_rules! read {
    ( $($expr:expr),* ) => (
        $(
            ::io::read($crate::io::Whitespace);
            ::io::read($expr);
        )*
    )
}

#[macro_export]
macro_rules! prelude {
    () => (
        prelude!(1024);
    );
    ($size:expr) => (
        #[allow(unused_imports)]
        use $crate::prelude::*;

        mod io {
            #[cfg(debug_assertions)]
            use std::ptr::drop_in_place;

            #[cfg(not(debug_assertions))]
            use core::ptr::drop_in_place;

            use $crate::io::stdio;
            use $crate::io::{fread, fwrite, Sink};

            #[allow(dead_code)]
            static mut STDIN : stdio::Input = stdio::stdin(&mut [0;$size]);
            static mut STDOUT : stdio::Output = stdio::stdout(&mut [0;$size]);

            #[allow(dead_code)]
            pub fn read<C: $crate::io::read::Consumer>(c: C) {
                unsafe {
                    fread(&mut STDIN, c);
                }
            }

            #[allow(dead_code)]
            pub fn write<'a, F : FnMut(&'a mut stdio::Output)>(f: F) {
                unsafe {
                    fwrite(&mut STDOUT, f);
                }
            }

            #[allow(dead_code)]
            pub fn writeln<'a, F : FnMut(&'a mut stdio::Output)>(f: F) {
                write(f);
                unsafe {
                    Sink::write(&mut STDOUT, b'\n');
                }
            }

            pub fn main() {
                ::solve();
                unsafe {
                    drop_in_place(&mut STDOUT as *mut _)
                };
            }
        }

        #[cfg(debug_assertions)]
        fn main() {
            io::main();
        }

        #[cfg(not(debug_assertions))]
        #[no_mangle]
        pub extern fn main() -> i32 {
            io::main();
            0
        }
    )
}
