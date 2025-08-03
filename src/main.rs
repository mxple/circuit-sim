use egui_macroquad::macroquad::prelude::*;

use frontend::canvas::camera::GridCamera;
use frontend::canvas::grid::draw_grid;
use frontend::canvas::wiring::WireSystem;
use frontend::gui::App;
mod frontend;
mod profiler;

#[macroquad::main("circuitsim")]
async fn main() {
    let mut camera = GridCamera::new();
    let mut gui = App::new();
    let mut ws = WireSystem::new();
    
    request_new_screen_size(1280.0, 720.0);
    next_frame().await;

    loop {
        profile_scope!("frame");
        let dt = get_frame_time();

        {
            profile_scope!("logic");
            camera.handle_input(dt);
            camera.update(dt);
            ws.handle_input(&camera);

            egui_macroquad::ui(|ctx| {
                gui.update(ctx);
                camera.draw_egui_ui(ctx);
                profiler::profile_update(ctx);
            });
        }

        {
            profile_scope!("draw");

            clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

            set_camera(&camera);

            draw_grid(&camera);
            ws.draw_preview(&camera);
            ws.draw_wires(&camera);

            egui_macroquad::draw();
        }

        next_frame().await;
    }
}
