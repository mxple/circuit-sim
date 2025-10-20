use std::ops::{BitAnd, BitOr, BitXor, Not};

fn mask(width: u8) -> u32 {
    if width == 32 { u32::MAX } else { (1u32 << width) - 1 }
}

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

impl Default for Value {
    fn default() -> Self {
        Self {
            bits: 0,
            float_mask: mask(32),
            width: 0,
            burned: false,
        }
    }
}

impl Value {
    /// create a fully-defined value (no floats)
    pub fn new(bits: u32, width: u8) -> Self {
        assert!((1..=32).contains(&width));
        let mask = mask(width);
        Self {
            bits: bits & mask,
            float_mask: 0,
            width,
            burned: false,
        }
    }

    pub fn from(val: u32) -> Self {
        Self {
            bits: val,
            float_mask: 0,
            width: 32,
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

    /// is logical hi? panic on burns
    /// main use case is for singular bits
    pub fn is_hi(self) -> bool {
        assert!(!self.burned);
        assert!(self.width == 1);
        assert!(self.float_mask == 0);
        self.bits & mask(self.width) != 0
    }

    pub fn is_floating(self) -> bool {
        assert!(!self.burned);
        self.float_mask != 0
    }
}


macro_rules! impl_bitwise_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self::Output {
                assert_eq!(self.width, rhs.width);
                if self.burned || rhs.burned {
                    return Self { burned: true, ..self };
                }
                let bits = self.bits $op rhs.bits;
                let float_mask = self.float_mask | rhs.float_mask;
                Self { bits, float_mask, width: self.width, burned: false }
            }
        }
    };
}

impl_bitwise_op!(BitAnd, bitand, &);
impl_bitwise_op!(BitOr, bitor, |);
impl_bitwise_op!(BitXor, bitxor, ^);

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

