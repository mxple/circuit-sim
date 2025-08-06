use macroquad::prelude::*;
use super::GridDrawImpl;

use crate::frontend::{canvas::camera::GridCamera, util::shader::create_shader_program};

use miniquad::graphics::raw_gl::*;

const VERTEX_SRC: &str = include_str!("shaders/instancing.vert");
const FRAGMENT_SRC: &str = include_str!("shaders/instancing.frag");

pub struct InstancedDrawer {
    vao: u32,
    vertex_vbo: u32,
    instance_vbo: u32,
    shader_id: u32,
}

const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
const MAX_INSTANCES: usize = 1 << 12;

const LINE_VERTICES: [f32; 4] = [
    0.0, 0.0, // start
    1.0, 0.0, // end
];

#[repr(C)]
#[derive(Copy, Clone)]
struct GridLineInstance {
    start_pos: Vec2,
    end_pos: Vec2,
}

impl InstancedDrawer {
    pub fn new() -> Self {
        unsafe {
            let mut prev_vao = 0;
            glGetIntegerv(GL_VERTEX_ARRAY_BINDING, &mut prev_vao);

            let mut vao = 0;
            let mut vertex_vbo = 0;
            let mut instance_vbo = 0;

            glGenVertexArrays(1, &mut vao);
            glGenBuffers(1, &mut vertex_vbo);
            glGenBuffers(1, &mut instance_vbo);

            // Bind VAO first
            glBindVertexArray(vao);

            // Setup vertex buffer for line geometry
            glBindBuffer(GL_ARRAY_BUFFER, vertex_vbo);
            glBufferData(
                GL_ARRAY_BUFFER,
                (LINE_VERTICES.len() * std::mem::size_of::<f32>()) as _,
                LINE_VERTICES.as_ptr() as *const _,
                GL_STATIC_DRAW,
            );

            // Vertex positions (location 0)
            glEnableVertexAttribArray(0);
            glVertexAttribPointer(
                0,
                2,
                GL_FLOAT,
                GL_FALSE.try_into().unwrap(),
                0,
                0 as _,
            );

            // Setup instance buffer with pre-allocated space
            glBindBuffer(GL_ARRAY_BUFFER, instance_vbo);
            glBufferData(
                GL_ARRAY_BUFFER,
                (MAX_INSTANCES * std::mem::size_of::<GridLineInstance>()) as _,
                0 as _,
                GL_DYNAMIC_DRAW,
            );

            // Instance data setup
            let stride = std::mem::size_of::<GridLineInstance>() as i32;

            // Instance start position (Vec2) - location 1
            glEnableVertexAttribArray(1);
            glVertexAttribPointer(
                1,
                2,
                GL_FLOAT,
                GL_FALSE.try_into().unwrap(),
                stride,
                0 as _,
            );
            glVertexAttribDivisor(1, 1);

            // Instance end position (Vec2) - location 2
            glEnableVertexAttribArray(2);
            glVertexAttribPointer(
                2,
                2,
                GL_FLOAT,
                GL_FALSE.try_into().unwrap(),
                stride,
                (std::mem::size_of::<Vec2>()) as _,
            );
            glVertexAttribDivisor(2, 1);

            let shader_id = create_shader_program(VERTEX_SRC, FRAGMENT_SRC);

            // Unbind everything
            glBindVertexArray(prev_vao as u32);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            Self {
                vao,
                vertex_vbo,
                instance_vbo,
                shader_id,
            }
        }
    }

    fn calculate_grid_spacing(camera: &GridCamera) -> f32 {
        let mut pixels = camera.get_cell_pixels();
        let mut spacing = 1.0;
    
        // limit to min 16 pixels per grid cell
        while pixels < 8.0 {
            pixels *= 2.0;
            spacing *= 2.0;
        }
        spacing
    }

    fn generate_grid_lines(&self, camera: &GridCamera) -> Vec<GridLineInstance> {
        let (view_min, view_max) = camera.get_view_bounds();
        
        let start_x = view_min.x.floor();
        let end_x = view_max.x.ceil();
        let start_y = view_min.y.floor();
        let end_y = view_max.y.ceil();
        
        let mut instances = Vec::new();

        let grid_spacing = Self::calculate_grid_spacing(camera);
        
        let start_x = (view_min.x / grid_spacing).floor() * grid_spacing;
        let start_y = (view_min.y / grid_spacing).floor() * grid_spacing;
        
        // Generate vertical lines
        let mut x = start_x;
        while x <= end_x {
            instances.push(GridLineInstance {
                start_pos: Vec2::new(x, view_min.y),
                end_pos: Vec2::new(x, view_max.y),
            });
            x += grid_spacing;
        }
        
        // Generate horizontal lines
        let mut y = start_y;
        while y <= end_y {
            instances.push(GridLineInstance {
                start_pos: Vec2::new(view_min.x, y),
                end_pos: Vec2::new(view_max.x, y),
            });
            y += grid_spacing;
        }
        
        instances
    }
}

impl GridDrawImpl for InstancedDrawer {
    fn draw_grid(&self, camera: &GridCamera, color: &Vec4) {
        unsafe {
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            let mut color_with_alpha = color.clone();
            color_with_alpha.w = camera.zoom / (200. - 10.);

            let mut prev_vao = 0;
            glGetIntegerv(GL_VERTEX_ARRAY_BINDING, &mut prev_vao);

            glUseProgram(self.shader_id);

            // Update uniforms
            let projection = camera.matrix();

            let loc = glGetUniformLocation(self.shader_id, b"projection\0".as_ptr() as *const _);
            glUniformMatrix4fv(
                loc,
                1,
                GL_FALSE.try_into().unwrap(),
                projection.to_cols_array().as_ptr(),
            );

            let loc = glGetUniformLocation(self.shader_id, b"u_color\0".as_ptr() as *const _);
            glUniform4fv(
                loc,
                1,
                color_with_alpha.to_array().as_ptr(),
            );

            // Generate grid line instances
            let grid_lines = self.generate_grid_lines(camera);
            
            if !grid_lines.is_empty() {
                // Update instance data
                let size = (grid_lines.len() * std::mem::size_of::<GridLineInstance>()) as _;
                glBindBuffer(GL_ARRAY_BUFFER, self.instance_vbo);
                glBufferSubData(
                    GL_ARRAY_BUFFER,
                    0,
                    size,
                    grid_lines.as_ptr() as *const _,
                );

                // Draw
                glBindVertexArray(self.vao);
                glDrawArraysInstanced(GL_LINES, 0, 2, grid_lines.len() as _);
            }

            // Clean up
            glBindVertexArray(prev_vao as u32);
            glUseProgram(0);
        }
    }
}

impl Drop for InstancedDrawer {
    fn drop(&mut self) {
        unsafe {
            glDeleteVertexArrays(1, &self.vao);
            glDeleteBuffers(1, &self.vertex_vbo);
            glDeleteBuffers(1, &self.instance_vbo);
            glDeleteProgram(self.shader_id);
        }
    }
}
