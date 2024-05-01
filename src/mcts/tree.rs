use super::node::Node;
use crate::GameState;
use std::ops::{Index, IndexMut};

pub struct Tree {
    nodes: Vec<Node>,
}

impl Index<i32> for Tree {
    type Output = Node;

    fn index(&self, index: i32) -> &Self::Output {
        &self.nodes[index as usize]
    }
}

impl IndexMut<i32> for Tree {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.nodes[index as usize]
    }
}

impl Tree {
    pub fn new(state: GameState) -> Self {
        let root_node = Node::from(state);

        Tree {
            nodes: vec![root_node],
        }
    }

    pub fn root(&self) -> Node {
        self.nodes[0].clone()
    }

    pub fn add(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> i32 {
        self.nodes.len() as i32
    }
}
