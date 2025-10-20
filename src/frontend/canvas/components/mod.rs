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

    pub fn intersects_selection(&self, comp_x: i32, comp_y: i32, c: ((i32, i32), (i32, i32))) -> bool {
        let ((x1, y1), (x2, y2)) = c;
        let sel_min_x = x1.min(x2);
        let sel_max_x = x1.max(x2);
        let sel_min_y = y1.min(y2);
        let sel_max_y = y1.max(y2);

        let (w, h) = self.get_size();
        let comp_min_x = comp_x;
        let comp_max_x = comp_x + w as i32 - 1;
        let comp_min_y = comp_y;
        let comp_max_y = comp_y + h as i32 - 1;

            // Check for intersection
        let intersects = sel_min_x <= comp_max_x
            && sel_max_x >= comp_min_x
            && sel_min_y <= comp_max_y
            && sel_max_y >= comp_min_y;

        intersects
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
        draw_rectangle(
            x as f32 + 0.5,
            y as f32,
            selected_component.get_size().0 as f32,
            selected_component.get_size().1 as f32,
            PURPLE.with_alpha(0.2),
        );
    }

    pub fn draw_components(&self, camera: &GridCamera, selection: Option<((i32, i32), (i32, i32))>) {
        // TODO: cull components outside camera boundaries
        let (view_min, view_max) = camera.get_view_bounds();
        for ((x, y), component) in &self.components {
            let (w, h) = component.get_size();
            if ((x + w as i32) as f32) < view_min.x
                || (*x as f32) > view_max.x
                || ((y + h as i32) as f32) < view_min.y
                || (*y as f32) > view_max.y

            {
            }
            draw_rectangle(
                *x as f32 + 0.5,
                *y as f32,
                w as f32,
                h as f32,
                WHITE
            );
            let border_color = if let Some(c) = selection && component.intersects_selection(*x, *y, c) {
                ORANGE
            } else {
                BLACK
            };
            draw_rectangle_lines(
                *x as f32 + 0.5,
                *y as f32,
                component.get_size().0 as f32,
                component.get_size().1 as f32,
                camera.get_pixel_thickness() * 4.0,
                border_color
            );
            const PORT_SIZE: f32 = 0.2;
            for (dx, dy) in &component.get_input_offsets() {
                draw_circle(
                    (*x + *dx as i32) as f32 + 0.5,
                    (*y + *dy as i32) as f32 + 0.5,
                    PORT_SIZE,
                    BLUE
                );
                // draw_circle_lines(
                //     (*x + *dx as i32) as f32 + 0.5,
                //     (*y + *dy as i32) as f32 + 0.5,
                //     PORT_SIZE,
                //     camera.get_pixel_thickness() * 2.0,
                //     BLACK,
                // );
            }
        }
    }

    pub fn get_selection_mut(
        &mut self,
        selection: Option<((i32, i32), (i32, i32))>,
    ) -> Vec<&mut ComponentData> {
        let mut selected = Vec::new();

        if let Some(c) = selection {
            for (&(comp_x, comp_y), component) in self.components.iter_mut() {
                if component.intersects_selection(comp_x, comp_y, c) {
                    selected.push(component);
                }
            }
        }

        selected
    }
}
