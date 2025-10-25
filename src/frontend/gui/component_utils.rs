use crate::canvas::components::{ComponentData, GateType, Orientation};
use epaint::{Pos2, Rect};
use egui_macroquad::macroquad::prelude::*;

pub enum DrawInstruction {
    Line([Pos2; 2]),
    CubicBezierCurve([Pos2; 4]),
    Ellipse(Pos2, f32, f32),
}

pub fn macroquad_draw_curve(
    instruction: &DrawInstruction,
    frame: Rect,
    width: f32,
    color: Color,
    orientation: Orientation,
) {
    match instruction {
        DrawInstruction::Line([start, end]) => {
            let p0 = rotate_around_top_left(map_to_frame(start, frame), frame, orientation);
            let p1 = rotate_around_top_left(map_to_frame(end, frame), frame, orientation);
            draw_line(p0.x, p0.y, p1.x, p1.y, width, color);
        }

        DrawInstruction::CubicBezierCurve([p0, p1, p2, p3]) => {
            let p0 = rotate_around_top_left(map_to_frame(p0, frame), frame, orientation);
            let p1 = rotate_around_top_left(map_to_frame(p1, frame), frame, orientation);
            let p2 = rotate_around_top_left(map_to_frame(p2, frame), frame, orientation);
            let p3 = rotate_around_top_left(map_to_frame(p3, frame), frame, orientation);

            let steps = 32;
            let mut prev = p0;
            for i in 1..=steps {
                let t = i as f32 / steps as f32;
                let point = cubic_bezier_point(p0, p1, p2, p3, t);
                draw_line(prev.x, prev.y, point.x, point.y, width, color);
                prev = point;
            }
        }
        DrawInstruction::Ellipse(c, r_h, r_v) => {
            let c = rotate_around_top_left(map_to_frame(c, frame), frame, orientation);
            let r_h = r_h * frame.width();
            let r_v = r_v * frame.height();
            draw_ellipse_lines(c.x, c.y, r_h, r_v, 0., width, color);
            // draw_ellipse(c.x, c.y, r_h, r_v, 0., color);
        }
    }
}

/// Map normalized (0..1) Pos2 into a position inside the given `frame` Rect
fn map_to_frame(pos: &Pos2, frame: Rect) -> Pos2 {
    Pos2 {
        x: frame.left() + pos.x * frame.width(),
        y: frame.top() + pos.y * frame.height(),
    }
}

/// Rotate a point around the top-left corner of the frame using orientation
fn rotate_around_top_left(pos: Pos2, frame: Rect, orientation: Orientation) -> Pos2 {
    let origin = frame.left_top();
    let local = pos - origin;

    let rotated = match orientation {
        Orientation::Zero => local,
        Orientation::One => egui::Vec2::new(-local.y, local.x),
        Orientation::Two => egui::Vec2::new(-local.x, -local.y),
        Orientation::Three => egui::Vec2::new(local.y, -local.x),
    };

    origin + rotated
}

/// Compute a point on a cubic Bezier curve at parameter t ∈ [0, 1]
fn cubic_bezier_point(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    let mut p = Pos2::ZERO;
    p += p0.to_vec2() * uuu;
    p += p1.to_vec2() * 3.0 * uu * t;
    p += p2.to_vec2() * 3.0 * u * tt;
    p += p3.to_vec2() * ttt;
    p
}

pub fn pos2_with_rect(pos: &Pos2, rect: egui::Rect) -> Pos2 {
    Pos2 {
        x: rect.min.x + pos.x * rect.width(),
        y: rect.min.y + pos.y * rect.height(),
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum GuiComponentType {
    AndGate,
    OrGate,
    NandGate,
    NorGate,
    XorGate,
    XnorGate,
    NotGate,
}

impl GuiComponentType {
    pub fn to_component_data(&self) -> ComponentData {
        match self {
            Self::AndGate => ComponentData::Gate {
                gate_type: GateType::And,
                bitsize: 1,
                num_inputs: 2,
            },
            Self::OrGate => ComponentData::Gate {
                gate_type: GateType::Or,
                bitsize: 1,
                num_inputs: 2,
            },
            Self::NandGate => ComponentData::Gate {
                gate_type: GateType::Nand,
                bitsize: 1,
                num_inputs: 2,
            },
            Self::NorGate => ComponentData::Gate {
                gate_type: GateType::Nor,
                bitsize: 1,
                num_inputs: 2,
            },
            Self::XorGate => ComponentData::Gate {
                gate_type: GateType::Xor,
                bitsize: 1,
                num_inputs: 2,
            },
            Self::XnorGate => ComponentData::Gate {
                gate_type: GateType::Xnor,
                bitsize: 1,
                num_inputs: 2,
            },
            Self::NotGate => ComponentData::NotGate {
                bitsize: 1,
            },
        }
    }

    pub fn get_label(&self) -> &'static str {
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
    pub const AND_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 4] = {
        const BOX_WIDTH: f32 = 0.5;
        const OFFSET_Y: f32 = 0.0;
        const OFFSET_X: f32 = 0.125;
        [
            DrawInstruction::Line([Pos2::new(OFFSET_X, OFFSET_Y), Pos2::new(OFFSET_X, 1.0 - OFFSET_Y)]),
            DrawInstruction::Line([
                Pos2::new(OFFSET_X, OFFSET_Y),
                Pos2::new(BOX_WIDTH, OFFSET_Y)
            ]),
            DrawInstruction::Line([
                Pos2::new(OFFSET_X, 1.0 - OFFSET_Y),
                Pos2::new(BOX_WIDTH, 1.0 - OFFSET_Y),
            ]),
            DrawInstruction::CubicBezierCurve([
                Pos2::new(BOX_WIDTH, 1.0 - OFFSET_Y),
                Pos2::new(1.0, 1.0 - OFFSET_Y),
                Pos2::new(1.0, OFFSET_Y),
                Pos2::new(BOX_WIDTH, OFFSET_Y),
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

    pub fn get_draw_instructions(&self) -> &'static [DrawInstruction] {
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


