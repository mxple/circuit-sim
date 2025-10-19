use crate::{component::Component, tagged_value::TaggedValue, value::Value};

#[derive(Debug, Clone)]
pub struct Wire {
    pub width: u8,
}

impl Wire {
    pub fn new(width: u8) -> Self {
        Self { width }
    }

    fn update(&mut self, inputs: &[Value]) -> Vec<TaggedValue> {
        assert!(inputs.len() == 1);
        let out = inputs[0];
        vec![TaggedValue::new("out", out)]
    }
}

impl Component for Wire {
    fn update_rising_edge(&mut self, inputs: &[Value]) -> Vec<TaggedValue> {
        self.update(inputs)
    }

    fn update_normal(&mut self, inputs: &[Value]) -> Vec<TaggedValue> {
        self.update(inputs)
    }

    fn update_falling_edge(&mut self, inputs: &[Value]) -> Vec<TaggedValue> {
        self.update(inputs)
    }

    fn send_state_to_frontend(&self) {
        println!("Wire width={} (stateless)", self.width);
    }
}

