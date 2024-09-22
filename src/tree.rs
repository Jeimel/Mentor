pub mod edge;
pub mod hash;
pub mod node;

use self::{edge::Edge, hash::HashTable, node::Node};
use crate::{Game, GameState};
use std::ops::{Index, IndexMut};

pub struct Tree {
    root: i32,
    nodes: Vec<Node>,
    table: HashTable,
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
    pub fn new(capacity: usize) -> Self {
        Tree {
            root: -1,
            nodes: vec![],
            table: HashTable::new(capacity),
        }
    }

    pub fn root(&self) -> i32 {
        self.root
    }

    pub fn add(&mut self, state: GameState, hash: u64, parent: i32) -> i32 {
        let index = self.len();

        self.nodes.push(Node::new(state, hash, parent));

        if self.root == -1 {
            self.set_root(index);
        }

        index
    }

    pub fn subtree<G: Game>(&mut self, root: &G, pos: &Option<G>) {
        if self.is_empty() {
            self.reset(root);
            return;
        }

        if let Some(pos) = pos {
            let node = self.find(self.root, root, pos, 2);
            if node == -1 {
                self.reset(pos);
                return;
            }

            self.set_root(node);
        }
    }

    pub fn propagate(&mut self, index: i32, reward: f32) {
        self[index].propagate(reward);

        let node = &self[index];
        self.table.insert(node.hash(), node.visits(), node.value());
    }

    pub fn reset<G: Game>(&mut self, pos: &G) {
        self.nodes.clear();

        let node = self.add(GameState::Ongoing, pos.hash(), -1);
        self.set_root(node);
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> i32 {
        self.nodes.len() as i32
    }

    pub fn edge(&self, index: i32, edge: usize) -> &Edge {
        &self[index].actions()[edge]
    }

    pub fn edge_mut(&mut self, index: i32, edge: usize) -> &mut Edge {
        &mut self[index].actions_mut()[edge]
    }

    fn set_root(&mut self, root: i32) {
        self.root = root;
    }

    fn find<G: Game>(&self, index: i32, child: &G, board: &G, depth: usize) -> i32 {
        if child == board {
            return index;
        }

        if index == -1 || depth == 0 {
            return -1;
        }

        let node = &self[index];
        for action in node.actions() {
            let mut child = *child;
            child.make_move(action.mov().into());

            let index = self.find(action.ptr(), &child, board, depth - 1);
            if index != -1 {
                return index;
            }
        }

        -1
    }
}
