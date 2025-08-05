use macroquad::prelude::*;
use miniquad::graphics::raw_gl::*;
use std::ffi::CStr;

pub fn create_shader_program(vertex_src: &str, fragment_src: &str) -> u32 {
    unsafe {
        let vertex_shader = compile_shader(vertex_src, GL_VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_src, GL_FRAGMENT_SHADER);

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

