use std::{
    collections::HashSet,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Eq, Debug, Hash, PartialEq, Clone, PartialOrd, Ord)]
pub struct Bunny {
    pub pos: (usize, usize),
}

#[derive(Eq, Debug, Hash, PartialEq, Clone, PartialOrd, Ord)]
pub struct Fox {
    pub pos1: (usize, usize),
    pub pos2: (usize, usize),
}

#[derive(PartialEq)]
enum FoxOrientation {
    Horizontal,
    Vertical,
}

impl Fox {
    pub fn normalize(mut self) -> Self {
        if self.pos1.0 + self.pos1.1 > self.pos2.0 + self.pos2.1 {
            std::mem::swap(&mut self.pos1, &mut self.pos2);
        }
        self
    }
    fn orientation(&self) -> FoxOrientation {
        if self.pos1.0 == self.pos2.0 {
            FoxOrientation::Vertical
        } else {
            FoxOrientation::Horizontal
        }
    }
}

#[derive(Eq, Debug, Hash, PartialEq, Clone, PartialOrd, Ord)]
pub struct Mushroom {
    pub pos: (usize, usize),
}

#[derive(Debug, Clone, Eq)]
pub struct Board {
    pub bunnies: Vec<Bunny>,
    pub foxes: Vec<Fox>,
    pub mushrooms: Vec<Mushroom>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StateKey {
    pub bunnies: u32,
    pub mushrooms: u32,
    pub foxes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    Bunny { from: Bunny, to: Bunny },
    Fox { from: Fox, to: Fox },
}

impl StateKey {
    pub fn canonical(&self) -> Self {
        (0..8)
            .map(|symmetry| self.transform(symmetry))
            .min()
            .unwrap()
    }

    fn transform(&self, symmetry: usize) -> Self {
        let mut transformed = Self {
            bunnies: 0,
            mushrooms: 0,
            foxes: Vec::with_capacity(self.foxes.len()),
        };

        for position in 0..25 {
            let source = 1 << position;
            let target = 1 << SYMMETRY_MAPS[symmetry][position];
            if self.bunnies & source != 0 {
                transformed.bunnies |= target;
            }
            if self.mushrooms & source != 0 {
                transformed.mushrooms |= target;
            }
        }

        for &placement in &self.foxes {
            let (first, second) = fox_placement_from_id(placement);
            let mut pair = (
                SYMMETRY_MAPS[symmetry][first as usize],
                SYMMETRY_MAPS[symmetry][second as usize],
            );
            if position_sum(pair.0) > position_sum(pair.1) {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
            transformed.foxes.push(fox_placement_id(pair.0, pair.1));
        }
        transformed.foxes.sort();
        transformed
    }

    pub fn apply_move(&self, movement: &Move) -> Self {
        let mut successor = self.clone();

        match movement {
            Move::Bunny { from, to } => {
                successor.bunnies &= !position_mask(from.pos);
                successor.bunnies |= position_mask(to.pos);
            }
            Move::Fox { from, to } => {
                let from_id = fox_key(from);
                let to_id = fox_key(to);
                let fox = successor
                    .foxes
                    .iter_mut()
                    .find(|fox| **fox == from_id)
                    .expect("move source must exist in state key");
                *fox = to_id;
                successor.foxes.sort();
            }
        }

        successor
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut bunnies = self.bunnies.clone();
        let mut foxes = self.foxes.clone();
        let mut mushrooms = self.mushrooms.clone();

        bunnies.sort();
        foxes.sort();
        mushrooms.sort();

        bunnies.hash(state);
        foxes.hash(state);
        mushrooms.hash(state);
    }
}

fn position_mask(pos: (usize, usize)) -> u32 {
    1 << (pos.1 * 5 + pos.0)
}

fn position_index(pos: (usize, usize)) -> u8 {
    (pos.1 * 5 + pos.0) as u8
}

fn fox_key(fox: &Fox) -> u8 {
    let mut key = (position_index(fox.pos1), position_index(fox.pos2));
    if position_sum(key.0) > position_sum(key.1) {
        std::mem::swap(&mut key.0, &mut key.1);
    }
    fox_placement_id(key.0, key.1)
}

fn fox_placement_id(first: u8, second: u8) -> u8 {
    let mut id = 0;
    for left in 0..25u8 {
        for right in (left + 1)..25u8 {
            if !is_legal_fox_edge(left, right) {
                continue;
            }

            let mut pair = (left, right);
            if position_sum(pair.0) > position_sum(pair.1) {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
            if pair == (first, second) {
                return id;
            }
            id += 1;
        }
    }
    panic!("invalid fox placement: {first}, {second}");
}

fn fox_placement_from_id(target: u8) -> (u8, u8) {
    let mut id = 0;
    for left in 0..25u8 {
        for right in (left + 1)..25u8 {
            if !is_legal_fox_edge(left, right) {
                continue;
            }

            let mut pair = (left, right);
            if position_sum(pair.0) > position_sum(pair.1) {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
            if id == target {
                return pair;
            }
            id += 1;
        }
    }
    panic!("invalid fox placement id: {target}");
}

fn is_legal_fox_edge(first: u8, second: u8) -> bool {
    let first_position = (first as usize % 5, first as usize / 5);
    let second_position = (second as usize % 5, second as usize / 5);
    let adjacent = first_position.0.abs_diff(second_position.0)
        + first_position.1.abs_diff(second_position.1)
        == 1;
    let first_on_lane = !(first_position.0 % 2 == 0 && first_position.1 % 2 == 0);
    let second_on_lane = !(second_position.0 % 2 == 0 && second_position.1 % 2 == 0);
    adjacent && first_on_lane && second_on_lane
}

fn position_sum(position: u8) -> usize {
    position as usize % 5 + position as usize / 5
}

const fn transform_position(position: u8, symmetry: usize) -> u8 {
    let x = position as usize % 5;
    let y = position as usize / 5;
    let (x, y) = match symmetry {
        0 => (x, y),
        1 => (y, 4 - x),
        2 => (4 - x, 4 - y),
        3 => (4 - y, x),
        4 => (4 - x, y),
        5 => (x, 4 - y),
        6 => (y, x),
        7 => (4 - y, 4 - x),
        _ => unreachable!(),
    };
    (y * 5 + x) as u8
}

const fn symmetry_maps() -> [[u8; 25]; 8] {
    let mut maps = [[0; 25]; 8];
    let mut symmetry = 0;
    while symmetry < 8 {
        let mut position = 0;
        while position < 25 {
            maps[symmetry][position] = transform_position(position as u8, symmetry);
            position += 1;
        }
        symmetry += 1;
    }
    maps
}

const SYMMETRY_MAPS: [[u8; 25]; 8] = symmetry_maps();

fn is_occupied(occupied: u32, pos: (usize, usize)) -> bool {
    occupied & position_mask(pos) != 0
}

impl PartialEq for Board {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        let mut self_bun = self.bunnies.clone();
        self_bun.sort();
        let mut self_fox = self.foxes.clone();
        self_fox.sort();
        let mut self_mushroom = self.mushrooms.clone();
        self_mushroom.sort();

        let mut other_bun = other.bunnies.clone();
        other_bun.sort();
        let mut other_fox = other.foxes.clone();
        other_fox.sort();
        let mut other_mushroom = other.mushrooms.clone();
        other_mushroom.sort();

        self_bun == other_bun && self_fox == other_fox && self_mushroom == other_mushroom
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum InvalidBoardError {
    InvalidPosition,
    OutOfBounds,
    TooManyBunnies,
    TooManyFoxes,
    TooManyMushrooms,
    FoxNotNormalized,
    FoxNotConnected,
    FoxNotOnOddLane,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = vec![vec!['.'; 5]; 5];

        for bunny in &self.bunnies {
            board[bunny.pos.1][bunny.pos.0] = 'B';
        }

        for fox in &self.foxes {
            board[fox.pos1.1][fox.pos1.0] = 'F';
            board[fox.pos2.1][fox.pos2.0] = 'F';
        }

        for mushroom in &self.mushrooms {
            board[mushroom.pos.1][mushroom.pos.0] = 'M';
        }

        for row in board.iter().rev() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl Board {
    pub fn new() -> Self {
        Board {
            bunnies: Vec::new(),
            foxes: Vec::new(),
            mushrooms: Vec::new(),
        }
    }

    pub fn state_key(&self) -> StateKey {
        let mut foxes = self.foxes.iter().map(fox_key).collect::<Vec<_>>();
        foxes.sort();

        StateKey {
            bunnies: self
                .bunnies
                .iter()
                .fold(0, |occupied, bunny| occupied | position_mask(bunny.pos)),
            mushrooms: self.mushrooms.iter().fold(0, |occupied, mushroom| {
                occupied | position_mask(mushroom.pos)
            }),
            foxes,
        }
    }

    pub fn get_all_possible_moves(&self) -> Vec<Board> {
        self.get_all_moves()
            .iter()
            .map(|movement| self.apply_move(movement).unwrap())
            .collect()
    }

    pub fn get_all_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let occupied = self.occupied_mask();

        for bun in &self.bunnies {
            for bun_move in self.get_possible_bunny_moves_with_occupancy(bun, occupied) {
                moves.push(Move::Bunny {
                    from: bun.clone(),
                    to: bun_move,
                });
            }
        }
        for fox in &self.foxes {
            for fox_move in self.get_possible_fox_moves_with_occupancy(fox, occupied) {
                moves.push(Move::Fox {
                    from: fox.clone(),
                    to: fox_move,
                });
            }
        }

        moves
    }

    pub fn apply_move(&self, movement: &Move) -> Result<Board, InvalidBoardError> {
        match movement {
            Move::Bunny { from, to } => self.clone().move_bunny(from, to.clone()),
            Move::Fox { from, to } => self.clone().move_fox(from, to.clone()),
        }
    }

    pub fn can_move_to(&self, target: &Board) -> bool {
        self.get_all_possible_moves()
            .iter()
            .any(|possible| possible == target)
    }

    pub fn get_possible_bunny_moves(&self, bunny: &Bunny) -> Vec<Bunny> {
        self.get_possible_bunny_moves_with_occupancy(bunny, self.occupied_mask())
    }

    fn get_possible_bunny_moves_with_occupancy(&self, bunny: &Bunny, occupied: u32) -> Vec<Bunny> {
        let mut moves = Vec::new();

        for direction in [
            (1 as isize, 0),
            (0, 1 as isize),
            (-1 as isize, 0),
            (0, -1 as isize),
        ] {
            let mut pos = (
                bunny.pos.0 as isize + direction.0,
                bunny.pos.1 as isize + direction.1,
            );
            if pos.1 < 0 || pos.1 >= 5 || pos.0 < 0 || pos.0 >= 5 {
                continue;
            }
            if !is_occupied(occupied, (pos.0 as usize, pos.1 as usize)) {
                continue;
            }

            pos = (pos.0 as isize + direction.0, pos.1 as isize + direction.1);
            loop {
                if pos.1 < 0 || pos.1 >= 5 || pos.0 < 0 || pos.0 >= 5 {
                    break;
                }
                if is_occupied(occupied, (pos.0 as usize, pos.1 as usize)) {
                    pos = (pos.0 as isize + direction.0, pos.1 as isize + direction.1);
                } else {
                    moves.push(Bunny {
                        pos: (pos.0 as usize, pos.1 as usize),
                    });
                    break;
                }
            }
        }
        moves
    }

    // Assumes normalized fox!
    pub fn get_possible_fox_moves(&self, fox: &Fox) -> Vec<Fox> {
        self.get_possible_fox_moves_with_occupancy(fox, self.occupied_mask())
    }

    fn get_possible_fox_moves_with_occupancy(&self, fox: &Fox, occupied: u32) -> Vec<Fox> {
        let mut moves = Vec::new();

        let directions = match fox.orientation() {
            FoxOrientation::Horizontal => [(1, 0), (-1, 0)],
            FoxOrientation::Vertical => [(0, 1), (0, -1)],
        };

        for direction in directions.iter() {
            let mut pos = if direction.0 + direction.1 == 1 {
                (fox.pos2.0 as isize, fox.pos2.1 as isize)
            } else {
                (fox.pos1.0 as isize, fox.pos1.1 as isize)
            };

            pos = (pos.0 + direction.0, pos.1 + direction.1);
            loop {
                if pos.1 < 0 || pos.1 >= 5 || pos.0 < 0 || pos.0 >= 5 {
                    break;
                }
                if is_occupied(occupied, (pos.0 as usize, pos.1 as usize)) {
                    break;
                } else {
                    moves.push(
                        Fox {
                            pos1: (pos.0 as usize, pos.1 as usize),
                            pos2: (
                                (pos.0 - direction.0) as usize,
                                (pos.1 - direction.1) as usize,
                            ),
                        }
                        .normalize(),
                    );
                }
                pos = (pos.0 + direction.0, pos.1 + direction.1);
            }
        }
        moves
    }

    pub fn move_bunny(mut self, from: &Bunny, to: Bunny) -> Result<Self, InvalidBoardError> {
        self.bunnies.retain(|e| e != from);
        self.add_bunny(to)?;
        Ok(self)
    }

    pub fn move_fox(mut self, from: &Fox, to: Fox) -> Result<Self, InvalidBoardError> {
        self.foxes.retain(|e| e != from);
        self.add_fox(to)?;
        Ok(self)
    }

    pub fn is_empty(&self) -> bool {
        self.bunnies.is_empty() && self.foxes.is_empty() && self.mushrooms.is_empty()
    }

    pub fn get_filled_positions(&self) -> HashSet<(usize, usize)> {
        let mut filled_positions = HashSet::new();

        for bunny in &self.bunnies {
            filled_positions.insert(bunny.pos);
        }

        for fox in &self.foxes {
            filled_positions.insert(fox.pos1);
            filled_positions.insert(fox.pos2);
        }

        for mushroom in &self.mushrooms {
            filled_positions.insert(mushroom.pos);
        }

        filled_positions
    }

    fn occupied_mask(&self) -> u32 {
        let mut occupied = 0;

        for bunny in &self.bunnies {
            occupied |= position_mask(bunny.pos);
        }
        for fox in &self.foxes {
            occupied |= position_mask(fox.pos1) | position_mask(fox.pos2);
        }
        for mushroom in &self.mushrooms {
            occupied |= position_mask(mushroom.pos);
        }

        occupied
    }

    pub fn add_bunny(&mut self, bunny: Bunny) -> Result<(), InvalidBoardError> {
        self.is_bunny_placement_position_valid(bunny.pos)?;
        self.bunnies.push(bunny);
        Ok(())
    }

    pub fn add_fox(&mut self, fox: Fox) -> Result<(), InvalidBoardError> {
        self.is_fox_placement_position_valid(fox.pos1, fox.pos2)?;
        self.foxes.push(fox);
        Ok(())
    }

    pub fn add_mushroom(&mut self, mushroom: Mushroom) -> Result<(), InvalidBoardError> {
        self.is_mushroom_placement_position_valid(mushroom.pos)?;
        self.mushrooms.push(mushroom);
        Ok(())
    }

    pub fn is_position_filled(&self, pos: (usize, usize)) -> bool {
        is_occupied(self.occupied_mask(), pos)
    }

    pub fn is_bunny_placement_position_valid(
        &self,
        pos: (usize, usize),
    ) -> Result<(), InvalidBoardError> {
        if pos.0 >= 5 || pos.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.is_position_filled(pos) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        Ok(())
    }

    pub fn is_fox_placement_position_valid(
        &self,
        pos1: (usize, usize),
        pos2: (usize, usize),
    ) -> Result<(), InvalidBoardError> {
        if pos1.0 >= 5 || pos1.1 >= 5 || pos2.0 >= 5 || pos2.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.is_position_filled(pos1) || self.is_position_filled(pos2) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        if pos1.0.abs_diff(pos2.0) != 1 && pos1.1.abs_diff(pos2.1) == 0
            || pos1.1.abs_diff(pos2.1) != 1 && pos1.0.abs_diff(pos2.0) == 0
        {
            return Err(InvalidBoardError::FoxNotConnected);
        }
        if pos1.0 % 2 == 0 && pos1.1 % 2 == 0 || pos2.0 % 2 == 0 && pos2.1 % 2 == 0 {
            return Err(InvalidBoardError::FoxNotOnOddLane);
        }
        if pos1.0 + pos1.1 > pos2.0 + pos2.1 {
            return Err(InvalidBoardError::FoxNotNormalized);
        }
        Ok(())
    }

    pub fn is_mushroom_placement_position_valid(
        &self,
        pos: (usize, usize),
    ) -> Result<(), InvalidBoardError> {
        if pos.0 >= 5 || pos.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.is_position_filled(pos) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        Ok(())
    }

    fn is_board_valid(&self) -> Result<(), InvalidBoardError> {
        if self.bunnies.len() > 3 {
            return Err(InvalidBoardError::TooManyBunnies);
        }
        if self.foxes.len() > 2 {
            return Err(InvalidBoardError::TooManyFoxes);
        }
        if self.mushrooms.len() > 3 {
            return Err(InvalidBoardError::TooManyMushrooms);
        }
        Ok(())
    }

    pub fn is_in_win_state(&self) -> bool {
        self.bunnies.len() == 3
            && self
                .bunnies
                .iter()
                .all(|bun| [(0, 0), (0, 4), (2, 2), (4, 0), (4, 4)].contains(&bun.pos))
    }

    fn transform_board(&self, trafo: impl Fn((usize, usize)) -> (usize, usize)) -> Board {
        let bunnies = self
            .bunnies
            .iter()
            .map(|bun| Bunny {
                pos: trafo(bun.pos),
            })
            .collect();
        let mushrooms = self
            .mushrooms
            .iter()
            .map(|mushroom| Mushroom {
                pos: trafo(mushroom.pos),
            })
            .collect();
        let foxes = self
            .foxes
            .iter()
            .map(|fox| {
                Fox {
                    pos1: trafo(fox.pos1),
                    pos2: trafo(fox.pos2),
                }
                .normalize()
            })
            .collect();
        Board {
            bunnies,
            foxes,
            mushrooms,
        }
    }

    pub fn rotate90(&self) -> Self {
        let trafo = |pos: (usize, usize)| (pos.1, 4 - pos.0 as usize);
        self.transform_board(trafo)
    }

    pub fn rotate180(&self) -> Self {
        self.rotate90().rotate90()
    }

    pub fn rotate270(&self) -> Self {
        self.rotate180().rotate90()
    }

    pub fn flip_x(&self) -> Self {
        let trafo = |pos: (usize, usize)| (4 - pos.0 as usize, pos.1);
        self.transform_board(trafo)
    }

    pub fn flip_y(&self) -> Self {
        self.flip_x().rotate180()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_board(lines: &[&str]) -> Board {
        let mut board = Board::new();
        let mut fox_cells = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            for (column, cell) in line.chars().enumerate() {
                let position = (column, 4 - row);
                match cell {
                    'B' => board.add_bunny(Bunny { pos: position }).unwrap(),
                    'F' => fox_cells.push(position),
                    'M' => board.add_mushroom(Mushroom { pos: position }).unwrap(),
                    '.' => {}
                    _ => panic!("invalid board cell: {cell}"),
                }
            }
        }

        for cells in fox_cells.chunks_exact(2) {
            board
                .add_fox(
                    Fox {
                        pos1: cells[0],
                        pos2: cells[1],
                    }
                    .normalize(),
                )
                .unwrap();
        }

        board
    }

    #[test]
    fn recorded_solution_contains_only_legal_transitions() {
        let states: Vec<Board> = include_str!("../solution_60.txt")
            .split("\n\n")
            .map(|state| parse_board(&state.lines().collect::<Vec<_>>()))
            .collect();

        assert_eq!(states.len(), 83);
        assert!(states.first().unwrap().is_in_win_state());
        let mut initial_bunnies = states.last().unwrap().bunnies.clone();
        initial_bunnies.sort();
        assert_eq!(
            initial_bunnies,
            vec![
                Bunny { pos: (3, 0) },
                Bunny { pos: (3, 3) },
                Bunny { pos: (4, 2) },
            ]
        );

        for (index, states) in states.windows(2).enumerate() {
            assert!(
                states[1].can_move_to(&states[0]),
                "illegal transition between states {} and {}",
                index,
                index + 1
            );
        }
    }

    #[test]
    fn reordered_entities_have_the_same_hash() {
        use std::collections::hash_map::DefaultHasher;

        let mut first = Board::new();
        first.add_bunny(Bunny { pos: (1, 1) }).unwrap();
        first.add_bunny(Bunny { pos: (3, 3) }).unwrap();
        first.add_mushroom(Mushroom { pos: (0, 1) }).unwrap();
        first.add_mushroom(Mushroom { pos: (2, 2) }).unwrap();

        let mut second = Board::new();
        second.add_bunny(Bunny { pos: (3, 3) }).unwrap();
        second.add_bunny(Bunny { pos: (1, 1) }).unwrap();
        second.add_mushroom(Mushroom { pos: (2, 2) }).unwrap();
        second.add_mushroom(Mushroom { pos: (0, 1) }).unwrap();

        let mut first_hasher = DefaultHasher::new();
        first.hash(&mut first_hasher);
        let mut second_hasher = DefaultHasher::new();
        second.hash(&mut second_hasher);

        assert_eq!(first, second);
        assert_eq!(first_hasher.finish(), second_hasher.finish());
        assert_eq!(first.state_key(), second.state_key());
    }

    #[test]
    fn incomplete_board_is_not_a_win() {
        let mut board = Board::new();
        board.add_bunny(Bunny { pos: (0, 0) }).unwrap();

        assert!(!board.is_in_win_state());
    }

    #[test]
    fn symmetries_share_a_canonical_state_key() {
        let mut board = Board::new();
        board.add_bunny(Bunny { pos: (1, 1) }).unwrap();
        board.add_bunny(Bunny { pos: (3, 3) }).unwrap();
        board.add_bunny(Bunny { pos: (2, 0) }).unwrap();
        board.add_mushroom(Mushroom { pos: (0, 1) }).unwrap();
        board
            .add_fox(
                Fox {
                    pos1: (0, 3),
                    pos2: (1, 3),
                }
                .normalize(),
            )
            .unwrap();

        let canonical = board.state_key().canonical();
        assert_eq!(canonical, board.rotate90().state_key().canonical());
        assert_eq!(canonical, board.flip_x().state_key().canonical());
    }

    #[test]
    fn booklet_solution_contains_only_legal_transitions() {
        let states: Vec<Board> = include_str!("../booklet_solution_60.txt")
            .split("\n\n")
            .map(|state| parse_board(&state.lines().collect::<Vec<_>>()))
            .collect();

        assert_eq!(states.len(), 88);
        assert!(!states.first().unwrap().is_in_win_state());
        assert!(states.last().unwrap().is_in_win_state());

        for (index, states) in states.windows(2).enumerate() {
            assert!(
                states[0].can_move_to(&states[1]),
                "illegal booklet transition between states {} and {}",
                index,
                index + 1
            );
        }
    }
}
