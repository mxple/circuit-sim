use egui_macroquad::macroquad::prelude::*;

use frontend::canvas::wiring::WireSystem;
use frontend::gui::App;
use frontend::canvas::camera::Camera;
use frontend::canvas::grid::draw_grid;
mod frontend;


#[macroquad::main("circuitsim")]
async fn main() {
    let mut camera = Camera::new();
    let mut gui = App::new();
    let mut ws = WireSystem::new();
    
    loop {
        let dt = get_frame_time();

        camera.handle_input(dt);
        camera.update(dt);
        ws.handle_input(&camera);

        egui_macroquad::ui(|ctx| {
            gui.update(ctx);
            camera.draw_egui_ui(ctx);
        });

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
        draw_grid(&camera);
        ws.draw_preview(&camera);
        ws.draw_wires(&camera);
        egui_macroquad::draw();
        next_frame().await;
    }
}
