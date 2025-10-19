use crate::{value::Value, tagged_value::TaggedValue};

/// all circuit components implement this
pub trait Component {
    /// rising clock edge update (e.g. latch inputs)
    fn update_rising_edge(&mut self, inputs: &[Value]) -> Vec<TaggedValue>;

    /// normal combinational propagation
    fn update_normal(&mut self, inputs: &[Value]) -> Vec<TaggedValue>;

    /// falling edge update
    fn update_falling_edge(&mut self, inputs: &[Value]) -> Vec<TaggedValue>;

    /// used for UI / debugging
    fn send_state_to_frontend(&self);
}

