use crate::{value::Value};

/// all circuit components implement this
pub trait Component {
    fn init(&mut self) -> Vec<Value>;
    fn update(&mut self, inputs: &[Value]) -> Vec<Value>;

    fn num_inputs(&self) -> usize;
    fn num_outputs(&self) -> usize;
}

