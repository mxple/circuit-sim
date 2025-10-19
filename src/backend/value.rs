use std::fmt;

fn mask(size: u8) -> u32 {
    if size == 32 {
        u32::MAX
    } else {
        (1 << size) - 1
    }
}

/// Four-state logic value using three 32-bit masks
/// Each bit can be: 0, 1, Z (high-impedance), or X (unknown/short circuit)
///
/// Encoding using three bit masks:
/// - logic_bits: the actual 0/1 values (only valid where high_z_mask & unknown_mask are both 0)
/// - high_z_mask: 1 indicates high-impedance, 0 indicates not high-impedance
/// - unknown_mask: 1 indicates unknown, 0 indicates not unknown
///
/// Bit state priority: unknown > high_z > logic
/// If unknown_mask[i] = 1: bit i is unknown (X)
/// Else if high_z_mask[i] = 1: bit i is high-impedance (Z)  
/// Else: bit i is logic (0 or 1 from logic_bits[i])
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value {
    logic_bits: u32,   // Logic values (0/1) - only valid where other masks are 0
    high_z_mask: u32,  // 1 = high-impedance, 0 = not high-impedance
    unknown_mask: u32, // 1 = unknown, 0 = not unknown
    width: u8,
}

impl Value {
    /// Create a pure logic value
    pub fn new_logic(value: u32, width: u8) -> Self {
        assert!(width > 0 && width <= 32);
        let mask = mask(width);
        Self {
            logic_bits: value & mask,
            high_z_mask: 0,
            unknown_mask: 0,
            width,
        }
    }

    /// Create a value with all bits in high-impedance state
    pub fn new_high_z(width: u8) -> Self {
        assert!(width > 0 && width <= 32);
				let mask = mask(width);
        Self {
            logic_bits: 0,
            high_z_mask: mask,
            unknown_mask: 0,
            width,
        }
    }

    /// Create a value with all bits in unknown state  
    pub fn new_unknown(width: u8) -> Self {
        assert!(width > 0 && width <= 32);
				let mask = mask(width);
        Self {
            logic_bits: 0,
            high_z_mask: 0,
            unknown_mask: mask,
            width,
        }
    }

    /// Create mixed-state value from masks
    pub fn from_masks(
        logic_bits: u32,
        high_z_mask: u32,
        unknown_mask: u32,
        width: u8,
    ) -> Self {
        assert!(width > 0 && width <= 32);
				let mask = mask(width);
        Self {
            logic_bits: logic_bits & mask,
            high_z_mask: high_z_mask & mask,
            unknown_mask: unknown_mask & mask,
            width,
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    /// Get logic value if all bits are in logic state, None otherwise
    pub fn as_logic(&self) -> Option<u32> {
        if self.is_all_logic() {
            Some(self.logic_bits)
        } else {
            None
        }
    }

    /// Check if all bits are in logic state (0 or 1)
    pub fn is_all_logic(&self) -> bool {
        (self.high_z_mask | self.unknown_mask) == 0
    }

    /// Check if any bit is in high-impedance state
    pub fn has_high_z(&self) -> bool {
        self.high_z_mask != 0
    }

    /// Check if any bit is in unknown state
    pub fn has_unknown(&self) -> bool {
        self.unknown_mask != 0
    }

    /// Get the state of a specific bit
    pub fn get_bit_state(&self, bit_index: u8) -> BitState {
        assert!(bit_index < 32);
        let bit_mask = 1u32 << bit_index;

        if (self.unknown_mask & bit_mask) != 0 {
            BitState::Unknown
        } else if (self.high_z_mask & bit_mask) != 0 {
            BitState::HighZ
        } else if (self.logic_bits & bit_mask) != 0 {
            BitState::Logic1
        } else {
            BitState::Logic0
        }
    }

    /// Set a specific bit to a logic value
    pub fn set_logic_bit(&mut self, bit_index: u8, value: bool) {
        assert!(bit_index < 32);
        let bit_mask = 1u32 << bit_index;

        // Clear from special states
        self.high_z_mask &= !bit_mask;
        self.unknown_mask &= !bit_mask;

        // Set logic value
        if value {
            self.logic_bits |= bit_mask;
        } else {
            self.logic_bits &= !bit_mask;
        }
    }

    /// Set a specific bit to high-impedance
    pub fn set_high_z_bit(&mut self, bit_index: u8) {
        assert!(bit_index < 32);
        let bit_mask = 1u32 << bit_index;

        self.unknown_mask &= !bit_mask; // Clear unknown
        self.high_z_mask |= bit_mask; // Set high-Z
        // self.logic_bits &= !bit_mask; // Clear logic bit
    }

    /// Set a specific bit to unknown
    pub fn set_unknown_bit(&mut self, bit_index: u8) {
        assert!(bit_index < 32);
        let bit_mask = 1u32 << bit_index;

        self.unknown_mask |= bit_mask; // Set unknown
        self.high_z_mask &= !bit_mask; // Clear high-Z
        self.logic_bits &= !bit_mask; // Clear logic bit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitState {
    Logic0,
    Logic1,
    HighZ,
    Unknown,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..self.width).rev() {
            match self.get_bit_state(i) {
                BitState::Logic0 => write!(f, "0")?,
                BitState::Logic1 => write!(f, "1")?,
                BitState::HighZ => write!(f, "Z")?,
                BitState::Unknown => write!(f, "X")?,
            }
        }
        Ok(())
    }
}

// Convenience constructors to maintain compatibility
impl Value {
    pub fn new(value: u32, width: u8) -> Self {
        Self::new_logic(value, width)
    }

    pub fn high_z(width: u8) -> Self {
        Self::new_high_z(width)
    }

    pub fn unknown(width: u8) -> Self {
        Self::new_unknown(width)
    }

    pub fn as_u32(&self) -> Option<u32> {
        self.as_logic()
    }

    pub fn is_high_z(&self) -> bool {
        self.has_high_z() && !self.has_unknown() && !self.is_all_logic()
    }

    pub fn is_unknown(&self) -> bool {
        self.has_unknown()
    }
}
