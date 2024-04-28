use super::node::Node;
use crate::{Game, GameState};
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

    pub fn len(&self) -> i32 {
        self.nodes.len() as i32
    }

    pub fn select<G: Game>(&mut self, pos: &mut G) -> i32 {
        let mut index = 0;

        loop {
            let result = self[index]
                .actions()
                .iter()
                .enumerate()
                .map(|(index, edge)| {
                    if edge.ptr() == -1 {
                        return (index, f32::INFINITY, -1, edge.mov());
                    }

                    (
                        index,
                        self[edge.ptr()].uct(self[self[edge.ptr()].parent()].visits()),
                        edge.ptr(),
                        edge.mov(),
                    )
                })
                .max_by(|(_, a, _, _), (_, b, _, _)| a.total_cmp(b))
                .map(|(edge, _, index, mov)| (edge, index, mov))
                .unwrap();

            if result.1 == -1 {
                let new_index = self.nodes.len() as i32;
                pos.make_move(result.2.into());

                let node = Node::new(pos.game_state(), index);
                self.add(node);
                self[index].action(result.0).set_ptr(new_index);

                index = new_index;
                break;
            }

            index = result.1;
            pos.make_move(result.2.into());

            if !self[index].is_expanded() || self[index].is_terminal() {
                break;
            }
        }

        index
    }
}
