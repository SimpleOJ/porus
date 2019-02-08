#![feature(alloc)]
extern crate alloc;
extern crate porus;

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::size_of;
use core::slice;
use core::ptr::copy_nonoverlapping;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = BTreeMap::new();
        for (i, x) in nums.iter().enumerate() {
            let y = target - x;
            if let Some(&j) = map.get(&y) {
                return vec![j as i32, i as i32];
            }
            map.insert(x, i);
        }
        return vec![-1, -1];
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn twoSum(nums: *const i32, numsSize: i32, target: i32) -> *const i32 {
    let v = Solution::two_sum(
        unsafe { slice::from_raw_parts(nums, numsSize as usize) }.to_vec(),
        target,
    );
    let p = unsafe { porus::libc::malloc(size_of::<i32>() * 2) as *mut i32 };
    unsafe { copy_nonoverlapping(v.as_ptr(), p, 2) };
    return p;
}
