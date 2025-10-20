use epaint::Pos2;

pub enum DrawInstruction {
    Line([Pos2; 2]),
    CubicBezierCurve([Pos2; 4]),
}

pub fn pos2_with_rect(pos: &Pos2, rect: egui::Rect) -> Pos2 {
    Pos2 {
        x: rect.min.x + pos.x * rect.width(),
        y: rect.min.y + pos.y * rect.height(),
    }
}

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
    pub fn get_label(&self) -> &'static str {
        match self {
            Self::And => "AND Gate",
            Self::Or => "OR Gate",
            Self::Nand => "NAND Gate",
            Self::Nor => "NOR Gate",
            Self::Xor => "XOR Gate",
            Self::Xnor => "XNOR Gate",
            Self::Not => "NOT Gate",
        }
    }

    const UNIMPLEMENTED_DRAW_INSTRUCTIONS: [DrawInstruction; 0] = [];
    const AND_GATE_DRAW_INSTRUCTIONS: [DrawInstruction; 4] = {
        const BOX_WIDTH: f32 = 0.5;
        const OFFSET_Y: f32 = 0.1;
        [
            DrawInstruction::Line([Pos2::new(0.0, OFFSET_Y), Pos2::new(0.0, 1.0 - OFFSET_Y)]),
            DrawInstruction::Line([Pos2::new(0.0, OFFSET_Y), Pos2::new(0.5, OFFSET_Y)]),
            DrawInstruction::Line([
                Pos2::new(0.0, 1.0 - OFFSET_Y),
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
            Self::And => &Self::AND_GATE_DRAW_INSTRUCTIONS,
            Self::Or => &Self::OR_GATE_DRAW_INSTRUCTIONS,
            Self::Nand => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::Nor => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::Xor => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::Xnor => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
            Self::Not => &Self::UNIMPLEMENTED_DRAW_INSTRUCTIONS,
        }
    }
}
