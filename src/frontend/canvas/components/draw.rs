use egui::Pos2;

use crate::gui::component_utils::DrawInstruction;


pub const UNIMPLEMENTED_DRAW_INSTRUCTIONS: [DrawInstruction; 0] = [];

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

pub const OR_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 3] = {
    const OFFSET_Y: f32 = 0.0;
    const MID_OFFSET_Y: f32 = 0.1;
    const ARC_START_X: f32 = 0.1;
    const ARC_END_X: f32 = 0.875;
    [
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, OFFSET_Y),
            Pos2::new(ARC_START_X, OFFSET_Y),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, OFFSET_Y),
            Pos2::new(0.55, MID_OFFSET_Y * 0.3),
            Pos2::new(0.7, MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(0.55, 1.0 - MID_OFFSET_Y * 0.3),
            Pos2::new(0.7, 1.0 - MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
    ]
};

pub const NAND_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 5] = {
    const BOX_WIDTH: f32 = 0.41;
    const OFFSET_Y: f32 = 0.0;
    const OFFSET_X: f32 = 0.125;
    const BUBBLE_R: f32 = 0.5 / 9.;
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
            Pos2::new(0.8, 1.0 - OFFSET_Y),
            Pos2::new(0.8, OFFSET_Y),
            Pos2::new(BOX_WIDTH, OFFSET_Y),
        ]),
        DrawInstruction::Ellipse(
            Pos2::new(0.78, 0.5),
            BUBBLE_R,
            BUBBLE_R * 4. / 3.,
        ),
    ]
};

pub const NOR_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 4] = {
    const OFFSET_Y: f32 = 0.0;
    const MID_OFFSET_Y: f32 = 0.1;
    const ARC_START_X: f32 = 0.1;
    const ARC_END_X: f32 = 0.715;
    const BUBBLE_R: f32 = 0.5 / 9.;
    [
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, OFFSET_Y),
            Pos2::new(ARC_START_X, OFFSET_Y),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, OFFSET_Y),
            Pos2::new(0.55, MID_OFFSET_Y * 0.3),
            Pos2::new(0.6, MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(0.55, 1.0 - MID_OFFSET_Y * 0.3),
            Pos2::new(0.6, 1.0 - MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::Ellipse(
            Pos2::new(0.78, 0.5),
            BUBBLE_R,
            BUBBLE_R * 4. / 3.,
        ),
    ]
};

pub const XOR_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 4] = {
    const OFFSET_Y: f32 = 0.0;
    const MID_OFFSET_Y: f32 = 0.1;
    const ARC_START_X: f32 = 0.1;
    const DOUBLE_STROKE_OFFSET: f32 = 0.09;
    const ARC_END_X: f32 = 0.875;
    [
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, OFFSET_Y),
            Pos2::new(ARC_START_X, OFFSET_Y),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET + 0.3, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET + 0.3, OFFSET_Y),
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, OFFSET_Y),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, OFFSET_Y),
            Pos2::new(0.55, MID_OFFSET_Y * 0.3),
            Pos2::new(0.7, MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, 1.0 - OFFSET_Y),
            Pos2::new(0.55, 1.0 - MID_OFFSET_Y * 0.3),
            Pos2::new(0.7, 1.0 - MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
    ]
};

pub const XNOR_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 5] = {
    const OFFSET_Y: f32 = 0.0;
    const MID_OFFSET_Y: f32 = 0.1;
    const ARC_START_X: f32 = 0.1;
    const DOUBLE_STROKE_OFFSET: f32 = 0.09;
    const ARC_END_X: f32 = 0.715;
    const BUBBLE_R: f32 = 0.5 / 9.;
    [
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + 0.3, OFFSET_Y),
            Pos2::new(ARC_START_X, OFFSET_Y),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET + 0.3, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET + 0.3, OFFSET_Y),
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, OFFSET_Y),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, OFFSET_Y),
            Pos2::new(0.55, MID_OFFSET_Y * 0.3),
            Pos2::new(0.6, MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::CubicBezierCurve([
            Pos2::new(ARC_START_X + DOUBLE_STROKE_OFFSET, 1.0 - OFFSET_Y),
            Pos2::new(0.55, 1.0 - MID_OFFSET_Y * 0.3),
            Pos2::new(0.6, 1.0 - MID_OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::Ellipse(
            Pos2::new(0.78, 0.5),
            BUBBLE_R,
            BUBBLE_R * 4. / 3.,
        ),
    ]
};

pub const NOT_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 4] = {
    const OFFSET_Y: f32 = 0.1;
    const ARC_START_X: f32 = 1. / 6.;
    const ARC_END_X: f32 = 1. - 2. * ARC_START_X;
    [
        DrawInstruction::Line([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(ARC_START_X, OFFSET_Y),
        ]),
        DrawInstruction::Line([
            Pos2::new(ARC_START_X, OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::Line([
            Pos2::new(ARC_START_X, 1.0 - OFFSET_Y),
            Pos2::new(ARC_END_X, 0.5),
        ]),
        DrawInstruction::Ellipse(
            Pos2::new(4.2 * ARC_START_X, 0.5),
            ARC_START_X / 3.,
            0.5 / 3.,
        ),
    ]
};
