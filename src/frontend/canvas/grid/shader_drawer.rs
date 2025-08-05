use macroquad::prelude::*;
use miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use super::GridDrawImpl;

use crate::frontend::canvas::camera::GridCamera;

pub struct ShaderDrawer {
    material: Material,
}

impl ShaderDrawer {
    pub fn new() -> ShaderDrawer {
        let vertex_shader = include_str!("shaders/grid.vert");
        let fragment_shader = include_str!("shaders/grid.frag");

        let pipeline_params = PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            alpha_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Zero,
                BlendFactor::One,
            )),
            ..Default::default()
        };
        let material = load_material(
            ShaderSource::Glsl {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("u_view_min", UniformType::Float2),
                    UniformDesc::new("u_view_max", UniformType::Float2),
                    UniformDesc::new("u_resolution", UniformType::Float2),
                    UniformDesc::new("u_color", UniformType::Float4),
                ],
                pipeline_params,
                ..Default::default()
            },
        ).unwrap();

        Self {material}
    }
}

impl GridDrawImpl for ShaderDrawer {
    fn draw_grid(&self, camera: &GridCamera, color: &Vec4) {
        let (view_min, view_max) = camera.get_view_bounds();

        self.material.set_uniform("u_view_min", view_min.to_array());
        self.material.set_uniform("u_view_max", view_max.to_array());
        self.material.set_uniform("u_resolution", [screen_width(), screen_height()]);
        self.material.set_uniform("u_color", color.to_array());

        gl_use_material(&self.material);
        draw_rectangle(view_min.x, view_min.y,
                       view_max.x - view_min.x,
                       view_max.y - view_min.y,
                       WHITE);
        gl_use_default_material();
    }

}
