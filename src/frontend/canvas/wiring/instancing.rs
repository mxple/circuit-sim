use miniquad::{
    RawId,
    graphics::raw_gl::*,
};
use crate::{
    GridCamera,
    glam::{Vec2, Vec4},
};
use macroquad::prelude::*;
use std::ffi::CStr;

const VERTEX_SRC: &str = r#"
#version 100
attribute vec2 a_position;
attribute vec2 a_offset;
attribute float a_rotation;
varying vec2 v_uv;
uniform mat4 projection;

void main() {
    vec2 position = a_position + a_offset;
    
    vec2 center = vec2(0.5, 0.5);
    vec2 pos = position - center;

    float angle = radians(45.0) * a_rotation; // replace with uniform if dynamic
    float cos_a = cos(angle);
    float sin_a = sin(angle);

    mat2 rot = mat2(
        cos_a, -sin_a,
        sin_a,  cos_a
    );

    vec2 rotated = rot * pos + center;
    rotated = position;

    gl_Position = projection * vec4(rotated, 0., 1.);
    v_uv = a_position; // NDC to UV
}
"#;

const FRAGMENT_SRC: &str = r#"
#version 100
precision mediump float;
varying vec2 v_uv;

void main() {
    // gl_FragColor = texture2D(u_texture, v_uv);
    // gl_FragColor = vec4(v_uv, 0., 1.);
    // gl_FragColor = vec4(0.2);
    gl_FragColor = vec4(1., 0., 0., 1.);
}
"#;


unsafe fn compile_shader(src: &str, shader_type: u32) -> u32 {
    let shader = glCreateShader(shader_type);
    let c_str = std::ffi::CString::new(src).unwrap();
    glShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
    glCompileShader(shader);

    // Optional: Check for compile errors here
    let mut success = 0;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);
    if success == 0 {
        // Compilation failed, get the log
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

pub unsafe fn create_shader_program() -> u32 {
    let vertex_shader = compile_shader(VERTEX_SRC, GL_VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SRC, GL_FRAGMENT_SHADER);

    let program = glCreateProgram();
    glAttachShader(program, vertex_shader);
    glAttachShader(program, fragment_shader);
    glLinkProgram(program);

    // Optional: Check for linking errors here
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

    program
}

fn to_ndc(x: f32, y: f32, screen_w: f32, screen_h: f32) -> Vec2 {
    let ndc_x = (x / screen_w) * 2.0 - 1.0;
    let ndc_y = 1.0 - (y / screen_h) * 2.0;
    Vec2::new(ndc_x, ndc_y)
}

pub fn instanced_draw(
    shader_id: u32,
    camera: &GridCamera,
    wire_connections: &[(Vec2, f32)],
) {
    unsafe {
        // Vertex data: 2D quad (x, y) in normalized device coords (NDC)
        let vertices: [f32; 12] = [
            0., 0., // bottom-left
             1., 0., // bottom-right
            0.,  1., // top-left

             1., 0., // bottom-right
             1.,  1., // top-right
            0.,  1., // top-left
        ];

        let wire_vertices: [f32; 12] = [
            0.5,    0.4,   // bl
            1.,     0.4,   // br
            1.,     0.6,   // tr

            0.5,    0.4,    // br
            1.,     0.6,    // tr
            0.5,    0.6,    // tl
        ];


        let mut vbo: u32 = 0;
        glGenBuffers(1, &mut vbo);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            (wire_vertices.len() * std::mem::size_of::<f32>()) as _,
            wire_vertices.as_ptr() as *const _,
            GL_STATIC_DRAW,
        );
        
        let mut instance_vbo: u32 = 0;
        glGenBuffers(1, &mut instance_vbo);
        glBindBuffer(GL_ARRAY_BUFFER, instance_vbo);
        glBufferData(
            GL_ARRAY_BUFFER,
            (wire_connections.len() * std::mem::size_of::<(Vec2, f32)>()) as _,
            wire_connections.as_ptr() as *const _,
            GL_STATIC_DRAW,
        );

        let mut vao: u32 = 0;
        glGenVertexArrays(1, &mut vao);
        glBindVertexArray(vao);

        // Vertex positions (location 0)
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        glEnableVertexAttribArray(0);
        glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE.try_into().unwrap(), std::mem::size_of::<Vec2>() as i32, std::ptr::null());

        // Instance offsets (location 1)
        let stride = std::mem::size_of::<(Vec2, f32)>() as i32;
        glBindBuffer(GL_ARRAY_BUFFER, instance_vbo);
        glEnableVertexAttribArray(1);
        glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE.try_into().unwrap(), stride, std::ptr::null());

        glEnableVertexAttribArray(2);
        glVertexAttribPointer(2, 1, GL_FLOAT, GL_FALSE.try_into().unwrap(), stride, std::mem::size_of::<Vec2>() as *const _);

        // Use your shader program
        glUseProgram(shader_id);
        let projection = camera.matrix();
        let loc = glGetUniformLocation(shader_id, b"projection\0".as_ptr() as *const _);
        glUniformMatrix4fv(loc, 1, GL_FALSE.try_into().unwrap(), projection.to_cols_array().as_ptr());

        // Bind VAO and draw
        glBindVertexArray(vao);
        glDrawArraysInstanced(GL_TRIANGLES, 0, 6, wire_connections.len() as _);
    }
}
