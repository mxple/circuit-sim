use epaint::Pos2;

use crate::canvas::components::{ComponentData, GateType};

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
            },
            Self::OrGate => ComponentData::Gate {
                gate_type: GateType::Or,
                bitsize: 1,
            },
            Self::NandGate => ComponentData::Gate {
                gate_type: GateType::Nand,
                bitsize: 1,
            },
            Self::NorGate => ComponentData::Gate {
                gate_type: GateType::Nor,
                bitsize: 1,
            },
            Self::XorGate => ComponentData::Gate {
                gate_type: GateType::Xor,
                bitsize: 1,
            },
            Self::XnorGate => ComponentData::Gate {
                gate_type: GateType::Xnor,
                bitsize: 1,
            },
            Self::NotGate => ComponentData::Gate {
                gate_type: GateType::Not,
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


