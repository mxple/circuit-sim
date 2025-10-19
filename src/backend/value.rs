use std::ops::{BitAnd, BitOr, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
    /// actual bits (valid up to `width`)
    pub bits: u32,
    /// per-bit floating (1 = floating/unknown)
    pub float_mask: u32,
    /// number of active bits (1..=32)
    pub width: u8,
    /// local short/burn flag
    pub burned: bool,
}

impl Value {
    /// create a fully-defined value (no floats)
    pub fn new(bits: u32, width: u8) -> Self {
        assert!((1..=32).contains(&width));
        let mask = if width == 32 { u32::MAX } else { (1u32 << width) - 1 };
        Self {
            bits: bits & mask,
            float_mask: 0,
            width,
            burned: false,
        }
    }

    /// create an all-floating value
    pub fn floating(width: u8) -> Self {
        assert!((1..=32).contains(&width));
        let mask = if width == 32 { u32::MAX } else { (1u32 << width) - 1 };
        Self {
            bits: 0,
            float_mask: mask,
            width,
            burned: false,
        }
    }

    /// mark value as burned
    pub fn burn(mut self) -> Self {
        self.burned = true;
        self
    }
}


impl BitAnd for Value {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        if self.burned || rhs.burned {
            return Self { burned: true, ..self };
        }
        let bits = self.bits & rhs.bits;
        let float_mask = self.float_mask | rhs.float_mask;
        Self { bits, float_mask, width: self.width, burned: false }
    }
}

impl BitOr for Value {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        if self.burned || rhs.burned {
            return Self { burned: true, ..self };
        }
        let bits = self.bits | rhs.bits;
        let float_mask = self.float_mask | rhs.float_mask;
        Self { bits, float_mask, width: self.width, burned: false }
    }
}

impl Not for Value {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.burned {
            return self;
        }
        let mask = if self.width == 32 { u32::MAX } else { (1u32 << self.width) - 1 };
        let bits = (!self.bits) & mask;
        Self { bits, float_mask: self.float_mask, width: self.width, burned: self.burned }
    }
}

