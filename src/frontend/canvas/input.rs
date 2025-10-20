use egui_macroquad::macroquad::prelude::*;
use crate::canvas::components::ComponentData;

use super::camera::GridCamera;

#[derive(PartialEq, Eq, Debug)]
pub enum CanvasInputState {
    Wire,
    Component,
    Select,
    Idle,
}

impl CanvasInputState {
    pub fn new() -> Self {
        Self::Idle
    }

    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Escape) {
            *self = Self::Idle;
        }
        if is_key_down(KeyCode::Q) {
            *self = Self::Wire;
        }
    }
}

// impl CanvasInputState {
//     pub fn handle_input(mut self, camera: &GridCamera) {
//         // Get mouse position and snap to grid
//         let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
//         let mouse_world = camera.screen_to_world(mouse_screen);
//         let end_pos = Vec2::new(mouse_world.x.floor(), mouse_world.y.floor());
//
//         if is_mouse_button_pressed(MouseButton::Left) {
//             if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
//                 if let WireDrawState::StartSelected(start_pos) = self.draw_state {
//                     self.draw_wire_path(start_pos, end_pos);
//                 }
//             }
//             self.draw_state = WireDrawState::StartSelected(end_pos);
//         }
//         if is_mouse_button_pressed(MouseButton::Right) {
//             self.draw_vertical_first = !self.draw_vertical_first;
//         }
//
//         if is_key_pressed(KeyCode::Escape) {
//             self = CanvasInputState::Idle;
//         }
//     }
// }
