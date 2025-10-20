use egui_macroquad::macroquad::prelude::*;

use crate::canvas::camera::GridCamera;

// TODO: move canvas selection and selected component into this enum
#[derive(PartialEq, Eq, Debug)]
pub enum CanvasInputState {
    Wire,
    Component,
    Idle,
}

pub struct CanvasInput {
    pub state: CanvasInputState,
    pub selection: Option<((i32, i32), (i32, i32))>,
    pub in_progress_selection: Option<((f32, f32), (f32, f32))>,
}

impl CanvasInput {
    pub fn new() -> Self {
        Self {
            state: CanvasInputState::Idle,
            selection: None,
            in_progress_selection: None,
        }
    }

    pub fn handle_input(&mut self, camera: &GridCamera, egui_wants_ptr: bool) {
        if is_key_down(KeyCode::Escape) {
            self.state = CanvasInputState::Idle;
            self.in_progress_selection = None;
            self.selection = None;
        }
        if self.state != CanvasInputState::Idle {
            self.in_progress_selection = None;
            self.selection = None;
        }
        if is_key_down(KeyCode::Q) {
            self.state = CanvasInputState::Wire;
        }

        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let end_pos = (mouse_world.x, mouse_world.y);

        // Mouse release events should be processed even if the pointer is over an egui frame
        if is_mouse_button_released(MouseButton::Left) && let Some(((x1, y1), (x2, y2))) = self.in_progress_selection {
            let min_x = x1.min(x2).floor() as i32;
            let min_y = y1.min(y2).floor() as i32;
            let max_x = x1.max(x2).ceil() as i32;
            let max_y = y1.max(y2).ceil() as i32;

            self.selection = Some( ((min_x, min_y), (max_x, max_y))
            );
            self.in_progress_selection = None;
        }
        // So should mouse drag events
        if is_mouse_button_down(MouseButton::Left) && self.in_progress_selection.is_some() {
            let Some((c1, _)) = self.in_progress_selection else {
                return;
            };
            self.in_progress_selection = Some((c1, end_pos));
        } 
        if egui_wants_ptr {
            return;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            self.selection = None;
            self.in_progress_selection = Some((end_pos, end_pos))
        } 
    }

    pub fn draw_selection(&self, camera: &GridCamera) {
        if let Some((c1, c2)) = self.in_progress_selection {
            draw_rectangle(
                c1.0 as f32,
                c1.1 as f32,
                c2.0 - c1.0,
                c2.1 - c1.1,
                BLUE.with_alpha(0.2),
            );
        } else if let Some((c1, c2)) = self.selection {
            draw_rectangle(
                c1.0 as f32,
                c1.1 as f32,
                c2.0 as f32 - c1.0 as f32,
                c2.1 as f32 - c1.1 as f32,
                BLUE.with_alpha(0.05),
            );
            draw_rectangle_lines(
                c1.0 as f32,
                c1.1 as f32,
                c2.0 as f32 - c1.0 as f32,
                c2.1 as f32 - c1.1 as f32,
                camera.get_pixel_thickness() * 4.0,
                BLUE
            );
            // let _  =((c1.0 as f32, c1.1 as f32), (c2.0 as f32, c2.1 as f32));
        } else {
            return
        };
        // draw_rectangle(
        //     c1.0 as f32,
        //     c1.1 as f32,
        //     c2.0 - c1.0,
        //     c2.1 - c1.1,
        //     BLUE.with_alpha(0.2),
        // );
    }
}
