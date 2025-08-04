use egui_macroquad::macroquad::prelude::*;

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
    pub fn is_junction(&self) -> bool {
        // Only relevant if all four connections exist
        (self.0 & (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST))
            == (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST)
            && (self.0 & Self::JUNCTION_BIT != 0)
    }
    pub fn is_overpass(&self) -> bool {
        // Only relevant if all four connections exist
        (self.0 & (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST))
            == (Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST)
            && (self.0 & Self::JUNCTION_BIT == 0)
    }
}
use std::fmt;

use crate::frontend::canvas::camera::GridCamera;

impl fmt::Display for WireVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = if self.has_north() { 'N' } else { '-' };
        let e = if self.has_east() { 'E' } else { '-' };
        let s = if self.has_south() { 'S' } else { '-' };
        let w = if self.has_west() { 'W' } else { '-' };
        if self.is_junction() {
            write!(f, "[{}{}{}{}][J]", n, e, s, w)
        } else if self.is_overpass() {
            write!(f, "[{}{}{}{}][O]", n, e, s, w)
        } else {
            write!(f, "[{}{}{}{}]", n, e, s, w)
        }
    }
}

pub struct Wire {
    pub position: Vec2,
    pub variant: WireVariant,
}

impl Wire {
    pub fn new(position: Vec2, variant: WireVariant) -> Self {
        Self { position, variant }
    }

    pub fn draw(&self) {
        let width: f32 = 0.2;
        // let width: f32 = 1.0;
        let pos = self.position;
        let scale = 1.0;
    
        let center = (scale - width * scale) / 2.0;
        let size = width * scale;
        let color = GREEN;
    
        let is_overpass = self.variant.is_overpass();
        let gap = if is_overpass { size * 0.75 } else { 0.0 };
    
        draw_rectangle(
            pos.x + center,
            pos.y + center,
            size,
            size,
            color,
        );
    
        if self.variant.has_north() {
            let height = if is_overpass {
                center + size / 2.0 - gap
            } else {
                center + size / 2.0
            };
            draw_rectangle(
                pos.x + center,
                pos.y,
                size,
                height,
                color,
            );
        }
    
        if self.variant.has_south() {
            let y_start = if is_overpass {
                pos.y + center + size / 2.0 + gap
            } else {
                pos.y + center + size / 2.0
            };
            let height = if is_overpass {
                center + size / 2.0 - gap / 2.0
            } else {
                center + size / 2.0
            };
            draw_rectangle(
                pos.x + center,
                y_start,
                size,
                height,
                color,
            );
        }
    
        if self.variant.has_east() {
            draw_rectangle(
                pos.x + center + size / 2.0,
                pos.y + center,
                center + size / 2.0,
                size,
                color,
            );
        }
    
        if self.variant.has_west() {
            draw_rectangle(
                pos.x,
                pos.y + center,
                center + size / 2.0,
                size,
                color,
            );
        }
    }
}
