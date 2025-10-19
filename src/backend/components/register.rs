use crate::{component::Component, tagged_value::TaggedValue, value::Value};

#[derive(Debug, Clone)]
pub struct Register {
    pub stored: Value,
    pub next: Value,
}

impl Register {
    pub fn new(width: u8) -> Self {
        let v = Value::floating(width);
        Self { stored: v, next: v }
    }
}

impl Component for Register {
    fn update_rising_edge(&mut self, _: &[Value]) -> Vec<TaggedValue> {
        self.stored = self.next;
        vec![TaggedValue::new("q", self.stored)]
    }

    fn update_normal(&mut self, inputs: &[Value]) -> Vec<TaggedValue> {
        assert_eq!(inputs.len(), 1);
        self.next = inputs[0];
        vec![TaggedValue::new("q_next", self.next)]
    }

    fn update_falling_edge(&mut self, _: &[Value]) -> Vec<TaggedValue> {
        Vec::new()
    }

    fn send_state_to_frontend(&self) {
        println!("Register stored={:?}", self.stored);
    }
}

