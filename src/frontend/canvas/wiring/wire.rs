use std::f32::consts::PI;

use egui_macroquad::macroquad::prelude::*;

use crate::frontend::canvas::camera::GridCamera;

use super::wire_variant::WireVariant;

pub struct Wire {
    pub position: Vec2,
    pub variant: WireVariant,
}

impl Wire {
    pub fn new(position: Vec2, variant: WireVariant) -> Self {
        Self { position, variant }
    }

    #[allow(unused)]
    pub fn draw(&self, _camera: &GridCamera) {
        let width: f32 = 0.2;
        let pos = self.position;
        let grid_size = 1.0;
        let half_grid = 0.5 * grid_size;

        let offset = (grid_size - width) / 2.0;
        let size = width * grid_size;
        let color = GREEN;

        let is_overpass = self.variant.is_overpass();
        let gap = if is_overpass { size * 0.75 } else { 0.0 };

        draw_poly(
            pos.x + half_grid,
            pos.y + half_grid,
            4,
            size/2.0,
            PI/4.0,
            color,
        );

        if self.variant.has_north() {
            let height = if is_overpass {
                half_grid - gap
            } else {
                half_grid
            };
            draw_rectangle(pos.x + offset, pos.y, size, height, color);
        }

        if self.variant.has_south() {
            let y_start = if is_overpass {
                pos.y + half_grid + gap
            } else {
                pos.y + half_grid
            };
            let height = if is_overpass {
                half_grid - gap
            } else {
                half_grid
            };
            draw_rectangle(pos.x + offset, y_start, size, height, color);
        }

        if self.variant.has_east() {
            draw_rectangle(
                pos.x + half_grid,
                pos.y + offset,
                half_grid,
                size,
                color,
            );
        }

        if self.variant.has_west() {
            draw_rectangle(pos.x, pos.y + offset, offset + size / 2.0, size, color);
        }
    }
}
