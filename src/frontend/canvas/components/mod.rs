use std::{collections::HashMap, ops::Neg};
use egui::ahash::HashSet;
use egui_macroquad::macroquad::prelude::*;
use uuid::Uuid;

use crate::{canvas::camera::GridCamera, gui::component_utils::{macroquad_draw_curve, GuiComponentType}};

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
    pub fn get_size(&self) -> (i32, i32) {
        match self {
            Self::Gate { gate_type, .. } => if *gate_type == GateType::Not {
                (3, 1)
            } else {
                (4, 3)
            },
            Self::Mux { .. } => todo!(),
        }
    }

    pub fn get_input_offsets(&self) -> Vec<(i32, i32)> {
        match self {
            Self::Gate { gate_type, .. } => if *gate_type == GateType::Not {
                vec![(0, 0)]
            } else {
                vec![(0, 0), (0, 2)]
            },
            Self::Mux { .. } => todo!(),
        }
    }

    pub fn get_output_offsets(&self) -> Vec<(i32, i32)> {
        match self {
            Self::Gate { gate_type, .. } => if *gate_type == GateType::Not {
                vec![(2, 0)]
            } else {
                vec![(3, 1)]
            }
            Self::Mux { .. } => todo!(),
        }
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

/// Orientation::One = pi / 2 CCW, ...
#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum Orientation {
    #[default]
    Zero,
    One,
    Two,
    Three
}

impl Orientation {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Zero => "North",
            Self::One => "West",
            Self::Two => "South",
            Self::Three => "East",
        }
    }
}

pub fn rotate_point_ccw<T: Neg<Output = T>>((x, y): (T, T), orientation: Orientation) -> (T, T) {
    match orientation {
        Orientation::Zero => (x, y),
        Orientation::One => (-y, x),
        Orientation::Two => (-x, -y),
        Orientation::Three => (y, -x),
    }
}

pub struct Component {
    uuid: Uuid,
    pub orientation: Orientation,
    pub label: String,
    pub data: ComponentData
}

impl Component {
    pub fn intersects_selection(&self, comp_x: i32, comp_y: i32, c: ((i32, i32), (i32, i32))) -> bool {
        let ((x1, y1), (x2, y2)) = c;
        let sel_min_x = x1.min(x2);
        let sel_max_x = x1.max(x2);
        let sel_min_y = y1.min(y2);
        let sel_max_y = y1.max(y2);

        let (w, h) = rotate_point_ccw(self.data.get_size(), self.orientation);
        let old_comp_min_x = comp_x;
        let old_comp_max_x = comp_x + w as i32 - 1;
        let old_comp_min_y = comp_y;
        let old_comp_max_y = comp_y + h as i32 - 1;
        let comp_min_x = old_comp_min_x.min(old_comp_max_x);
        let comp_max_x = old_comp_min_x.max(old_comp_max_x);
        let comp_min_y = old_comp_min_y.min(old_comp_max_y);
        let comp_max_y = old_comp_min_y.max(old_comp_max_y);

            // Check for intersection
        let intersects = sel_min_x <= comp_max_x
            && sel_max_x >= comp_min_x
            && sel_min_y <= comp_max_y
            && sel_max_y >= comp_min_y;

        intersects
    }

    pub fn contains_point(&self, comp_x: i32, comp_y: i32, point: (f32, f32)) -> bool {
        let (w, h) = rotate_point_ccw(self.data.get_size(), self.orientation);
        let x1 = comp_x;
        let x2 = comp_x + w;
        let y1 = comp_y;
        let y2 = comp_y + h;
        let comp_min_x = x1.min(x2);
        let comp_max_x = x1.max(x2);
        let comp_min_y = y1.min(y2);
        let comp_max_y = y1.max(y2);
        comp_min_x as f32 <= point.0 && point.0 <= comp_max_x as f32 && comp_min_y as f32 <= point.1 && point.1 <= comp_max_y as f32
    }
}

#[derive(Default)]
pub struct ComponentSystem {
    components: HashMap<(i32, i32), Component>,
    selection: HashSet<Uuid>,
    drag_delta: (i32, i32),
    drag_handled: bool,
}

impl ComponentSystem {
    pub fn new() -> Self {
        let mut a = Self::default();
        a.components.insert((0, 0), Component {
            uuid: Uuid::new_v4(), 
            orientation: Orientation::Zero,
            label: String::new(),
            data: ComponentData::Gate {
                gate_type: GateType::And,
                bitsize: 32,
            },
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
                Component {
                    uuid: Uuid::new_v4(),
                    orientation: Orientation::Zero,
                    label: String::new(),
                    data: selected_component,
                },
            );
        }
    }

    // TODO: this should be merged with handle_input when CanvasInputState is updated to contain
    // selected_component
    pub fn handle_delete(&mut self) {
        if is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Delete) || is_key_pressed(KeyCode::X) {
            self.components.retain(|_, component| !self.selection.contains(&component.uuid));
        }
    }

    pub fn draw_new_component_preview(&self, camera: &GridCamera, selected_component: ComponentData) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = camera.screen_to_world(mouse_screen);
        let (x, y) = (mouse_world.x.floor() as i32, mouse_world.y.floor() as i32);
        let (x, y) = (x - selected_component.get_size().0 as i32 / 2, y - selected_component.get_size().1 as i32 / 2);
        draw_rectangle(
            x as f32,
            y as f32,
            selected_component.get_size().0 as f32,
            selected_component.get_size().1 as f32,
            ORANGE.with_alpha(0.2),
        );
    }

    pub fn draw_components(&self, camera: &GridCamera) {
        let (view_min, view_max) = camera.get_view_bounds();
        // let stroke_width = camera.get_pixel_thickness() * 4.0;
        let stroke_width = 0.1;
        for ((x, y), component) in &self.components {
            let (w, h) = component.data.get_size();
            // let (w, h) = rotate_point_ccw(component.data.get_size(), component.orientation);
            // return;
            if ((x + w) as f32) < view_min.x
                || (*x as f32) > view_max.x
                || ((y + h) as f32) < view_min.y
                || (*y as f32) > view_max.y

            {
                continue;
            }
            let shifted_x = *x as f32;
            let shifted_y = *y as f32;
            let target_x = shifted_x + if self.selection.contains(&component.uuid) {
                self.drag_delta.0 as f32
            } else {
                0.
            };
            let target_y = shifted_y + if self.selection.contains(&component.uuid) {
                self.drag_delta.1 as f32
            } else {
                0.
            };
            let (rot_w, rot_h) = rotate_point_ccw((w, h), component.orientation);
            if self.drag_delta != (0, 0) && self.selection.contains(&component.uuid) {
                draw_rectangle(
                    shifted_x,
                    shifted_y,
                    rot_w as f32,
                    rot_h as f32,
                    ORANGE.with_alpha(0.2),
                );
            }
            let b_target_x = x + if self.selection.contains(&component.uuid) {
                self.drag_delta.0
            } else { 0 };
            let b_target_y = y + if self.selection.contains(&component.uuid) {
                self.drag_delta.1
            } else { 0 };
            let border_color = if self.selection.contains(&component.uuid) {
                ORANGE
            } else {
                BLACK
            };
            for i in GuiComponentType::AND_GATE_DRAW_INSTRUCTIONS {
                macroquad_draw_curve(
                    i,
                    epaint::Rect::from_two_pos(
                        epaint::Pos2 { x: b_target_x as f32, y: b_target_y as f32 },
                        epaint::Pos2 { x: (b_target_x + w) as f32, y: (b_target_y + h) as f32 },
                    ),
                    stroke_width,
                    border_color,
                    component.orientation
                );
            }
            const PORT_SIZE: f32 = 0.2;
            for (dx, dy) in component.data.get_input_offsets() {
                let (dx, dy) = rotate_point_ccw((dx as f32 + 0.5, dy as f32 + 0.5), component.orientation);
                draw_circle(
                    target_x + dx,
                    target_y + dy,
                    PORT_SIZE,
                    BLUE
                );
            }
            for (dx, dy) in component.data.get_output_offsets() {
                let (dx, dy) = rotate_point_ccw((dx as f32 + 0.5, dy as f32 + 0.5), component.orientation);
                draw_circle(
                    target_x + dx,
                    target_y + dy,
                    PORT_SIZE,
                    BLUE
                );
                draw_circle(
                    target_x + dx as f32,
                    target_y + dy as f32,
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
                for (coords, component) in self.components.iter() {
                    if self.selection.contains(&component.uuid) {
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
                for (&(comp_x, comp_y), component) in self.components.iter_mut() {
                    if component.intersects_selection(comp_x, comp_y, c) {
                        self.selection.insert(component.uuid);
                    }
                }
            }
        } else if let Some((start, end)) = in_progress_selection {
            for (&(comp_x, comp_y), component) in self.components.iter_mut() {
                if self.selection.contains(&component.uuid) && component.contains_point(comp_x, comp_y, start) {
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
    ) -> Vec<&mut Component> {
        let mut selected = Vec::new();

        for (_, component) in self.components.iter_mut() {
            if self.selection.contains(&component.uuid) {
                selected.push(component);
            }
        }

        selected
    }
}
