use egui_macroquad::macroquad::prelude::*;
use wire_mesh::{apply_transform, generate_wire_meshes, WireMesh};
use std::collections::HashMap;
use wire_variant::WireVariant;

use wire::Wire;

use super::camera::GridCamera;

mod wire;
mod wire_mesh;
mod wire_variant;

#[derive(Debug, Clone, Copy)]
enum WireDrawState {
    Idle,
    StartSelected(Vec2), // Starting point selected
}

pub struct WireSystem {
    wires: HashMap<(i32, i32), Wire>,
    draw_state: WireDrawState,
    wire_meshes: Vec<WireMesh>,
}

impl WireSystem {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
            draw_state: WireDrawState::Idle,
            wire_meshes: generate_wire_meshes(),
        }
    }

    pub fn handle_input(&mut self, camera: &GridCamera) {
        // Get mouse position and snap to grid
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let end_pos = Vec2::new(mouse_world.x.floor(), mouse_world.y.floor());

        if is_mouse_button_pressed(MouseButton::Left) {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                if let WireDrawState::StartSelected(start_pos) = self.draw_state {
                    self.draw_wire_path(start_pos, end_pos);
                    self.draw_state = WireDrawState::Idle;
                }
            } else {
                self.draw_state = WireDrawState::StartSelected(end_pos);
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            self.draw_state = WireDrawState::Idle;
        }
    }

    fn draw_wire_path(&mut self, start: Vec2, end: Vec2) {
        if start == end {
            self.place_single_wire(start);
            return;
        }

        let start_i = (start.x as i32, start.y as i32);
        let end_i = (end.x as i32, end.y as i32);

        // Create L-shaped path: start -> corner -> end
        // Path goes: start -> (start.x, end.y) -> end
        let corner = Vec2::new(start.x, end.y);
        let corner_i = (corner.x as i32, corner.y as i32);

        let mut path_positions = Vec::new();

        // Vertical segment: start to corner
        if start.y != corner.y {
            let y_step = if corner.y > start.y { 1 } else { -1 };
            let mut current_y = start.y as i32;
            while current_y != corner.y as i32 {
                path_positions.push((start.x as i32, current_y));
                current_y += y_step;
            }
        }

        // Add corner position
        if corner != start && corner != end {
            path_positions.push(corner_i);
        }

        // Horizontal segment: corner to end
        if corner.x != end.x {
            let x_step = if end.x > corner.x { 1 } else { -1 };
            let mut current_x = corner.x as i32;
            if corner != start {
                // Don't double-add corner
                current_x += x_step;
            }
            while current_x != end.x as i32 + x_step {
                path_positions.push((current_x, end.y as i32));
                current_x += x_step;
            }
        }

        // Always include start and end
        if !path_positions.contains(&start_i) {
            path_positions.insert(0, start_i);
        }
        if !path_positions.contains(&end_i) {
            path_positions.push(end_i);
        }

        // Place wires with correct connections
        for (i, &pos) in path_positions.iter().enumerate() {
            let prev_pos = if i > 0 {
                Some(path_positions[i - 1])
            } else {
                None
            };
            let next_pos = if i < path_positions.len() - 1 {
                Some(path_positions[i + 1])
            } else {
                None
            };

            let variant = self.calculate_wire_variant(pos, prev_pos, next_pos);
            if let Some(existing_wire) = self.wires.get_mut(&pos) {
                // Merge the connections
                existing_wire.variant = existing_wire.variant.merge_with(&variant);
            } else {
                // Place new wire
                let wire = Wire::new(Vec2::new(pos.0 as f32, pos.1 as f32), variant);
                self.wires.insert(pos, wire);
            }
        }
    }

    fn place_single_wire(&mut self, position: Vec2) {
        let grid_key = (position.x as i32, position.y as i32);
        let variant = WireVariant::new(false, false, false, false, false); // No connections
        let wire = Wire::new(position, variant);
        self.wires.insert(grid_key, wire);
    }

    fn calculate_wire_variant(
        &self,
        current: (i32, i32),
        prev: Option<(i32, i32)>,
        next: Option<(i32, i32)>,
    ) -> WireVariant {
        let mut north = false;
        let mut east = false;
        let mut south = false;
        let mut west = false;

        // Check connections to previous position
        if let Some(prev_pos) = prev {
            if prev_pos.1 < current.1 {
                north = true;
            }
            if prev_pos.0 > current.0 {
                east = true;
            }
            if prev_pos.1 > current.1 {
                south = true;
            }
            if prev_pos.0 < current.0 {
                west = true;
            }
        }

        // Check connections to next position
        if let Some(next_pos) = next {
            if next_pos.1 < current.1 {
                north = true;
            }
            if next_pos.0 > current.0 {
                east = true;
            }
            if next_pos.1 > current.1 {
                south = true;
            }
            if next_pos.0 < current.0 {
                west = true;
            }
        }

        WireVariant::new(north, east, south, west, false)
    }

    #[allow(unused)]
    pub fn draw_wires(&self, camera: &GridCamera) {
        let (view_min, view_max) = camera.get_view_bounds();

        for wire in self.wires.values() {
            if wire.position.x >= view_min.x - 1.0
                && wire.position.x <= view_max.x + 1.0
                && wire.position.y >= view_min.y - 1.0
                && wire.position.y <= view_max.y + 1.0
            {
                wire.draw(camera);
            }
        }
    }

    #[allow(unused)]
    pub fn draw_wires_meshed(&self, camera: &GridCamera) {
        let (view_min, view_max) = camera.get_view_bounds();

        for wire in self.wires.values() {
            if wire.position.x >= view_min.x - 1.0
                && wire.position.x <= view_max.x + 1.0
                && wire.position.y >= view_min.y - 1.0
                && wire.position.y <= view_max.y + 1.0
            {
                let mesh = &self.wire_meshes[wire.variant.0 as usize];
                let transformed_mesh = apply_transform(mesh, wire.position);
                draw_mesh(&transformed_mesh.mesh);
            }
        }
    }
    pub fn draw_preview(&self, camera: &GridCamera) {
        if let WireDrawState::StartSelected(start_pos) = self.draw_state {
            let size = 0.875;
            let rect_x = start_pos.x + (1.0 - size) / 2.0;
            let rect_y = start_pos.y + (1.0 - size) / 2.0;

            draw_rectangle_lines(
                rect_x,
                rect_y,
                size,
                size,
                camera.get_pixel_thickness() * 2.0,
                GREEN,
            );

            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
                let mouse_world = camera.screen_to_world(mouse_screen);
                let end_pos = Vec2::new(mouse_world.x.floor(), mouse_world.y.floor());

                self.draw_preview_path(start_pos, end_pos, camera);
            }
        }
    }

    fn draw_preview_path(&self, start: Vec2, end: Vec2, camera: &GridCamera) {
        if start == end {
            return;
        }

        let width = camera.get_pixel_thickness();
        let corner = Vec2::new(start.x, end.y);

        if start.y != corner.y {
            let line_start = start + vec2(0.5, 0.5);
            let line_end = corner + vec2(0.5, 0.5);
            draw_line(
                line_start.x,
                line_start.y,
                line_end.x,
                line_end.y,
                width,
                Color::new(0.0, 1.0, 0.0, 0.6),
            );
        }

        if corner.x != end.x {
            let line_start = corner + vec2(0.5, 0.5);
            let line_end = end + vec2(0.5, 0.5);
            draw_line(
                line_start.x,
                line_start.y,
                line_end.x,
                line_end.y,
                width,
                Color::new(0.0, 1.0, 0.0, 0.6),
            );
        }

        let size = 0.9;
        let end_center = end + vec2(0.5, 0.5);
        draw_rectangle_lines(
            end_center.x - size / 2.0,
            end_center.y - size / 2.0,
            size,
            size,
            width * 2.0,
            Color::new(0.0, 1.0, 0.0, 0.8),
        );
    }
}
