use crate::canvas::input::CanvasInputState;
use crate::App;
use crate::gui::component_utils::{
    pos2_with_rect, DrawInstruction, GuiComponentType
};
use egui_macroquad::egui::{Color32, Response, Sense, Stroke, Ui};
use epaint::{CubicBezierShape, Pos2};

impl App {
    pub fn circuit_component_button(
        &mut self,
        ui: &mut Ui,
        size: egui::Vec2,
        component_type: GuiComponentType,
        input_state: &mut CanvasInputState
    ) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::all());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let inside_margin = rect.shrink(10.0);
            let mut inner_rect = inside_margin;
            inner_rect.max.x = inner_rect.min.x + inner_rect.max.y - inner_rect.min.y;

            for instruction in component_type.get_draw_instructions() {
                match instruction {
                    DrawInstruction::Line([a, b]) => {
                        painter.line_segment(
                            [pos2_with_rect(a, inner_rect), pos2_with_rect(b, inner_rect)],
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
                    DrawInstruction::Ellipse(_, _, _) => todo!()
                }
            }

            painter.text(
                Pos2::new(inner_rect.right() + 10.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                component_type.get_label(),
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::WHITE,
            );

            if response.clicked() {
                self.set_selected_component(Some(component_type), input_state);
            }
            if response.drag_stopped() {
                self.dragged_component = Some(component_type);
            }
            if let Some(component) = self.get_selected_component()
                && component == component_type
            {
                painter.rect_filled(rect, 4.0, Color32::from_white_alpha(20));
            } else if response.hovered() {
                painter.rect_filled(rect, 4.0, Color32::from_white_alpha(5));
            }
        }

        response
    }
}
