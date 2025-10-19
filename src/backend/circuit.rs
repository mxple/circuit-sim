use std::{collections::HashMap, rc::Rc};

use crate::backend::value::Value;

pub struct SignalId(u32);

// Directed graph of components
pub struct Circuit {
    nodes: Vec<Box<dyn Node>>,
    signals: Vec<Value>,
    edges_to: HashMap<SignalId, Vec<Rc<dyn Node>>>, // one to many
    edges_from: HashMap<SignalId, Rc<dyn Node>>,    // many to one
    inputs: Vec<Box<dyn Node>>,
    outputs: Vec<Box<dyn Node>>,
}

struct NodeState {
    output_signals: Vec<SignalId>,
    outputs: Vec<(SignalId, Value)>,
}

struct AndGate {
    num_inputs: u8,
    bit_width: u8,
    node_state: NodeState,
}

impl Node for AndGate {
    fn read_values(&self) {
        let inputs = ...
    }

    fn get_output(&self) -> &[( SignalId , Value )] {
        &self.node_state.outputs
    }
}

trait Node {
    // fn create(&self, );
    fn read_values(&self);
    fn get_output(&self) -> &[( SignalId , Value )];
}
