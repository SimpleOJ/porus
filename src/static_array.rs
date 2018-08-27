use super::collection::Collection;
use super::list::{List, ListBase};

#[derive(List)]
pub struct StaticArray<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> StaticArray<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        StaticArray { slice }
    }
}

impl<'a, T: 'a> Collection for StaticArray<'a, T> {
    fn size(&self) -> usize {
        self.slice.len()
    }
}

impl<'a, T: 'a> ListBase for StaticArray<'a, T> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.slice.get(index)
    }
}

#[macro_export]
macro_rules! static_array {
    ($($arg:tt)*) => (
        &$crate::static_array::StaticArray::new(&[$($arg)*])
    );
}
