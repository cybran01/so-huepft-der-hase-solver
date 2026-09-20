use std::collections::{HashMap, VecDeque};

use crate::board::*;

type NodeId = usize;

#[derive(Debug)]
struct SearchNode {
    board: Board,
    state: StateKey,
    parent: Option<NodeId>,
    move_taken: Option<Move>,
}

#[derive(Debug)]
pub struct Graph {
    pub root: Board,
    nodes: Vec<SearchNode>,
    visited: HashMap<StateKey, NodeId>,
}

impl Graph {
    pub fn path_to(&self, end: &Board) -> Vec<Board> {
        let mut path = Vec::new();
        let mut node_id = self.visited[&canonical_key(end)];

        loop {
            let node = &self.nodes[node_id];
            path.push(node.board.clone());
            match node.parent {
                Some(parent) => node_id = parent,
                None => break,
            }
        }

        path.reverse();
        debug_assert_eq!(path.first(), Some(&self.root));
        path
    }

    pub fn moves_to(&self, end: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut node_id = self.visited[&canonical_key(end)];

        while let Some(parent) = self.nodes[node_id].parent {
            moves.push(self.nodes[node_id].move_taken.clone().unwrap());
            node_id = parent;
        }

        moves.reverse();
        moves
    }

    pub fn generate_solution_graph_from_board(board: &Board) -> (Self, Option<Board>) {
        let root = board.clone();
        let root_key = canonical_key(&root);
        let mut nodes = vec![SearchNode {
            board: root.clone(),
            state: root.state_key(),
            parent: None,
            move_taken: None,
        }];
        let mut visited = HashMap::from([(root_key, 0)]);
        let mut queue = VecDeque::from([0]);

        if root.is_in_win_state() {
            return (
                Graph {
                    root,
                    nodes,
                    visited,
                },
                Some(board.clone()),
            );
        }

        while let Some(parent_id) = queue.pop_front() {
            let parent = nodes[parent_id].board.clone();
            let parent_state = nodes[parent_id].state.clone();

            for movement in parent.get_all_moves() {
                let successor_state = parent_state.apply_move(&movement);
                let key = successor_state.canonical();
                if visited.contains_key(&key) {
                    continue;
                }

                let successor = parent.apply_move(&movement).unwrap();
                let node_id = nodes.len();
                visited.insert(key, node_id);
                nodes.push(SearchNode {
                    board: successor.clone(),
                    state: successor_state,
                    parent: Some(parent_id),
                    move_taken: Some(movement),
                });

                if successor.is_in_win_state() {
                    return (
                        Graph {
                            root,
                            nodes,
                            visited,
                        },
                        Some(successor),
                    );
                }

                queue.push_back(node_id);
            }
        }

        (
            Graph {
                root,
                nodes,
                visited,
            },
            None,
        )
    }
}

fn canonical_key(board: &Board) -> StateKey {
    board.state_key().canonical()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constellation_60_keeps_shortest_solution_depth() {
        let mut board = Board::new();
        board.add_bunny(Bunny { pos: (3, 0) }).unwrap();
        board.add_bunny(Bunny { pos: (4, 2) }).unwrap();
        board.add_bunny(Bunny { pos: (3, 3) }).unwrap();
        board.add_mushroom(Mushroom { pos: (0, 1) }).unwrap();
        board.add_mushroom(Mushroom { pos: (2, 2) }).unwrap();
        board.add_mushroom(Mushroom { pos: (3, 4) }).unwrap();
        board
            .add_fox(
                Fox {
                    pos1: (0, 3),
                    pos2: (1, 3),
                }
                .normalize(),
            )
            .unwrap();

        let (graph, winning_board) = Graph::generate_solution_graph_from_board(&board);
        let winning_board = winning_board.unwrap();
        let path = graph.path_to(&winning_board);
        let moves = graph.moves_to(&winning_board);

        assert_eq!(moves.len(), 82);
        assert_eq!(path.len(), moves.len() + 1);
        assert!(path.last().unwrap().is_in_win_state());
        for states in path.windows(2) {
            assert!(states[0].can_move_to(&states[1]));
        }
    }
}
