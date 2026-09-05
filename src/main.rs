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

    // Move a bunny from (2, 3) to (2, 1)
    let mv = Move::<Bunny>::new(
        &board,
        board.bunnies.iter().find(|b| b.pos == (2, 3)).unwrap(),
        Bunny { pos: (2, 1) },
    );
    let new_board = board.move_bunny(mv).unwrap();

    println!("{}", new_board);

    // Move fox 1 from (3, 3) and (4, 3) to (2, 3) and (3, 3)
    let mv = Move::<Fox>::new(
        &new_board,
        new_board
            .foxes
            .iter()
            .find(|f| f.pos1 == (3, 3) && f.pos2 == (4, 3))
            .unwrap(),
        Fox {
            pos1: (3, 3),
            pos2: (2, 3),
        }
        .normalize(),
    );
    let new_board = new_board.move_fox(mv).unwrap();

    println!("{}", new_board);

    // Move bunny from (2, 1) to (2, 4)
    let mv = Move::<Bunny>::new(
        &new_board,
        new_board.bunnies.iter().find(|b| b.pos == (2, 1)).unwrap(),
        Bunny { pos: (2, 4) },
    );
    let new_board = new_board.move_bunny(mv).unwrap();

    println!("{}", new_board);

    // Move fox 2 from (3, 1) and (4, 1) to (0, 1) and (1, 1)
    let mv = Move::<Fox>::new(
        &new_board,
        new_board
            .foxes
            .iter()
            .find(|f| f.pos1 == (3, 1) && f.pos2 == (4, 1))
            .unwrap(),
        Fox {
            pos1: (0, 1),
            pos2: (1, 1),
        }
        .normalize(),
    );
    let new_board = new_board.move_fox(mv).unwrap();

    println!("{}", new_board);
}
