use macroquad::prelude::*;
use super::GridDrawImpl;

use crate::canvas::camera::GridCamera;

pub struct NaiveDrawer;

impl NaiveDrawer {
    pub fn new() -> NaiveDrawer {
        Self {}
    }
}

impl GridDrawImpl for NaiveDrawer {
    fn draw_grid(&self, camera: &GridCamera, color: &Vec4) {
        let (view_min, view_max) = camera.get_view_bounds();

        let grid_spacing = 1.0; // Adjust this for your desired grid spacing

        // Calculate grid lines that are visible
        let grid_start_x = (view_min.x / grid_spacing).floor() * grid_spacing;
        let grid_end_x = (view_max.x / grid_spacing).ceil() * grid_spacing;
        let grid_start_y = (view_min.y / grid_spacing).floor() * grid_spacing;
        let grid_end_y = (view_max.y / grid_spacing).ceil() * grid_spacing;

        // let line_thickness = 1.0 / camera.zoom;
        let line_thickness = camera.get_pixel_thickness();

        // Draw vertical lines
        let mut x = grid_start_x;
        while x <= grid_end_x {
            draw_line(x, grid_start_y, x, grid_end_y, line_thickness, Color::from(color.to_array()));
            x += grid_spacing;
        }

        // Draw horizontal lines
        let mut y = grid_start_y;
        while y <= grid_end_y {
            draw_line(grid_start_x, y, grid_end_x, y, line_thickness, Color::from(color.to_array()));
            y += grid_spacing;
        }
    }
}
