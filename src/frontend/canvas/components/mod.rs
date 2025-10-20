use std::collections::HashMap;
use egui_macroquad::macroquad::prelude::*;

use crate::canvas::camera::GridCamera;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GateType {
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Xnor,
    Not,
}

pub enum ComponentData {
    Gate {
        gate_type: GateType,
        bitsize: u8,
    },
    Mux {
        bitsize: u8,
    },
}

impl ComponentData {
    pub fn get_size(&self) -> (u32, u32) {
        match self {
            Self::Gate { .. } => (3, 3),
            Self::Mux { .. } => todo!(),
        }
    }

    pub fn get_input_offsets(&self) -> Vec<(u32, u32)> {
        match self {
            Self::Gate { .. } => vec![(0, 0), (0, 2)],
            Self::Mux { .. } => todo!(),
        }
    }
}

#[derive(Default)]
pub struct ComponentSystem {
    components: HashMap<(i32, i32), ComponentData>,
}

impl ComponentSystem {
    pub fn new() -> Self {
        let mut a = Self::default();
        a.components.insert((0, 0), ComponentData::Gate {
            gate_type: GateType::And,
            bitsize: 32,
        });
        a
    }

    pub fn handle_input(&mut self, camera:  &GridCamera, selected_component: ComponentData) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let end_pos = (mouse_world.x.floor() as i32, mouse_world.y.floor() as i32);
        let end_pos = (end_pos.0 - selected_component.get_size().0 as i32 / 2, end_pos.1 - selected_component.get_size().1 as i32 / 2);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.components.insert(
                end_pos,
                selected_component,
            );
        }
    }

    pub fn draw_preview(&self, camera: &GridCamera, selected_component: ComponentData) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let (x, y) = (mouse_world.x.floor() as i32, mouse_world.y.floor() as i32);
        let (x, y) = (x - selected_component.get_size().0 as i32 / 2, y - selected_component.get_size().1 as i32 / 2);
        draw_rectangle_lines(
            x as f32 + 0.5,
            y as f32,
            selected_component.get_size().0 as f32,
            selected_component.get_size().1 as f32,
            0.1,
            GREEN,
        );
    }

    pub fn draw_components(&self, camera: &GridCamera) {
        // TODO: cull components outside camera boundaries
        // let (view_min, view_max) = camera.get_view_bounds();
        for ((x, y), component) in &self.components {
            draw_rectangle(
                *x as f32 + 0.5,
                *y as f32,
                component.get_size().0 as f32,
                component.get_size().1 as f32,
                WHITE
            );
            draw_rectangle_lines(
                *x as f32 + 0.5,
                *y as f32,
                component.get_size().0 as f32,
                component.get_size().1 as f32,
                0.1,
                BLACK
            );
            const PORT_SIZE: f32 = 0.2;
            for (dx, dy) in &component.get_input_offsets() {
                draw_circle(
                    (*x + *dx as i32) as f32 + 0.5,
                    (*y + *dy as i32) as f32 + 0.5,
                    PORT_SIZE,
                    BLUE
                );
                // draw_rectangle(
                //     (*x + *dx as i32) as f32 + 0.5 - PORT_SIZE / 2.,
                //     (*y + *dy as i32) as f32 + 0.5 - PORT_SIZE / 2.,
                //     PORT_SIZE,
                //     PORT_SIZE,
                //     BLUE
                // );
            }
        }
    }
}
