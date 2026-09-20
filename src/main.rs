use crate::{board::*, graph::Graph};

mod board;
mod graph;

fn main() {
    let mut board = Board::new();

    // Constellation 1
    // board.add_mushroom(Mushroom { pos: (1, 4) }).unwrap();
    // board.add_mushroom(Mushroom { pos: (2, 4) }).unwrap();
    // board.add_mushroom(Mushroom { pos: (3, 3) }).unwrap();

    // board.add_bunny(Bunny { pos: (3, 2) }).unwrap();

    // Constellation 26
    // board.add_mushroom(Mushroom { pos: (1, 4) }).unwrap();
    // board.add_mushroom(Mushroom { pos: (1, 1) }).unwrap();

    // board.add_bunny(Bunny { pos: (2, 2) }).unwrap();
    // board.add_bunny(Bunny { pos: (3, 3) }).unwrap();

    // board.add_fox(Fox {
    //     pos1: (0, 3),
    //     pos2: (1, 3),
    // }).unwrap();

    // Constellation 60 warning: takes long!
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

    let (graph, winning_board) = Graph::generate_solution_graph_from_board(&board);
    let winning_board = winning_board.as_ref().unwrap();
    let path = graph.path_to(winning_board).unwrap();
    debug_assert_eq!(graph.moves_to(winning_board).unwrap().len() + 1, path.len());

    for state in path {
        println!("{state}");
    }
}
