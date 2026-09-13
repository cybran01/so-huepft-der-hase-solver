use std::{collections::HashMap, hash::Hash};

use crate::{
    board::*,
    graph::GraphNode::{Duplicate, FlipX, FlipY, Rotate90, Rotate180, Rotate270, Unique},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphNode {
    Unique(Board),
    Duplicate(Board),
    FlipX(Board),
    FlipY(Board),
    Rotate90(Board),
    Rotate180(Board),
    Rotate270(Board),
}

impl GraphNode {
    pub fn get_board(&self) -> &Board {
        match self {
            Unique(board) => board,
            Duplicate(board) => board,
            FlipX(board) => board,
            FlipY(board) => board,
            Rotate90(board) => board,
            Rotate180(board) => board,
            Rotate270(board) => board,
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub root: Board,
    // key: child, value: parent
    pub edgemap: HashMap<GraphNode, Board>,
}

impl Graph {
    pub fn generate_solution_graph_from_board(board: &Board) -> (Self, Option<Board>) {
        let mut edgemap = HashMap::new();

        if board.is_in_win_state() {
            return (
                Graph {
                    root: board.clone(),
                    edgemap,
                },
                Some(board.clone()),
            );
        }

        let mut cur_layer_moves = vec![Unique(board.clone())];
        let mut winning_board = None;
        while winning_board.is_none()
            && cur_layer_moves.iter().any(|n| {
                if let Unique(_) = n {
                    return true;
                }
                return false;
            })
        {
            let mut next_layer_moves = HashMap::new();
            for board_move in cur_layer_moves.iter().filter_map(|n| {
                if let Unique(n) = n {
                    return Some(n);
                }
                return None;
            }) {
                let possible_moves = board_move.get_all_possible_moves();

                let item_next_layer_moves = possible_moves
                    .iter()
                    .map(|b| Self::check_duplicates(&edgemap, board, b));
                for item in item_next_layer_moves {
                    next_layer_moves.insert(item, board_move.clone());
                    if board_move.is_in_win_state() {
                        winning_board = Some(board_move.clone());
                    }
                }
            }
            edgemap.extend(next_layer_moves.clone());
            cur_layer_moves = next_layer_moves.keys().cloned().collect();
        }

        return (
            Graph {
                root: board.clone(),
                edgemap,
            },
            winning_board,
        );
    }

    fn check_duplicates(
        edgemap: &HashMap<GraphNode, Board>,
        root: &Board,
        board: &Board,
    ) -> GraphNode {
        if root == board {
            return Duplicate(board.clone());
        }
        for cur_node in edgemap.keys() {
            if let Unique(cur_board) = cur_node {
                if board.flip_x() == *cur_board {
                    return FlipX(board.clone());
                }
                if board.rotate90() == *cur_board {
                    return Rotate90(board.clone());
                }
                if board.rotate180() == *cur_board {
                    return Rotate180(board.clone());
                }
                if board.rotate270() == *cur_board {
                    return Rotate270(board.clone());
                }
                if board.flip_y() == *cur_board {
                    return FlipY(board.clone());
                }
                if board == cur_board {
                    return Duplicate(board.clone());
                }
            }
        }
        GraphNode::Unique(board.clone())
    }
}
