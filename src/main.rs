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

    println!("{}", board);
}
