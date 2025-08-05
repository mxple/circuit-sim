use crate::{GridCamera, glam::Vec2};
use macroquad::prelude::*;
use miniquad::graphics::raw_gl::*;
use std::ffi::CStr;

const VERTEX_SRC: &str = include_str!("shaders/instancing.vert");
const FRAGMENT_SRC: &str = include_str!("shaders/instancing.frag");

pub struct InstancedWireRenderer {
    vao: u32,
    vertex_vbo: u32,
    instance_vbo: u32,
    shader_id: u32,
    max_instances: usize,
}

const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;

impl InstancedWireRenderer {
    pub fn new(max_instances: usize) -> Self {
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

            // Setup vertex buffer
            glBindBuffer(GL_ARRAY_BUFFER, vertex_vbo);
            glBufferData(
                GL_ARRAY_BUFFER,
                (WIRE_VERTICES.len() * std::mem::size_of::<f32>()) as _,
                WIRE_VERTICES.as_ptr() as *const _,
                GL_STATIC_DRAW,
            );

            // Vertex positions (location 0) - FIXED stride
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
                (max_instances * std::mem::size_of::<(Vec2, f32)>()) as _,
                0 as _,
                GL_DYNAMIC_DRAW,
            );

            // Instance data setup
            let stride = std::mem::size_of::<(Vec2, f32)>() as i32;

            // Instance offset (Vec2) - location 1
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

            // Instance rotation (f32) - location 2
            glEnableVertexAttribArray(2);
            glVertexAttribPointer(
                2,
                1,
                GL_FLOAT,
                GL_FALSE.try_into().unwrap(),
                stride,
                std::mem::size_of::<Vec2>() as *const _,
            );
            glVertexAttribDivisor(2, 1);

            let shader_id = create_shader_program();

            // Unbind everything
            glBindVertexArray(prev_vao as u32);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            Self {
                vao,
                vertex_vbo,
                instance_vbo,
                shader_id,
                max_instances,
            }
        }
    }

    pub fn instanced_draw(&self, wire_connections: &[(Vec2, f32)], camera: &GridCamera) {
        if wire_connections.is_empty() {
            return;
        }

        if wire_connections.len() > self.max_instances {
            eprintln!(
                "Warning: trying to draw {} instances but max is {}",
                wire_connections.len(),
                self.max_instances
            );
            return;
        }

        unsafe {
            let mut prev_vao = 0;
            glGetIntegerv(GL_VERTEX_ARRAY_BINDING, &mut prev_vao);

            glUseProgram(self.shader_id);

            // Update projection matrix
            let projection = camera.matrix();
            let loc = glGetUniformLocation(self.shader_id, b"projection\0".as_ptr() as *const _);
            glUniformMatrix4fv(
                loc,
                1,
                GL_FALSE.try_into().unwrap(),
                projection.to_cols_array().as_ptr(),
            );

            // Update instance data - FIXED glBufferSubData call
            let size: i32 = std::mem::size_of_val(wire_connections) as _;
            glBindBuffer(GL_ARRAY_BUFFER, self.instance_vbo);
            glBufferSubData(
                GL_ARRAY_BUFFER,
                0,
                size as _,
                wire_connections.as_ptr() as *const _,
            );

            // Draw
            glBindVertexArray(self.vao);
            glDrawArraysInstanced(GL_TRIANGLES, 0, 6, wire_connections.len() as _);

            // Clean up
            glBindVertexArray(prev_vao as u32);
            glUseProgram(0);
        }
    }
}

fn compile_shader(src: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = glCreateShader(shader_type);
        let c_str = std::ffi::CString::new(src).unwrap();
        glShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        glCompileShader(shader);

        // Check for compile errors
        let mut success = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0;
            glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut log_len);
            let mut buffer = vec![0u8; log_len as usize];
            glGetShaderInfoLog(
                shader,
                log_len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );
            let message = CStr::from_ptr(buffer.as_ptr() as *const _)
                .to_string_lossy()
                .into_owned();
            panic!(
                "Shader compilation failed (type={}):\n{}",
                shader_type, message
            );
        }

        shader
    }
}

pub fn create_shader_program() -> u32 {
    unsafe {
        let vertex_shader = compile_shader(VERTEX_SRC, GL_VERTEX_SHADER);
        let fragment_shader = compile_shader(FRAGMENT_SRC, GL_FRAGMENT_SHADER);

        let program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);

        // Check for linking errors
        let mut success = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0;
            glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut log_len);
            let mut buffer = vec![0u8; log_len as usize];
            glGetProgramInfoLog(
                program,
                log_len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr() as *mut _,
            );
            let message = CStr::from_ptr(buffer.as_ptr() as *const _)
                .to_string_lossy()
                .into_owned();
            panic!("Shader linking failed:\n{}", message);
        }

        // Clean up shaders
        glDeleteShader(vertex_shader);
        glDeleteShader(fragment_shader);

        program
    }
}

impl Drop for InstancedWireRenderer {
    fn drop(&mut self) {
        unsafe {
            glDeleteVertexArrays(1, &self.vao);
            glDeleteBuffers(1, &self.vertex_vbo);
            glDeleteBuffers(1, &self.instance_vbo);
            glDeleteProgram(self.shader_id);
        }
    }
}

const WIRE_VERTICES: [f32; 12] = [
    0.5, 0.4, // bl
    1.0, 0.4, // br
    1.0, 0.6, // tr
    0.5, 0.4, // bl
    1.0, 0.6, // tr
    0.5, 0.6, // tl
];
