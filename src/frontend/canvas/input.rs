use egui_macroquad::macroquad::prelude::*;

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
