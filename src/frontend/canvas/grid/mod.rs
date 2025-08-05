use macroquad::math::Vec4;
use naive_drawer::NaiveDrawer;
use shader_drawer::ShaderDrawer;
use instanced_drawer::InstancedDrawer;

use super::camera::GridCamera;

mod instanced_drawer;
mod naive_drawer;
mod shader_drawer;

#[allow(unused)]
pub enum GridDrawOptions {
    Naive,
    Shader,
    Instanced,
}

pub trait GridDrawImpl {
    fn draw_grid(&self, camera: &GridCamera, color: &Vec4);
}

pub struct GridDrawer {
    engine: Box<dyn GridDrawImpl>,
    color: Vec4,
}

impl GridDrawer {
    pub fn new(draw_type: GridDrawOptions, color: Vec4) -> Self {
        let engine: Box<dyn GridDrawImpl> = match draw_type {
            GridDrawOptions::Naive => {
                Box::new(NaiveDrawer::new())
            },
            GridDrawOptions::Shader => {
                Box::new(ShaderDrawer::new())
            },
            GridDrawOptions::Instanced => {
                Box::new(InstancedDrawer::new())
            },
        };
        Self { engine, color }
    }

    pub fn draw_grid(&self, camera: &GridCamera) {
        self.engine.draw_grid(camera, &self.color);
    }
}
