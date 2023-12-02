use num::Num;
use std::ops::{Add, Mul, Sub};

pub mod day1;
pub mod day2;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + Add + Sub + Mul + Copy>(slice: &[u8]) -> T {
    let mut res = T::zero();
    let ten: T = 10.into();

    for &b in slice {
        res = res * ten + b.into() - b'0'.into();
    }

    res
}
