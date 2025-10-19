use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use crate::backend::value::Value;

pub type NodeId = u32;

#[derive(Debug, Clone)]
pub struct NodeState {
    pub outputs: Vec<Value>,
    pub metadata: HashMap<String, Value>, // for components like RAM
    pub input_hash: u64, // for oscillation detection
    pub generation: u32, // helps distinguish state changes
}

impl NodeState {
    pub fn new(num_inputs: usize, num_outputs: usize) -> Self {
        Self {
            outputs: vec![Value::unknown(Value::BitWidth(1)); num_outputs],
            metadata: HashMap::new(),
            input_hash: 0,
            generation: 0,
        }
    }
}

pub trait NodeLogic: Send + Sync {
    fn compute(&self, state: &mut NodeState) -> Vec<NodeId>;
    fn name(&self) -> &str;
    fn input_count(&self) -> usize;
    fn output_count(&self) -> usize;
}

pub struct Node {
    pub id: NodeId,
    pub logic: Box<dyn NodeLogic>,
    pub state: NodeState,
    pub input_connections: Vec<Vec<NodeId>>, // input_pin -> [source_nodes]
    pub output_connections: Vec<Vec<NodeId>>, // output_pin -> [destination_nodes]
}

impl Node {
    pub fn new(id: NodeId, logic: Box<dyn NodeLogic>) -> Self {
        let input_count = logic.input_count();
        let output_count = logic.output_count();
        
        Self {
            id,
            state: NodeState::new(input_count, output_count),
            logic,
            input_connections: vec![Vec::new(); input_count],
            output_connections: vec![Vec::new(); output_count],
        }
    }

    pub fn read_values(&mut self, circuit_state: &HashMap<NodeId, Vec<Value>>) {
        for (pin, sources) in self.input_connections.iter().enumerate() {
            if let Some(source_id) = sources.first() {
                if let Some(source_outputs) = circuit_state.get(source_id) {
                    if let Some(value) = source_outputs.get(0) { // assume single output for now
                        self.state.inputs[pin] = value.clone();
                    }
                }
            }
        }
    }

    pub fn update(&mut self) -> Vec<NodeId> {
        // Run logic
        self.logic.compute(&mut self.state)
    }
}
