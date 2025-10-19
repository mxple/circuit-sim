use crate::{component::Component, tagged_value::TaggedValue, value::Value};

#[derive(Debug, Clone)]
pub struct AndGate {
    pub width: u8,
}

impl AndGate {
    pub fn new(width: u8) -> Self {
        Self { width }
    }
}

impl Component for AndGate {
    fn update_rising_edge(&mut self, _: &[Value]) -> Vec<TaggedValue> {
        Vec::new()
    }

    fn update_normal(&mut self, inputs: &[Value]) -> Vec<TaggedValue> {
        assert!(!inputs.is_empty());
        let mut out = inputs[0];
        for &v in &inputs[1..] {
            out = out & v;
        }
        vec![TaggedValue::new("out", out)]
    }

    fn update_falling_edge(&mut self, _: &[Value]) -> Vec<TaggedValue> {
        Vec::new()
    }

    fn send_state_to_frontend(&self) {
        println!("AndGate width={} (stateless)", self.width);
    }
}

