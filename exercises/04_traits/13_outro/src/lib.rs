//  Define a new `SaturatingU16` type.
//   x It should hold a `u16` value.
//   x It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   x It should support addition with a right-hand side of type
//   x SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   x It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   x It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value.into(),
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: (*value).into(),
        }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;

    fn add(self, other: u16) -> u16 {
        self.value.saturating_add(other) //.into()
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self {
        SaturatingU16 {
            value: self.value.saturating_add(*other), //.into()
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
