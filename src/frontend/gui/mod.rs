use egui_macroquad::egui;
use egui_macroquad::macroquad::prelude::*;

pub mod profiler;

pub struct App {
    expanded: bool,
    // add more fields as needed
}

impl App {
    pub fn new() -> Self {
        Self {
            expanded: true,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
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
            .resizable(false)
            .show_animated(ctx, self.expanded, |ui| {
                ui.label("test");
            });

        SidePanel::left("toggle_button_panel")
            .exact_width(8.0)
            .frame(Frame::NONE)
            .show_separator_line(false)
            .resizable(false)
            .show(ctx, |ui| {
                let icon = if self.expanded { "⏴" } else { "⏵" };
                if ui.add_sized(ui.available_size(), Button::new(icon)).clicked() {
                    self.expanded = !self.expanded;
                }
            });

        SidePanel::right("Right").show(ctx, |ui| {
            ui.button("test1");
        });
    }
}

