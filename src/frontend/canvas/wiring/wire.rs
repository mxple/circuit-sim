use egui_macroquad::macroquad::prelude::*;

pub struct WireVariant(pub u16);

// 4 bits to determine whether connections exist for N, E, S, W
// 1 bit to determine if a wire with all 4 connections is a junction or overpass
impl WireVariant {
    /// Bits for N, E, S, W connections: 0b0000_NESW
    const NORTH: u16 = 1 << 3;
    const EAST: u16 = 1 << 2;
    const SOUTH: u16 = 1 << 1;
    const WEST: u16 = 1 << 0;

    /// Bit for junction (if all four connections), overpass if unset.
    const JUNCTION_BIT: u16 = 1 << 4;

    /// If set on an overpass, horizontal wire is rendered on top of vertical wire.
    const DEPTH: u16 = 1 << 5;

    /// For non-overpass wires, COLOR_A is the primary color and COLOR_B is ignored.
    /// For overpass wires, COLOR_A is the vertical wire's color and COLOR_B is the horizontal
    /// wire's color.
    const COLOR_A_1: u16 = 1 << 6;
    const COLOR_A_2: u16 = 1 << 7;
    const COLOR_B_1: u16 = 1 << 8;
    const COLOR_B_2: u16 = 1 << 9;

    pub const NUM_VARIANTS: u16 = 1 << 10;

    pub fn new(north: bool, east: bool, south: bool, west: bool, junction: bool) -> Self {
        let mut value = 0u16;
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
    pub fn vertical_on_top(&self) -> bool {
        self.0 & Self::DEPTH == 0
    }
    fn get_one_color(color_1: u16, color_2: u16) -> Color {
        if color_1 == 0 && color_2 == 0 {
            // ZERO
            DARKGREEN
        } else if color_1 == 0 && color_2 != 0 {
            // ONE
            GREEN
        } else if color_1 != 0 && color_2 == 0 {
            // HIGH IMPEDANCE
            BLUE
        } else if color_1 != 0 && color_2 != 0 {
            // SHORT CIRCUIT
            RED
        } else {
            unreachable!()
        }
    }
    pub fn get_colors(&self) -> (Color, Color) {
        (
            Self::get_one_color(self.0 & Self::COLOR_A_1, self.0 & Self::COLOR_A_2),
            Self::get_one_color(self.0 & Self::COLOR_B_1, self.0 & Self::COLOR_B_2),
        )
    }
}
use std::fmt;

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
    // TODO: is it necessary to store position in the wire itself?
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
        let (color_a, color_b) = self.variant.get_colors();

        let is_overpass = self.variant.is_overpass();
        let gap = if is_overpass { size * 1.00 } else { 0.0 };


        if self.variant.is_overpass() {
            // let side = center + size / 2.0;
            if self.variant.vertical_on_top() {
                draw_rectangle(pos.x + center, pos.y, size, scale, color_a);
                draw_rectangle(pos.x, pos.y + center, scale / 2. - gap, size, color_b);
                draw_rectangle(pos.x + scale / 2. + gap, pos.y + center, scale / 2. - gap, size, color_b);
            } else {
                draw_rectangle(pos.x, pos.y + center, scale, size, color_b);
                draw_rectangle(pos.x + center, pos.y, size, scale / 2. - gap, color_a);
                draw_rectangle(pos.x + center, pos.y + scale / 2. + gap, size, scale / 2. - gap, color_a);
            }
            return;
        }

        draw_rectangle(pos.x + center, pos.y + center, size, size, color_a);

        if self.variant.has_north() {
            let height = if is_overpass {
                center + size / 2.0 - gap
            } else {
                center + size / 2.0
            };
            draw_rectangle(pos.x + center, pos.y, size, height, color_a);
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
            draw_rectangle(pos.x + center, y_start, size, height, color_a);
        }

        if self.variant.has_east() {
            draw_rectangle(
                pos.x + center + size / 2.0,
                pos.y + center,
                center + size / 2.0,
                size,
                color_a,
            );
        }

        if self.variant.has_west() {
            draw_rectangle(pos.x, pos.y + center, center + size / 2.0, size, color_a);
        }
    }
}
