use std::collections::HashMap;
use egui::ahash::HashSet;
use egui_macroquad::macroquad::prelude::*;
use uuid::Uuid;

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

impl GateType {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::Nand => "NAND",
            Self::Nor => "NOR",
            Self::Xor => "XOR",
            Self::Xnor => "XNOR",
            Self::Not => "NOT",
        }
    }
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
            Self::Gate { gate_type, .. } => if *gate_type == GateType::Not {
                (3, 1)
            } else {
                (4, 3)
            },
            Self::Mux { .. } => todo!(),
        }
    }

    pub fn get_input_offsets(&self) -> Vec<(u32, u32)> {
        match self {
            Self::Gate { gate_type, .. } => if *gate_type == GateType::Not {
                vec![(0, 0)]
            } else {
                vec![(0, 0), (0, 2)]
            },
            Self::Mux { .. } => todo!(),
        }
    }

    pub fn get_output_offsets(&self) -> Vec<(u32, u32)> {
        match self {
            Self::Gate { gate_type, .. } => if *gate_type == GateType::Not {
                vec![(2, 0)]
            } else {
                vec![(3, 1)]
            }
            Self::Mux { .. } => todo!(),
        }
    }

    pub fn contains_point(&self, comp_x: i32, comp_y: i32, point: (f32, f32)) -> bool {
        let (w, h) = self.get_size();
        comp_x as f32 <= point.0 && point.0 <= (comp_x + w as i32) as f32 && comp_y as f32 <= point.1 && point.1 <= (comp_y + h as i32) as f32
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

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Gate { gate_type, .. } => {
                match gate_type {
                    GateType::And => "AND gate",
                    GateType::Or => "OR gate",
                    GateType::Nand => "NAND gate",
                    GateType::Nor => "NOR gate",
                    GateType::Xor => "XOR gate",
                    GateType::Xnor => "XNOR gate",
                    GateType::Not => "NOT gate",
                }
            }
            Self::Mux { .. } => "Multiplexer",
        }
    }
}

#[derive(Default)]
pub struct ComponentSystem {
    components: HashMap<(i32, i32), (Uuid, ComponentData)>,
    selection: HashSet<Uuid>,
    drag_delta: (i32, i32),
    drag_handled: bool,
}

impl ComponentSystem {
    pub fn new() -> Self {
        let mut a = Self::default();
        a.components.insert((0, 0), (Uuid::new_v4(), ComponentData::Gate {
            gate_type: GateType::And,
            bitsize: 32,
        }));
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
                (Uuid::new_v4(), selected_component),
            );
        }
    }

    // TODO: this should be merged with handle_input when CanvasInputState is updated to contain
    // selected_component
    pub fn handle_delete(&mut self) {
        if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete) || is_key_pressed(KeyCode::X) {
            self.components.retain(|_, (uuid, _)| !self.selection.contains(uuid));
        }
    }

    pub fn draw_new_component_preview(&self, camera: &GridCamera, selected_component: ComponentData) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let (x, y) = (mouse_world.x.floor() as i32, mouse_world.y.floor() as i32);
        let (x, y) = (x - selected_component.get_size().0 as i32 / 2, y - selected_component.get_size().1 as i32 / 2);
        draw_rectangle(
            x as f32 + 0.5,
            y as f32,
            selected_component.get_size().0 as f32 - 1.,
            selected_component.get_size().1 as f32,
            ORANGE.with_alpha(0.2),
        );
    }

    pub fn draw_components(&self, camera: &GridCamera) {
        let (view_min, view_max) = camera.get_view_bounds();
        for ((x, y), (uuid, component)) in &self.components {
            let (w, h) = component.get_size();
            if ((x + w as i32) as f32) < view_min.x
                || (*x as f32) > view_max.x
                || ((y + h as i32) as f32) < view_min.y
                || (*y as f32) > view_max.y

            {
                continue;
            }
            let shifted_x = *x as f32 + 0.5;
            let shifted_y = *y as f32;
            let target_x = shifted_x + if self.selection.contains(uuid) {
                self.drag_delta.0 as f32
            } else {
                0.
            };
            let target_y = shifted_y + if self.selection.contains(uuid) {
                self.drag_delta.1 as f32
            } else {
                0.
            };
            if self.drag_delta != (0, 0) && self.selection.contains(uuid) {
                draw_rectangle(
                    shifted_x,
                    shifted_y,
                    w as f32 - 1.,
                    h as f32,
                    ORANGE.with_alpha(0.2),
                );
            }
            draw_rectangle(
                target_x,
                target_y,
                w as f32 - 1.,
                h as f32,
                WHITE,
            );
            let border_color = if self.selection.contains(uuid) {
                ORANGE
            } else {
                BLACK
            };
            draw_rectangle_lines(
                target_x,
                target_y,
                w as f32 - 1.,
                h as f32,
                camera.get_pixel_thickness() * 4.0,
                border_color
            );
            const PORT_SIZE: f32 = 0.2;
            for (dx, dy) in &component.get_input_offsets() {
                draw_circle(
                    target_x + *dx as f32,
                    target_y + *dy as f32 + 0.5,
                    PORT_SIZE,
                    BLUE
                );
            }
            for (dx, dy) in &component.get_output_offsets() {
                draw_circle(
                    target_x + *dx as f32,
                    target_y + *dy as f32 + 0.5,
                    PORT_SIZE,
                    BLUE
                );
            }
        }
    }

    /// Returns false if selection preview should not be drawn, true otherwise.
    pub fn update_selection(
        &mut self,
        in_progress_selection: Option<((f32, f32), (f32, f32))>,
        selection: Option<((i32, i32), (i32, i32))>,
    ) -> bool {
        if in_progress_selection.is_some() {
            self.drag_handled = false;
        }
        if let Some(c) = selection {
            if self.drag_delta != (0, 0) {
                let mut to_move = Vec::new();
                for (coords, (uuid, _)) in self.components.iter() {
                    if self.selection.contains(&uuid) {
                        to_move.push(coords.clone());
                    }
                }
                for (x, y) in to_move {
                    if let Some(v) = self.components.remove(&(x, y)) {
                    self.components.insert((x + self.drag_delta.0, y + self.drag_delta.1), v);
                    }
                }
                self.drag_delta = (0, 0);
                self.drag_handled = true;
            } else if !self.drag_handled {
                for (&(comp_x, comp_y), (uuid, component)) in self.components.iter_mut() {
                    if component.intersects_selection(comp_x, comp_y, c) {
                        self.selection.insert(*uuid);
                    }
                }
            }
        } else if let Some((start, end)) = in_progress_selection {
            for (&(comp_x, comp_y), (uuid, component)) in self.components.iter_mut() {
                if self.selection.contains(uuid) && component.contains_point(comp_x, comp_y, start) {
                    self.drag_delta = (
                        (end.0 - start.0) as i32,
                        (end.1 - start.1) as i32,
                    );
                    return false;
                }
            }
            self.selection.clear();
        } else {
            self.selection.clear();
            self.drag_handled = false;
        }
        return true;
    }

    pub fn get_selection_mut(
        &mut self,
    ) -> Vec<&mut ComponentData> {
        let mut selected = Vec::new();

        for (_, (uuid, component)) in self.components.iter_mut() {
            if self.selection.contains(uuid) {
                selected.push(component);
            }
        }

        selected
    }
}
