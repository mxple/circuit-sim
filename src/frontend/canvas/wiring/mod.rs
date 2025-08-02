use egui_macroquad::macroquad::prelude::*;
use std::collections::HashMap;

use wire::{Wire, WireVariant};

use super::camera::Camera;

mod wire;

#[derive(Debug, Clone, Copy)]
enum WireDrawState {
    Idle,
    StartSelected(Vec2), // Starting point selected
}

pub struct WireSystem {
    wires: HashMap<(i32, i32), Wire>,
    draw_state: WireDrawState,
}

impl WireSystem {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
            draw_state: WireDrawState::Idle,
        }
    }

    pub fn handle_input(&mut self, camera: &Camera) {
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
            if corner != start { // Don't double-add corner
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
            let prev_pos = if i > 0 { Some(path_positions[i - 1]) } else { None };
            let next_pos = if i < path_positions.len() - 1 { Some(path_positions[i + 1]) } else { None };

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

    fn calculate_wire_variant(&self, current: (i32, i32), prev: Option<(i32, i32)>, next: Option<(i32, i32)>) -> WireVariant {
        let mut north = false;
        let mut east = false;
        let mut south = false;
        let mut west = false;

        // Check connections to previous position
        if let Some(prev_pos) = prev {
            if prev_pos.1 < current.1 { north = true; }
            if prev_pos.0 > current.0 { east = true; }
            if prev_pos.1 > current.1 { south = true; }
            if prev_pos.0 < current.0 { west = true; }
        }

        // Check connections to next position
        if let Some(next_pos) = next {
            if next_pos.1 < current.1 { north = true; }
            if next_pos.0 > current.0 { east = true; }
            if next_pos.1 > current.1 { south = true; }
            if next_pos.0 < current.0 { west = true; }
        }

        WireVariant::new(north, east, south, west, false)
    }

    pub fn draw_wires(&self, camera: &Camera) {
        let (view_min, view_max) = camera.get_view_bounds();
        
        for wire in self.wires.values() {
            if wire.position.x >= view_min.x - 1.0 && wire.position.x <= view_max.x + 1.0 &&
               wire.position.y >= view_min.y - 1.0 && wire.position.y <= view_max.y + 1.0 {
                wire.draw(camera);
            }
        }
    }

    pub fn draw_preview(&self, camera: &Camera) {
        if let WireDrawState::StartSelected(start_pos) = self.draw_state {
            // Draw start position highlight
            let start_screen = camera.world_to_screen(start_pos + vec2(0.5, 0.5));
            let size = camera.zoom * 0.9;
            draw_rectangle_lines(
                start_screen.x - size / 2.0,
                start_screen.y - size / 2.0,
                size,
                size,
                3.0,
                GREEN,
            );

            // Draw preview path if shift is held
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
                let mouse_world = camera.screen_to_world(mouse_screen);
                let end_pos = Vec2::new(mouse_world.x.floor(), mouse_world.y.floor());

                self.draw_preview_path(start_pos, end_pos, camera);
            }
        }
    }

    fn draw_preview_path(&self, start: Vec2, end: Vec2, camera: &Camera) {
        if start == end {
            return;
        }

        // Draw preview L-shaped path
        let corner = Vec2::new(start.x, end.y);
        
        // Vertical line preview
        if start.y != corner.y {
            let line_start = camera.world_to_screen(start + vec2(0.5, 0.5));
            let line_end = camera.world_to_screen(corner + vec2(0.5, 0.5));
            draw_line(line_start.x, line_start.y, line_end.x, line_end.y, 2.0, Color::new(0.0, 1.0, 0.0, 0.6));
        }

        // Horizontal line preview
        if corner.x != end.x {
            let line_start = camera.world_to_screen(corner + vec2(0.5, 0.5));
            let line_end = camera.world_to_screen(end + vec2(0.5, 0.5));
            draw_line(line_start.x, line_start.y, line_end.x, line_end.y, 2.0, Color::new(0.0, 1.0, 0.0, 0.6));
        }

        // Draw end position highlight
        let end_screen = camera.world_to_screen(end + vec2(0.5, 0.5));
        let size = camera.zoom * 0.9;
        draw_rectangle_lines(
            end_screen.x - size / 2.0,
            end_screen.y - size / 2.0,
            size,
            size,
            3.0,
            Color::new(0.0, 1.0, 0.0, 0.8),
        );
    }
}
