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
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self {
            value: *value as u16,
        }
    }
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn saturating_add(&self, other: Self) -> Self {
        let sum = self.value.saturating_add(other.value);
        Self::new(sum)
    }

    pub fn saturating_add_u16(&self, other: u16) -> Self {
        let sum = self.value.saturating_add(other);
        Self::new(sum)
    }

    pub fn saturating_add_ref_u16(&self, other: &u16) -> Self {
        let sum = self.value.saturating_add(*other);
        Self::new(sum)
    }

    pub fn saturating_add_ref(&self, other: &Self) -> Self {
        let sum = self.value.saturating_add(other.value);
        Self::new(sum)
    }

    pub fn saturating_add_u8(&self, other: u8) -> Self {
        let sum = self.value.saturating_add(other as u16);
        Self::new(sum)
    }

    pub fn saturating_add_ref_u8(&self, other: &u8) -> Self {
        let sum = self.value.saturating_add(*other as u16);
        Self::new(sum)
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &SaturatingU16) -> Self::Output {
        Self {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(other),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(*other),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
