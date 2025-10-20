use egui_macroquad::macroquad::prelude::*;

use crate::canvas::camera::GridCamera;
use crate::canvas::components::ComponentSystem;
use crate::canvas::grid::GridDrawer;
use crate::canvas::input::CanvasInputState;
use crate::canvas::wiring::WireSystem;
use crate::gui::App;

mod profiler;
mod canvas;
mod gui;
mod util;

#[macroquad::main("circuitsim")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut enable_camera_debug = false;
    let mut enable_profiler_debug = false;
    for arg in args {
        if arg == "c" {
            enable_camera_debug = true;
        }
        if arg == "p" {
            enable_profiler_debug = true;
        }
    }
    let mut camera = GridCamera::new();
    let gd = GridDrawer::new(crate::canvas::grid::GridDrawOptions::Instanced, vec4(0.3, 0.3, 0.3, 0.3));
    let mut gui = App::new();
    let mut input_state = CanvasInputState::new();
    let mut ws = WireSystem::new();
    let mut cs = ComponentSystem::new();

    request_new_screen_size(1280.0, 720.0);
    next_frame().await;

    loop {
        profile_scope!("frame");
        let dt = get_frame_time();

        {
            profile_scope!("logic");
            camera.handle_input(dt);
            camera.update(dt);
            input_state.handle_input();
            if input_state == CanvasInputState::Component {
                cs.handle_input(&camera, gui.get_selected_component().unwrap());
            } else if input_state == CanvasInputState::Wire {
                ws.handle_input(&camera);
            }

            egui_macroquad::ui(|ctx| {
                gui.update(ctx, &mut input_state);
                if enable_camera_debug {
                    camera.draw_egui_ui(ctx);
                }
                if enable_profiler_debug {
                    profiler::profile_update(ctx);
                }
            });
        }

        {
            profile_scope!("draw");

            clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
            set_camera(&camera);
            gd.draw_grid(&camera);
            cs.draw_components(&camera);
            ws.draw_wires(&camera);
            if input_state == CanvasInputState::Wire {
                ws.draw_preview(&camera);
            } else {
            }

            gl_use_default_material();
            set_default_camera();
            egui_macroquad::draw();
        }

        next_frame().await;
    }
}
