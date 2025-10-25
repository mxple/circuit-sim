use egui_macroquad::egui;
use egui_macroquad::macroquad::prelude::*;

use crate::canvas::components::{Component, ComponentData, GateType, Orientation};
use crate::canvas::input::CanvasInputState;
use crate::gui::component_utils::GuiComponentType;

mod component_selector;
pub mod component_utils;
mod toolbar;

pub struct App {
    expanded: bool,
    selected_component: Option<GuiComponentType>,
    hotbar_selections: [Option<GuiComponentType>; Self::NUM_HOTBAR_BUTTONS],
    hovered_hotbar_button: Option<usize>,
    dragged_component: Option<GuiComponentType>,
}

impl App {
    pub const NUM_HOTBAR_BUTTONS: usize = 5;
    pub const HOTBAR_BUTTON_LABELS: [&'static str; Self::NUM_HOTBAR_BUTTONS] =
        ["1", "2", "3", "4", "5"];
    pub fn new() -> Self {
        Self {
            expanded: true,
            selected_component: None,
            hotbar_selections: [None; Self::NUM_HOTBAR_BUTTONS],
            hovered_hotbar_button: None,
            dragged_component: None,
        }
    }

    pub fn get_selected_component(&mut self) -> Option<GuiComponentType> {
        self.selected_component
    }

    pub fn set_selected_component(&mut self, component: Option<GuiComponentType>, input_state: &mut CanvasInputState) {
        self.selected_component = component;
        if component.is_some() {
            *input_state = CanvasInputState::Component;
        } else {
            *input_state = CanvasInputState::Idle;
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, input_state: &mut CanvasInputState, selection: &mut [&mut Component]) {
        self.hovered_hotbar_button = None;
        self.dragged_component = None;
        use egui::*;

        TopBottomPanel::top("menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // …
                    }
                });
            });
        });

        // show build (wasm or native) at bottom left
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            let build = if cfg!(target_arch = "wasm32") {
                "WASM"
            } else {
                "Native"
            };
            let version = env!("CARGO_PKG_VERSION");
            let mode = format!("{:?}", input_state);
            ui.horizontal(|ui| {
                ui.label(format!("Build: {}", build));
                ui.label(format!("Version: {}", version));
                ui.label(format!("Mode: {}", mode));
            });
        });

        SidePanel::left("Circuits")
            .min_width(screen_width() / 6.)
            .max_width(screen_width() / 6.)
            .resizable(false)
            .show_animated(ctx, self.expanded, |ui| {
                ui.allocate_ui(ui.available_size(), |ui| {
                    ui.label("Components");
                    CollapsingHeader::new("Gates").show(ui, |ui| {
                        let gates = [
                            GuiComponentType::AndGate,
                            GuiComponentType::OrGate,
                            GuiComponentType::NandGate,
                            GuiComponentType::NorGate,
                            GuiComponentType::XorGate,
                            GuiComponentType::XnorGate,
                            GuiComponentType::NotGate,
                        ];
                        for gate in gates {
                            self.circuit_component_button(
                                ui,
                                egui::Vec2::new(ui.available_size().x, 60.0),
                                gate,
                                input_state
                            );
                        }
                    });
                    ui.separator();
                    fn orientation_dropdown(ui: &mut Ui, data: &mut Orientation) {
                        ComboBox::from_label("Orientation")
                            .selected_text(data.get_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(data, Orientation::Zero, Orientation::Zero.get_name());
                                ui.selectable_value(data, Orientation::One, Orientation::One.get_name());
                                ui.selectable_value(data, Orientation::Two, Orientation::Two.get_name());
                                ui.selectable_value(data, Orientation::Three, Orientation::Three.get_name());
                            });
                    }
                    fn gate_type_dropdown(ui: &mut Ui, data: &mut GateType) {
                        ComboBox::from_label("Gate type")
                            .selected_text(data.get_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(data, GateType::And, "AND");
                                ui.selectable_value(data, GateType::Or, "OR");
                                ui.selectable_value(data, GateType::Nand, "NAND");
                                ui.selectable_value(data, GateType::Nor, "NOR");
                                ui.selectable_value(data, GateType::Xor, "XOR");
                                ui.selectable_value(data, GateType::Xnor, "XNOR");
                            });
                    }
                    fn bitsize_dropdown(ui: &mut Ui, data: &mut u8) {
                        ComboBox::from_label("Bitsize")
                            .selected_text(data.to_string())
                            .show_ui(ui, |ui| {
                                for i in 1..=32 {
                                    ui.selectable_value(data, i, i.to_string());
                                }
                            });
                    }
                    fn num_inputs_dropdown(ui: &mut Ui, data: &mut u8) {
                        ComboBox::from_label("# inputs")
                            .selected_text(data.to_string())
                            .show_ui(ui, |ui| {
                                for i in 1..=32 {
                                    ui.selectable_value(data, i, i.to_string());
                                }
                            });
                    }
                    if selection.len() == 1 && let Some(c) = selection.get_mut(0) {
                        ui.label(c.data.get_name());
                        orientation_dropdown(ui, &mut c.orientation);
                        match (*c).data {
                            ComponentData::Gate {
                                ref mut gate_type,
                                ref mut bitsize,
                                ref mut num_inputs,
                            } => {
                                gate_type_dropdown(ui, gate_type);
                                bitsize_dropdown(ui, bitsize);
                                num_inputs_dropdown(ui, num_inputs);
                            }
                            ComponentData::NotGate { ref mut bitsize }  => {
                                bitsize_dropdown(ui, bitsize);
                            }
                            ComponentData::Mux {
                                ref mut bitsize
                            } => {
                                bitsize_dropdown(ui, bitsize);
                            }
                        }
                    }
                });
            });

        SidePanel::left("toggle_button_panel")
            .frame(Frame::NONE)
            .exact_width(10.0)
            .show_separator_line(false)
            .resizable(false)
            .show(ctx, |ui| {
                let icon = if self.expanded { "⏴" } else { "⏵" };
                let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::all());
                let painter = ui.painter_at(rect);
                if response.clicked() {
                    self.expanded = !self.expanded;
                } else if response.hovered() {
                    painter.rect_filled(rect, 0.0, Color32::from_rgb(70, 70, 70));
                } else {
                    painter.rect_filled(rect, 0.0, Color32::from_rgb(50, 50, 50));
                }
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    icon,
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::LIGHT_GRAY,
                );
            });

        self.render_toolbar(ctx, input_state);

        // Ensure that each component can only be in one hotbar slot at a time
        if let Some(hovered_index) = self.hovered_hotbar_button
            && self.dragged_component.is_some()
        {
            for i in 0..Self::NUM_HOTBAR_BUTTONS {
                if self.hotbar_selections[i] == self.dragged_component {
                    self.hotbar_selections[i] = self.hotbar_selections[hovered_index];
                    break;
                }
            }
            self.hotbar_selections[hovered_index] = self.dragged_component;
        }
    }
}
