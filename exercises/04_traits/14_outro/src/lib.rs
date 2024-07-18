// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16 {
     value: u16
}

impl From<u16> for SaturatingU16 {
    fn from(item: u16) -> Self {
        SaturatingU16{ value: item}
    }
}

impl From<u8> for SaturatingU16 {
    fn from(item: u8) -> Self {
        SaturatingU16{ value: item.into()}
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(item: &u8) -> Self {
        SaturatingU16{ value: <u8 as Into<u16>>::into(*item)}
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(item: &u16) -> Self {
        SaturatingU16 { value: *item}
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
        
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        let sum = self.value.saturating_add(rhs);
        Self::Output{
            value: sum
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        self+*rhs
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *(other)
    }
}
