use std::collections::VecDeque;
use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use crate::{component::Component, value::Value};

struct TaggedValue {
    inp: usize,
    out: usize,
    val: Value,
}

/// main circuit model
pub struct Model {
    graph: Graph<Box<dyn Component>, TaggedValue>,
    queue: VecDeque<NodeIndex>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            queue: VecDeque::new(),
        }
    }

    /// add a component node, returns its NodeIndex
    pub fn add_component(&mut self, c: Box<dyn Component>) -> NodeIndex {
        self.graph.add_node(c)
    }

    /// connect nodes with a tagged wire
    pub fn connect(
        &mut self,
        src: NodeIndex,
        dst: NodeIndex,
        inp: usize,
        out: usize,
        val: Value,
    ) -> EdgeIndex {
        self.graph.add_edge(src, dst, TaggedValue {inp, out, val})
    }

    /// enqueue a component for processing
    pub fn enqueue(&mut self, node: NodeIndex) {
        if !self.queue.contains(&node) {
            self.queue.push_back(node);
        }
    }

    /// helper: collect inputs to a node from incoming edges
    fn collect_inputs(&self, node: NodeIndex) -> Vec<Value> {
        let num_inputs = self.graph[node].num_inputs();
        if num_inputs == 0 {
            self.graph
                .edges_directed(node, Direction::Incoming)
                .map(|tv| tv.weight().val).collect()
        } else {
            let mut inputs = vec![Value::default(); num_inputs];
            self.graph
                .edges_directed(node, Direction::Incoming)
                .for_each(|tv| inputs[tv.weight().out] = tv.weight().val);
            inputs
        }
    }

    /// helper: write outputs from a component to connected edges
    fn write_outputs(&mut self, node: NodeIndex, outputs: Vec<Value>) {
        let edge_indices = self.graph.edges_directed(node, Direction::Outgoing).map(|e|
            e.id()).collect::<Vec<_>>();
        for e in edge_indices {
            let edge = &mut self.graph.edge_weight_mut(e).unwrap();
            edge.val = outputs[edge.inp];
        }
    }

    fn process_queue(&mut self) {
        while let Some(node) = self.queue.pop_front() {
            let inputs = self.collect_inputs(node);
            let outputs = {
                let comp = &mut self.graph[node];
                comp.update(&inputs)
            };
            self.write_outputs(node, outputs);
        }
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }
}

