use crate::{board::*, graph::Graph};

mod board;
mod graph;

fn main() {
    let mut board = Board::new();

    // Constellation 60
    board.add_bunny(Bunny { pos: (3, 0) }).unwrap();
    board.add_bunny(Bunny { pos: (4, 2) }).unwrap();
    board.add_bunny(Bunny { pos: (3, 3) }).unwrap();

    board.add_mushroom(Mushroom { pos: (0, 1) }).unwrap();
    board.add_mushroom(Mushroom { pos: (2, 2) }).unwrap();
    board.add_mushroom(Mushroom { pos: (3, 4) }).unwrap();

    board
        .add_fox(Fox {
            pos1: (0, 3),
            pos2: (1, 3),
        })
        .unwrap();
    // End constelltion 60

    let (graph, winning_board) = Graph::generate_solution_graph_from_board(&board);
    match winning_board.as_ref() {
        Some(winning_board) => {
            let path = graph.path_to(winning_board).unwrap();
            let moves = graph.moves_to(winning_board).unwrap();
            debug_assert_eq!(moves.len() + 1, path.len());

            for (index, state) in path.iter().enumerate() {
                println!("{state}");
                if let Some(movement) = moves.get(index) {
                    println!("Move {index}: {movement}");
                }
            }
        }
        None => println!("No solution found for this board."),
    }
}
