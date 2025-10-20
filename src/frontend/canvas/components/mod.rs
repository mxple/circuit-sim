use std::collections::HashMap;
use egui_macroquad::macroquad::prelude::*;

use crate::{canvas::camera::GridCamera, gui::component_utils::GateType};

pub enum ComponentData {
    Gate {
        gate_type: GateType,
        bitsize: u8,
    },
    Mux {
        bitsize: u8,
    },
}

pub struct Component {
    /// Component-specific data
    data: ComponentData,
    /// Size of the component (width, height)
    size: (u32, u32),
    /// Offsets of input pins from top left
    input_offsets: Vec<(u32, u32)>,
}

#[derive(Default)]
pub struct ComponentSystem {
    components: HashMap<(i32, i32), Component>,
}

impl ComponentSystem {
    pub fn new() -> Self {
        let mut a = Self::default();
        a.components.insert((0, 0), Component {
            data: ComponentData::Gate {
                gate_type: GateType::And,
                bitsize: 32,
            },
            size: (3, 3),
            input_offsets: vec![
                (0, 0),
                (0, 2),
            ],
        });
        a
    }

    pub fn handle_input(&mut self, camera:  &GridCamera, selected_component: GateType) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let end_pos = (mouse_world.x.floor() as i32, mouse_world.y.floor() as i32);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.components.insert(
                end_pos,
                Component {
                    data: ComponentData::Gate {
                        gate_type: selected_component,
                        bitsize: 1,
                    },
                    size: (3, 3),
                    input_offsets: vec![(0, 0), (0, 2)]
                }
            );
        }
    }

    pub fn draw_components(&self, camera: &GridCamera) {
        // TODO: cull components outside camera boundaries
        // let (view_min, view_max) = camera.get_view_bounds();
        for ((x, y), component) in &self.components {
            draw_rectangle(
                *x as f32 + 0.5,
                *y as f32,
                component.size.0 as f32,
                component.size.1 as f32,
                WHITE
            );
            draw_rectangle_lines(
                *x as f32 + 0.5,
                *y as f32,
                component.size.0 as f32,
                component.size.1 as f32,
                0.1,
                BLACK
            );
            const PORT_SIZE: f32 = 0.2;
            for (dx, dy) in &component.input_offsets {
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
