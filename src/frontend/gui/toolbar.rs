use crate::App;
use crate::gui::component_utils::{DrawInstruction, pos2_with_rect};
use egui_macroquad::egui::{
    Color32, Key, Response, Sense, Stroke, StrokeKind, TopBottomPanel, Ui, Vec2, menu,
};
use epaint::CubicBezierShape;

impl App {
    fn handle_hotbar_keys(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_down(Key::Num1) {
                self.selected_component = self.hotbar_selections[0];
            }
            if i.key_down(Key::Num2) {
                self.selected_component = self.hotbar_selections[1];
            }
            if i.key_down(Key::Num3) {
                self.selected_component = self.hotbar_selections[2];
            }
            if i.key_down(Key::Num4) {
                self.selected_component = self.hotbar_selections[3];
            }
            if i.key_down(Key::Num5) {
                self.selected_component = self.hotbar_selections[4];
            }
        });
    }

    fn hotbar_button(&mut self, size: egui::Vec2, ui: &mut Ui, index: usize) -> Response {
        let (rect, response) = ui.allocate_exact_size(size, Sense::all());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let padded_rect = rect.shrink(5.0);
            let inner_rect = padded_rect.shrink(5.0);

            let button_contents = self.hotbar_selections[index];

            if let Some(selected_component) = button_contents {
                for instruction in selected_component.get_draw_instructions() {
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
                    }
                }
            }

            if response.clicked() {
                self.set_selected_component(button_contents);
            }
            if let Some(component) = self.get_selected_component()
                && self.get_selected_component() == button_contents
            {
                painter.rect_filled(padded_rect, 4.0, Color32::from_white_alpha(20));
            } else if response.contains_pointer() {
                painter.rect_filled(padded_rect, 4.0, Color32::from_white_alpha(5));
            }

            if response.contains_pointer() {
                self.hovered_hotbar_button = Some(index);
            }
            painter.rect_stroke(
                padded_rect,
                4.0,
                Stroke {
                    width: 1.0,
                    color: Color32::from_white_alpha(50),
                },
                StrokeKind::Inside,
            );

            painter.text(
                padded_rect.max,
                egui::Align2::CENTER_CENTER,
                Self::HOTBAR_BUTTON_LABELS[index],
                egui::TextStyle::Body.resolve(ui.style()),
                Color32::WHITE,
            );
        }

        response
    }

    pub fn render_toolbar(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                for i in 0..Self::NUM_HOTBAR_BUTTONS {
                    self.hotbar_button(Vec2::splat(40.0), ui, i);
                }
            });
        });
        self.handle_hotbar_keys(ctx);
    }
}
