use egui_macroquad::macroquad::prelude::*;

use frontend::gui::App;

mod frontend;

#[macroquad::main("circuitsim")]
async fn main() {
    let mut gui = App::new();

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|ctx| {
            gui.update(ctx);
        });

        egui_macroquad::draw();
        next_frame().await;
    }
}

