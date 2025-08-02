use egui_macroquad::egui;
use egui_macroquad::macroquad::prelude::*;

#[macroquad::main("egui + macroquad wasm")]
async fn main() {
    loop {
        clear_background(MAGENTA);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Hello").show(ctx, |ui| {
                ui.label("Running in WASM!");
            });
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}

