pub use super::compat::prelude::*;

#[allow(non_camel_case_types)]
pub type int = isize;

pub use std::cmp::Ordering::Less;
pub use std::cmp::Ordering::Equal;
pub use std::cmp::Ordering::Greater;

pub use super::io;

#[cfg(debug_assertions)]
mod internal {
    use std::boxed::Box;

    pub type Error = Box<::std::error::Error>;
}


#[cfg(not(debug_assertions))]
mod internal {
    pub use super::super::compat::error::Error;
}

pub use self::internal::*;

pub fn default<T: Default>() -> T {
    Default::default()
}
