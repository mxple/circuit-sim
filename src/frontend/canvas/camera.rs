use egui_macroquad::macroquad::prelude::*;

pub struct Camera {
    position: Vec2,
    target_position: Vec2,
    pub zoom: f32,
    target_zoom: f32,
    lerp_speed: f32,
}

impl Camera {
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

        self.position = self.position.lerp(self.target_position, self.lerp_speed * dt);
        self.zoom = lerp(self.zoom, self.target_zoom, self.lerp_speed * dt);
    }

    pub fn handle_input(&mut self, dt: f32) {
        let move_speed = 500.0 / self.zoom;
        
        if is_key_down(KeyCode::W) {
            self.target_position.y -= move_speed * dt;
        }
        if is_key_down(KeyCode::S) {
            self.target_position.y += move_speed * dt;
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
            self.target_zoom = (self.target_zoom * zoom_factor).clamp(10.0, 100.0);
        }
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let offset_from_center = screen_pos - screen_center;
        self.position + offset_from_center / self.zoom
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let offset = world_pos - self.position;
        screen_center + offset * self.zoom
    }

    pub fn get_view_bounds(&self) -> (Vec2, Vec2) {
        let half_width = screen_width() / (2.0 * self.zoom);
        let half_height = screen_height() / (2.0 * self.zoom);
        
        let min = Vec2::new(
            self.position.x - half_width,
            self.position.y - half_height,
        );
        let max = Vec2::new(
            self.position.x + half_width,
            self.position.y + half_height,
        );
        
        (min, max)
    }

    pub fn draw_egui_ui(&self, ctx: &egui::Context) {
        let mouse_screen = Vec2::new(mouse_position().0, mouse_position().1);
        let mouse_world = self.screen_to_world(mouse_screen);
        
        egui::Window::new("Camera Info")
            .default_pos(egui::pos2(10.0, 10.0))
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!("Camera Position: ({:.1}, {:.1})", self.position.x, self.position.y));
                ui.label(format!("Zoom: {:.2}x", self.zoom));
                
                ui.separator();
                
                ui.colored_label(
                    egui::Color32::YELLOW,
                    format!("Mouse World: ({:.1}, {:.1})", mouse_world.x, mouse_world.y)
                );
                
                ui.separator();
                
                ui.small("Controls:");
                ui.small("• WASD to move camera");
                ui.small("• Mouse wheel to zoom");
            });
    }
}


