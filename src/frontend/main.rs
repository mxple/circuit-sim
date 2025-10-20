use egui_macroquad::macroquad::prelude::*;

use crate::canvas::camera::GridCamera;
use crate::canvas::components::{ComponentData, ComponentSystem};
use crate::canvas::grid::GridDrawer;
use crate::canvas::input::{CanvasInput, CanvasInputState};
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
    let mut input = CanvasInput::new();
    let mut ws = WireSystem::new();
    let mut cs = ComponentSystem::new();

    request_new_screen_size(1280.0, 720.0);
    next_frame().await;

    loop {
        profile_scope!("frame");
        let dt = get_frame_time();
        let mut egui_wants_ptr = false;

        {
            profile_scope!("logic");
            camera.handle_input(dt);
            camera.update(dt);
            if input.state == CanvasInputState::Component {
                cs.handle_input(
                    &camera,
                    gui.get_selected_component().unwrap().to_component_data()
                );
            } else if input.state == CanvasInputState::Wire {
                ws.handle_input(&camera);
            }

            egui_macroquad::ui(|ctx| {
                gui.update(ctx, &mut input.state, &mut cs.get_selection_mut(input.selection));
                if enable_camera_debug {
                    camera.draw_egui_ui(ctx);
                }
                if enable_profiler_debug {
                    profiler::profile_update(ctx);
                }
                egui_wants_ptr = ctx.is_pointer_over_area()
            });
        }

        {
            profile_scope!("draw");

            // clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
            clear_background(Color::new(1.0, 1.0, 1.0, 1.0));
            set_camera(&camera);
            gd.draw_grid(&camera);
            cs.draw_components(&camera, input.selection);
            ws.draw_wires(&camera);
            if input.state == CanvasInputState::Wire {
                ws.draw_preview(&camera);
            } else if input.state == CanvasInputState::Component {
                cs.draw_preview(
                    &camera,
                    gui.get_selected_component().unwrap().to_component_data()
                );
            } else if input.state == CanvasInputState::Idle {
                input.draw_selection(&camera);
            }

            gl_use_default_material();
            set_default_camera();
            egui_macroquad::draw();
        }
            input.handle_input(&camera, egui_wants_ptr);

        next_frame().await;
    }
}
