use egui_macroquad::macroquad::prelude::*;

use super::camera::Camera;

pub fn draw_grid(camera: &Camera) {
    let (view_min, view_max) = camera.get_view_bounds();
    
    let start_x = view_min.x.floor();
    let end_x = view_max.x.ceil();
    let start_y = view_min.y.floor();
    let end_y = view_max.y.ceil();
    
    let color = Color::new(0.3, 0.3, 0.3, 0.5);

    let mut x = start_x;
    while x <= end_x {
        let start_screen = camera.world_to_screen(Vec2::new(x, view_min.y));
        let end_screen = camera.world_to_screen(Vec2::new(x, view_max.y));
        
        draw_line(
            start_screen.x,
            start_screen.y,
            end_screen.x,
            end_screen.y,
            1.0,
            color,
        );
        
        x += 1.0;
    }
    
    let mut y = start_y;
    while y <= end_y {
        let start_screen = camera.world_to_screen(Vec2::new(view_min.x, y));
        let end_screen = camera.world_to_screen(Vec2::new(view_max.x, y));
        
        draw_line(
            start_screen.x,
            start_screen.y,
            end_screen.x,
            end_screen.y,
            1.0,
            color,
        );
        
        y += 1.0;
    }
}

