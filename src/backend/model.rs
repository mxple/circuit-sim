use std::collections::VecDeque;
use petgraph::graph::{Graph, NodeIndex, EdgeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use crate::{component::Component, tagged_value::TaggedValue, value::Value};

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
        tag: impl Into<String>,
        value: Value,
    ) -> EdgeIndex {
        self.graph.add_edge(src, dst, TaggedValue::new(tag, value))
    }

    /// enqueue a component for processing
    pub fn enqueue(&mut self, node: NodeIndex) {
        if !self.queue.contains(&node) {
            self.queue.push_back(node);
        }
    }

    /// helper: collect inputs to a node from incoming edges
    fn collect_inputs(&self, node: NodeIndex) -> Vec<Value> {
        self.graph
            .neighbors_directed(node, Direction::Incoming)
            .filter_map(|src| {
                self.graph.find_edge(src, node).map(|e| self.graph[e].value)
            })
            .collect()
    }

    /// helper: write outputs from a component to connected edges
    fn write_outputs(&mut self, node: NodeIndex, outputs: Vec<TaggedValue>) {
        let edge_indices = self.graph.edges_directed(node, Direction::Outgoing).map(|e|
            e.id()).collect::<Vec<_>>();
        for e in edge_indices {
            let edge = &mut self.graph.edge_weight_mut(e).unwrap();

            // match output tag
            if let Some(tv) = outputs.iter().find(|tv| tv.tag == edge.tag) {
                edge.value = tv.value;
            } else {
                debug_assert!(
                    false,
                    "component at {:?} did not produce output for tag '{}'",
                    node,
                    edge.tag
                );
            }
        }
    }

    fn process_queue<F>(&mut self, mut phase_fn: F)
    where
        F: FnMut(&mut dyn Component, &[Value]) -> Vec<TaggedValue>,
    {
        while let Some(node) = self.queue.pop_front() {
            let inputs = self.collect_inputs(node);
            let outputs = {
                let comp = &mut self.graph[node];
                phase_fn(comp.as_mut(), &inputs)
            };
            self.write_outputs(node, outputs);
        }
    }

    pub fn run_normal(&mut self) {
        self.process_queue(|c, inputs| c.update_normal(inputs));
    }

    pub fn run_rising_edge(&mut self) {
        self.process_queue(|c, inputs| c.update_rising_edge(inputs));
    }

    pub fn run_falling_edge(&mut self) {
        self.process_queue(|c, inputs| c.update_falling_edge(inputs));
    }

    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }
}

