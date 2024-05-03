pub mod edge;
pub mod node;

use crate::{Game, GameState};

use std::ops::{Index, IndexMut};

use self::node::Node;

pub struct Tree {
    root: i32,
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
    pub fn new() -> Self {
        Tree {
            root: -1,
            nodes: vec![],
        }
    }

    pub fn root(&self) -> i32 {
        self.root
    }

    pub fn add(&mut self, node: Node) -> i32 {
        let index = self.len();
        self.nodes.push(node);

        if self.root == -1 {
            self.set_root(index)
        }

        index
    }

    pub fn subtree<G: Game>(&mut self, root: &G, pos: &Option<G>) {
        if self.is_empty() {
            self.reset();
            return;
        }

        if let Some(pos) = pos {
            let node = self.find(self.root, root, pos, 2);
            if node == -1 {
                self.reset();
                return;
            }

            self.set_root(node)
        }
    }

    pub fn reset(&mut self) {
        self.clear();

        let node = self.add(Node::new(GameState::Ongoing, -1));
        self.set_root(node);
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> i32 {
        self.nodes.len() as i32
    }

    fn set_root(&mut self, root: i32) {
        self.root = root;
    }

    fn find<G: Game>(&self, index: i32, child: &G, board: &G, depth: usize) -> i32 {
        if child.equals(board) {
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
