use crate::{component::Component, value::Value};

#[derive(Debug, Clone)]
pub struct Wire {
    pub width: u8,
}

impl Wire {
    pub fn new(width: u8) -> Self {
        Self { width }
    }
}

impl Component for Wire {
    fn init(&mut self) -> Vec<Value> {
        vec![Value::floating(self.width)]
    }
    fn update(&mut self, inputs: &[Value]) -> Vec<Value> {
        if inputs.is_empty() {
            return vec![Value::floating(self.width)];
        }
        let short = inputs.iter().all(|&a| a == inputs[0]);
        if short {
            vec![inputs[0].burn()]
        } else {
            vec![inputs[0]]
        }
    }

    fn num_inputs(&self) -> usize {
        0
    }
    fn num_outputs(&self) -> usize {
        0
    }
}

