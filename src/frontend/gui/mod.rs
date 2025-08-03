use egui_macroquad::egui;
use egui_macroquad::macroquad::prelude::*;
use component_utils::CircuitComponentType;

mod component_utils;
mod component_selector;
mod toolbar;

pub struct App {
    expanded: bool,
    selected_component: Option<CircuitComponentType>,
    hotbar_selections: [Option<CircuitComponentType>; Self::NUM_HOTBAR_BUTTONS],
    hovered_hotbar_button: Option<usize>,
    dragged_component: Option<CircuitComponentType>,
}

impl App {
    pub const NUM_HOTBAR_BUTTONS: usize = 5;
    pub const HOTBAR_BUTTON_LABELS: [&'static str; Self::NUM_HOTBAR_BUTTONS] = [
        "1", "2", "3", "4", "5",
    ];
    pub fn new() -> Self {
        Self {
            expanded: true,
            selected_component: None,
            hotbar_selections: [None; Self::NUM_HOTBAR_BUTTONS],
            hovered_hotbar_button: None,
            dragged_component: None,
        }
    }

    pub fn get_selected_component(&mut self) -> Option<CircuitComponentType> {
        self.selected_component.clone()
    }

    pub fn set_selected_component(&mut self, component: Option<CircuitComponentType>) {
        self.selected_component = component;
    }

    pub fn update(&mut self, ctx: &egui::Context) {
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
            let build = if cfg!(target_arch = "wasm32") { "WASM" } else { "Native" };
            let version = env!("CARGO_PKG_VERSION");
            ui.horizontal(|ui| {
                ui.label(format!("Build: {}", build));
                ui.label(format!("Version: {}", version));
            });
        });

        SidePanel::left("Circuits")
            .min_width(screen_width() / 6.)
            .max_width(screen_width() / 6.)
            .resizable(false)
            .show_animated(ctx, self.expanded, |ui| {
                ui.label("Components");
                CollapsingHeader::new("Gates").show(ui, |ui| {
                    let gates = [
                        CircuitComponentType::AndGate,
                        CircuitComponentType::OrGate,
                        CircuitComponentType::NandGate,
                        CircuitComponentType::NorGate,
                        CircuitComponentType::XorGate,
                        CircuitComponentType::XnorGate,
                        CircuitComponentType::NotGate,
                    ];
                    for gate in gates {
                        self.circuit_component_button(
                            ui,
                            egui::Vec2::new(ui.available_size().x, 60.0),
                            gate,
                        );
                    }
                });
            });


        // SidePanel::left("toggle_button_panel")
        //     .frame(Frame::NONE)
        //     .exact_width(8.0)
        //     .show_separator_line(false)
        //     .resizable(false)
        //     .show(ctx, |ui| {
        //         let icon = if self.expanded { "⏴" } else { "⏵" };
        //         println!("{}", ui.available_width());
        //         if ui.add_sized(ui.available_size(), Button::new(icon)).clicked() {
        //             self.expanded = !self.expanded;
        //         }
        //     });

        SidePanel::right("Right").show(ctx, |ui| {
            ui.label("test1");
        });
        
        self.render_toolbar(ctx);

        // Ensure that each component can only be in one hotbar slot at a time
        if let Some(hovered_index) = self.hovered_hotbar_button && self.dragged_component.is_some() {
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
