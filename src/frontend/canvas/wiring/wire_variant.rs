pub struct WireVariant(pub u8);

// 4 bits to determine whether connections exist for N, E, S, W
// 1 bit to determine if a wire with all 4 connections is a junction or overpass
impl WireVariant {
    /// Bits for N, E, S, W connections: 0b0000_NESW
    const NORTH: u8 = 0b1000;
    const EAST: u8 = 0b0100;
    const SOUTH: u8 = 0b0010;
    const WEST: u8 = 0b0001;

    /// Bit for junction (if all four connections), overpass if unset.
    const JUNCTION_BIT: u8 = 0b10000;

    pub fn new(north: bool, east: bool, south: bool, west: bool, junction: bool) -> Self {
        let mut value = 0u8;
        if north {
            value |= Self::NORTH;
        }
        if east {
            value |= Self::EAST;
        }
        if south {
            value |= Self::SOUTH;
        }
        if west {
            value |= Self::WEST;
        }
        if junction {
            value |= Self::JUNCTION_BIT;
        }
        Self(value)
    }

    pub fn merge_with(&self, other: &WireVariant) -> WireVariant {
        WireVariant(self.0 | other.0)
    }

    pub fn has_north(&self) -> bool {
        self.0 & Self::NORTH != 0
    }
    pub fn has_east(&self) -> bool {
        self.0 & Self::EAST != 0
    }
    pub fn has_south(&self) -> bool {
        self.0 & Self::SOUTH != 0
    }
    pub fn has_west(&self) -> bool {
        self.0 & Self::WEST != 0
    }
    #[allow(unused)]
    pub fn is_junction(&self) -> bool {
        // Only relevant if all four connections exist
        (self.0 & (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST))
            == (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST)
            && (self.0 & Self::JUNCTION_BIT != 0)
    }
    #[allow(unused)]
    pub fn is_overpass(&self) -> bool {
        // Only relevant if all four connections exist
        (self.0 & (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST))
            == (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST)
            && (self.0 & Self::JUNCTION_BIT == 0)
    }
}

