use egui_macroquad::macroquad::prelude::*;
use frontend::gui::profiler::Profiler;

use frontend::canvas::camera::GridCamera;
use frontend::canvas::grid::draw_grid;
use frontend::canvas::wiring::WireSystem;
use frontend::gui::App;
mod frontend;

#[macroquad::main("circuitsim")]
async fn main() {
    let mut camera = GridCamera::new();
    let mut gui = App::new();
    let mut ws = WireSystem::new();
    let mut profiler = Profiler::new(0.05);
    profiler.register("logic");
    profiler.register("draw");
    profiler.register("frame");

    loop {
        profiler.start("logic");
        profiler.start("frame");
        let dt = get_frame_time();

        camera.handle_input(dt);
        camera.update(dt);
        ws.handle_input(&camera);

        egui_macroquad::ui(|ctx| {
            gui.update(ctx);
            camera.draw_egui_ui(ctx);
            profiler.update(ctx);
        });
        profiler.end("logic");
        profiler.start("draw");

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        set_camera(&camera);

        draw_grid(&camera);
        ws.draw_preview(&camera);
        ws.draw_wires(&camera);

        egui_macroquad::draw();

        profiler.end("draw");

        next_frame().await;
        profiler.end("frame");
        // TODO wait frame
    }
}
