use crate::board::*;

mod board;

fn main() {
    let mut board = Board::new();

    board.add_bunny(Bunny { pos: (2, 3) }).unwrap();

    board.add_mushroom(Mushroom { pos: (1, 3) }).unwrap();
    board.add_mushroom(Mushroom { pos: (2, 2) }).unwrap();
    board.add_mushroom(Mushroom { pos: (3, 4) }).unwrap();

    board
        .add_fox(
            Fox {
                pos1: (4, 3),
                pos2: (3, 3),
            }
            .normalize(),
        )
        .unwrap();

    board
        .add_fox(
            Fox {
                pos1: (4, 1),
                pos2: (3, 1),
            }
            .normalize(),
        )
        .unwrap();

    println!("{}", board);

    // Get possible moves for the bunny at position (2, 3)
    let bunny_moves = board.get_possible_bunny_moves(&Bunny { pos: (2, 3) });
    for bunny_move in bunny_moves {
        let test_board = board.clone();
        println!(
            "Bunny can move to:\n{}",
            test_board
                .move_bunny(&Bunny { pos: (2, 3) }, bunny_move)
                .unwrap()
        );
    }

    let board = board
        .move_bunny(&Bunny { pos: (2, 3) }, Bunny { pos: (2, 1) })
        .unwrap();

    println!("New Board:\n{}", board);

    // Get possible moves for the fox at position (4, 3) and (3, 3)
    let fox_moves = board.get_possible_fox_moves(
        &Fox {
            pos1: (4, 3),
            pos2: (3, 3),
        }
        .normalize(),
    );
    for fox_move in fox_moves {
        let test_board = board.clone();
        println!(
            "Fox1 can move to:\n{}",
            test_board
                .move_fox(
                    &Fox {
                        pos1: (4, 3),
                        pos2: (3, 3)
                    }
                    .normalize(),
                    fox_move
                )
                .unwrap()
        );
    }

    // Get possible moves for the fox at position (4, 1) and (3, 1)
        let fox_moves = board.get_possible_fox_moves(
        &Fox {
            pos1: (4, 1),
            pos2: (3, 1),
        }
        .normalize(),
    );
    for fox_move in fox_moves {
        let test_board = board.clone();
        println!(
            "Fox2 can move to:\n{}",
            test_board
                .move_fox(
                    &Fox {
                        pos1: (4, 1),
                        pos2: (3, 1)
                    }
                    .normalize(),
                    fox_move
                )
                .unwrap()
        );
    }
}
