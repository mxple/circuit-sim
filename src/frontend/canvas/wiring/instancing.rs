use crate::{GridCamera, glam::Vec2, frontend::canvas::wiring::{Wire, WireVariant}};
use macroquad::prelude::*;
use miniquad::{RawId, graphics::raw_gl::*};
use std::ffi::CStr;

const VERTEX_SRC: &str = include_str!("shaders/instancing.vert");
const FRAGMENT_SRC: &str = include_str!("shaders/instancing.frag");

pub struct InstancedWireRenderer {
    vao: u32,
    vertex_vbo: u32,
    instance_vbo: u32,
    shader_id: u32,
    max_instances: usize,
    render_target: RenderTarget,
}

impl InstancedWireRenderer {
    pub fn new(max_instances: usize) -> Self {
        let scale = 16;
        let width = scale * WireVariant::NUM_VARIANTS as u32;
        let height = scale;
        let target = render_target(width, height);
        target.texture.set_filter(FilterMode::Nearest);
        set_camera(&Camera2D {
            zoom: vec2(2. / WireVariant::NUM_VARIANTS as f32, 2.),
            target: vec2(0.5 * WireVariant::NUM_VARIANTS as f32 , 0.5),
            render_target: Some(target.clone()),
            ..Default::default()
        });

        for i in 0..WireVariant::NUM_VARIANTS {
            let wire = Wire::new(Vec2::new(i as f32, 0.), WireVariant(i));
            wire.draw();
        }

        set_default_camera();
        unsafe {
            let mut vao = 0;
            let mut vertex_vbo = 0;
            let mut instance_vbo = 0;

            glGenVertexArrays(1, &mut vao);
            glGenBuffers(1, &mut vertex_vbo);
            glGenBuffers(1, &mut instance_vbo);

            // Bind VAO first
            glBindVertexArray(vao);

            let shader_id = create_shader_program();

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
                std::ptr::null(),
            );

            // Setup instance buffer with pre-allocated space
            glBindBuffer(GL_ARRAY_BUFFER, instance_vbo);
            glBufferData(
                GL_ARRAY_BUFFER,
                (max_instances * std::mem::size_of::<(Vec2, f32)>()) as _,
                std::ptr::null(),
                GL_DYNAMIC_DRAW, // Since we'll update this frequently
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
                std::ptr::null(),
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

            // Unbind everything
            glBindVertexArray(0);
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            Self {
                vao,
                vertex_vbo,
                instance_vbo,
                shader_id,
                max_instances,
                render_target: target,
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

            let gl_context = get_internal_gl();
            let RawId::OpenGl(texture_id) = gl_context.quad_context.texture_raw_id(self.render_target.texture.raw_miniquad_id());
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, texture_id);

            let loc = glGetUniformLocation(self.shader_id, b"u_texture\0".as_ptr() as *const _);
            glUniform1i(loc, 0);


            // Draw
            glBindVertexArray(self.vao);
            glDrawArraysInstanced(GL_TRIANGLES, 0, 6, wire_connections.len() as _);

            // Clean up
            glBindVertexArray(0);
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

const WIRE_VERTICES: [f32; 12] = [
    0.0, 0.0, // bl
    1.0, 0.0, // br
    1.0, 1.0, // tr
    0.0, 0.0, // bl
    1.0, 1.0, // tr
    0.0, 1.0, // tl
];
