use num::Num;
use std::ops::{Add, Mul, Sub};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[inline(always)]
pub fn byte_slice_to_int<T: Num + From<u8> + Add + Sub + Mul + Copy>(slice: &[u8]) -> T {
    let mut res = T::zero();
    let ten: T = 10.into();

    for &b in slice {
        res = res * ten + b.into() - b'0'.into();
    }

    res
}
