use egui_macroquad::egui::{Color32, Stroke, Sense, Response, Ui};
use epaint::{Pos2, CubicBezierShape};
use crate::App;

enum DrawInstruction {
    Line([Pos2; 2]),
    CubicBezierCurve([Pos2; 4]),
}

fn pos2_with_rect(pos: &Pos2, rect: egui::Rect) -> Pos2 {
    Pos2 {
        x: rect.min.x + pos.x * rect.width(),
        y: rect.min.y + pos.y * rect.height(),
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CircuitComponentType {
    AndGate,
    OrGate,
    NandGate,
    NorGate,
    XorGate,
    XnorGate,
    NotGate,
}

impl CircuitComponentType {
    fn get_label(&self) -> &'static str {
        match self {
            Self::AndGate => "AND Gate",
            Self::OrGate => "OR Gate",
            Self::NandGate => "NAND Gate",
            Self::NorGate => "NOR Gate",
            Self::XorGate => "XOR Gate",
            Self::XnorGate => "XNOR Gate",
            Self::NotGate => "NOT Gate",
        }
    }

    const UNIMPLEMENTED_DRAW_INSTRUCTIONS: [DrawInstruction; 0] = [];
    const AND_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 4] = {
        const BOX_WIDTH: f32 = 0.5;
        const OFFSET_Y: f32 = 0.1;
        [
            DrawInstruction::Line([Pos2::new(0.0, OFFSET_Y), Pos2::new(0.0, 1.0 - OFFSET_Y)]),
            DrawInstruction::Line([Pos2::new(0.0, OFFSET_Y), Pos2::new(0.5, OFFSET_Y)]),
            DrawInstruction::Line([Pos2::new(0.0, 1.0 - OFFSET_Y), Pos2::new(BOX_WIDTH, 1.0 - OFFSET_Y)]),
            DrawInstruction::CubicBezierCurve([
                Pos2::new(BOX_WIDTH, 1.0 - OFFSET_Y),
                Pos2::new(1.0, 1.0 - OFFSET_Y),
                Pos2::new(1.0, OFFSET_Y),
                Pos2::new(BOX_WIDTH, OFFSET_Y)
            ]),
        ]
    };
    const OR_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 3] = {
        const OFFSET_Y: f32 = 0.1;
        const ARC_START_X: f32 = 0.0;
        const ARC_END_X: f32 = 1.0;
        [
            DrawInstruction::CubicBezierCurve([
                Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
                Pos2::new(ARC_START_X + 0.3, 1.0 - OFFSET_Y),
                Pos2::new(ARC_START_X + 0.3, OFFSET_Y),
                Pos2::new(ARC_START_X, OFFSET_Y),
            ]),
            DrawInstruction::CubicBezierCurve([
                Pos2::new(ARC_START_X, OFFSET_Y),
                Pos2::new(0.55, OFFSET_Y),
                Pos2::new(0.8, OFFSET_Y),
                Pos2::new(ARC_END_X, 0.5),
            ]),
            DrawInstruction::CubicBezierCurve([
                Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
                Pos2::new(0.55, 1.0 - OFFSET_Y),
                Pos2::new(0.8, 1.0 - OFFSET_Y),
                Pos2::new(ARC_END_X, 0.5),
            ]),
        ]
    };

    
    fn get_draw_instructions(&self) -> &'static [DrawInstruction] {
        match self {
            Self::AndGate => &Self::AND_GATE_DRAW_INSTRUCTIONS,
            Self::OrGate => &Self::OR_GATE_DRAW_INSTRUCTIONS,
            Self::NandGate => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::NorGate => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::XorGate => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::XnorGate => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::NotGate => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
        }
    }
}

impl App {
    pub fn circuit_component_button(
        &mut self,
        ui: &mut Ui,
        size: egui::Vec2,
        component_type: CircuitComponentType,
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let inside_margin = rect.shrink(10.0);
            let mut inner_rect = inside_margin;
            inner_rect.max.x = inner_rect.min.x + inner_rect.max.y - inner_rect.min.y;

            for instruction in component_type.get_draw_instructions() {
                match instruction {
                    DrawInstruction::Line([a, b]) => {
                        painter.line_segment(
                            [
                                pos2_with_rect(a, inner_rect),
                                pos2_with_rect(b, inner_rect),
                            ],
                            Stroke::new(2.0, Color32::WHITE),
                        );
                    }
                    DrawInstruction::CubicBezierCurve([a, c1, c2, b]) => {
                        let shape = CubicBezierShape::from_points_stroke(
                            [
                                pos2_with_rect(a, inner_rect),
                                pos2_with_rect(c1, inner_rect),
                                pos2_with_rect(c2, inner_rect),
                                pos2_with_rect(b, inner_rect),
                            ],
                            false,
                            Color32::TRANSPARENT,
                            Stroke::new(2.0, Color32::WHITE),
                        );
                        painter.add(shape);
                    }
                }
            }

            painter.text(
                Pos2::new(inner_rect.right() + 10.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                component_type.get_label(),
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::WHITE,
            );

            // if (app.get_selected_component().is_some() && 
            if response.clicked() {
                self.set_selected_component(Some(component_type));
            }
            if let Some(component) = self.get_selected_component() && component == component_type {
                painter.rect_filled(rect, 4.0, Color32::from_white_alpha(20));
            } else if response.hovered() {
                painter.rect_filled(rect, 4.0, Color32::from_white_alpha(5));
            }
        }

        response
    }
}
