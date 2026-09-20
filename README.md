# Solver for Single Player Board Game `So hüpft der Hase`

## Usage
Specify the position of the bunnies, foxes and mushrooms in `main.rs` by calling the `add_bunny`, `add_fox` and `add_mushroom` APIs.
As an example, the constellation 60 from the game manual is included as a test.

Run 
```bash
cargo run
```
to get the output of the tool.
It will produce one of the shortest possible solutions, or an error if the board is not solveable.
