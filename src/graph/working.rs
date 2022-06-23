use super::history::{Hash, History};
use std::collections::HashMap;

enum NodeData {
    Inactive(Box<[u8]>),
    Active(Vec<u8>),
}

pub struct Node {
    data: NodeData,
}

impl Node {
    pub fn data(&self) -> &[u8] {
        match &self.data {
            NodeData::Inactive(b) => b.as_ref(),
            NodeData::Active(v) => v.as_slice(),
        }
    }
}

pub struct Working {
    history: History,
    nodes: HashMap<Hash, Box<[Node]>>,
}

impl Working {
    pub fn new(history: History) -> Self {
        Self {
            history,
            nodes: HashMap::new(),
        }
    }

    pub fn assemble(&mut self) {}

    pub fn nodes(&self) -> &[Node] {
        todo!()
    }
}
