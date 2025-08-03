use egui_macroquad::macroquad::prelude::*;
use macroquad::{camera::Camera, texture::RenderPass};

pub struct GridCamera {
    position: Vec2,
    target_position: Vec2,
    pub zoom: f32,
    target_zoom: f32,
    lerp_speed: f32,
}

impl Camera for GridCamera {
    fn matrix(&self) -> Mat4 {
        let translate_world = Mat4::from_translation(vec3(-self.position.x, -self.position.y, 0.0));
        let scale = Mat4::from_scale(vec3(
            self.zoom / screen_width(),
            self.zoom / screen_height(),
            1.0,
        ));

        scale * translate_world
    }

    fn depth_enabled(&self) -> bool {
        false
    }

    fn render_pass(&self) -> Option<RenderPass> {
        None
    }

    fn viewport(&self) -> Option<(i32, i32, i32, i32)> {
        None
    }
}

impl GridCamera {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            target_position: Vec2::ZERO,
            zoom: 50.0,
            target_zoom: 50.0,
            lerp_speed: 8.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        fn lerp(a: f32, b: f32, t: f32) -> f32 {
            a + (b - a) * t.clamp(0.0, 1.0)
        }

        self.position = self
            .position
            .lerp(self.target_position, self.lerp_speed * dt);
        self.zoom = lerp(self.zoom, self.target_zoom, self.lerp_speed * dt);
    }

    pub fn handle_input(&mut self, dt: f32) {
        let move_speed = 1000.0 / self.zoom;

        if is_key_down(KeyCode::W) {
            self.target_position.y += move_speed * dt;
        }
        if is_key_down(KeyCode::S) {
            self.target_position.y -= move_speed * dt;
        }
        if is_key_down(KeyCode::A) {
            self.target_position.x -= move_speed * dt;
        }
        if is_key_down(KeyCode::D) {
            self.target_position.x += move_speed * dt;
        }

        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            let zoom_factor = 1.1_f32.powf(wheel);
            self.target_zoom = (self.target_zoom * zoom_factor).clamp(10., 200.);
        }
    }

    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        let dims = self
            .viewport()
            .map(|(vx, vy, vw, vh)| Rect {
                x: vx as f32,
                y: screen_height() - (vy + vh) as f32,
                w: vw as f32,
                h: vh as f32,
            })
            .unwrap_or(Rect {
                x: 0.0,
                y: 0.0,
                w: screen_width(),
                h: screen_height(),
            });

        let point = vec2(
            (point.x - dims.x) / dims.w * 2. - 1.,
            1. - (point.y - dims.y) / dims.h * 2.,
        );
        let inv_mat = self.matrix().inverse();
        let transform = inv_mat.transform_point3(vec3(point.x, point.y, 0.));

        vec2(transform.x, transform.y)
    }

    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        let mat = self.matrix();
        let transform = mat.transform_point3(vec3(point.x, point.y, 0.));

        vec2(
            (transform.x / 2. + 0.5) * screen_width(),
            (0.5 - transform.y / 2.) * screen_height(),
        )
    }

    pub fn get_view_bounds(&self) -> (Vec2, Vec2) {
        // Screen coordinates go from (0,0) to (screen_width, screen_height)
        // Convert these corners to world coordinates
        let top_left_world = self.screen_to_world(Vec2::new(0.0, 0.0));
        let bottom_right_world = self.screen_to_world(Vec2::new(screen_width(), screen_height()));

        let min = Vec2::new(
            top_left_world.x.min(bottom_right_world.x),
            top_left_world.y.min(bottom_right_world.y),
        );
        let max = Vec2::new(
            top_left_world.x.max(bottom_right_world.x),
            top_left_world.y.max(bottom_right_world.y),
        );

        (min, max)
    }

    pub fn draw_egui_ui(&self, ctx: &egui::Context) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = self.screen_to_world(mouse_screen);

        egui::Window::new("Camera Info")
            .default_pos(egui::pos2(500.0, 10.0))
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!(
                    "Camera Position: ({:.1}, {:.1})",
                    self.position.x, self.position.y
                ));
                ui.label(format!("Zoom: {:.2}x", self.zoom));

                ui.separator();

                ui.colored_label(
                    egui::Color32::YELLOW,
                    format!("Mouse World: ({:.1}, {:.1})", mouse_world.x, mouse_world.y),
                );

                ui.separator();

                ui.small("Controls:");
                ui.small("• WASD to move camera");
                ui.small("• Mouse wheel to zoom");
            });
    }

    pub fn get_pixel_thickness(&self) -> f32 {
        let screen_point1 = Vec2::new(0.0, 0.0);
        let screen_point2 = Vec2::new(1.0, 0.0);
        let world_point1 = self.screen_to_world(screen_point1);
        let world_point2 = self.screen_to_world(screen_point2);
        (world_point2 - world_point1).length()
    }
}
