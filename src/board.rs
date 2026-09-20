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
    bunnies: Vec<Bunny>,
    foxes: Vec<Fox>,
    mushrooms: Vec<Mushroom>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StateKey {
    pub bunnies: u32,
    pub mushrooms: u32,
    pub foxes: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
enum Symmetry {
    Identity,
    Rotate90,
    Rotate180,
    Rotate270,
    ReflectX,
    ReflectY,
    ReflectDiagonal,
    ReflectAntiDiagonal,
}

impl Symmetry {
    const ALL: [Self; 8] = [
        Self::Identity,
        Self::Rotate90,
        Self::Rotate180,
        Self::Rotate270,
        Self::ReflectX,
        Self::ReflectY,
        Self::ReflectDiagonal,
        Self::ReflectAntiDiagonal,
    ];

    const fn index(self) -> usize {
        match self {
            Self::Identity => 0,
            Self::Rotate90 => 1,
            Self::Rotate180 => 2,
            Self::Rotate270 => 3,
            Self::ReflectX => 4,
            Self::ReflectY => 5,
            Self::ReflectDiagonal => 6,
            Self::ReflectAntiDiagonal => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    Bunny { from: u8, to: u8 },
    Fox { from: u8, to: u8 },
}

impl StateKey {
    pub fn canonical(&self) -> Self {
        let mut symmetries = Symmetry::ALL.into_iter();
        let mut canonical = self.transform(symmetries.next().unwrap());

        for symmetry in symmetries {
            let candidate = self.transform(symmetry);
            if candidate < canonical {
                canonical = candidate;
            }
        }

        canonical
    }

    fn transform(&self, symmetry: Symmetry) -> Self {
        let mut transformed = Self {
            bunnies: 0,
            mushrooms: 0,
            foxes: Vec::with_capacity(self.foxes.len()),
        };

        for position in 0..25 {
            let source = 1 << position;
            let target = 1 << SYMMETRY_MAPS[symmetry.index()][position];
            if self.bunnies & source != 0 {
                transformed.bunnies |= target;
            }
            if self.mushrooms & source != 0 {
                transformed.mushrooms |= target;
            }
        }

        for &placement in &self.foxes {
            let (first, second) = fox_placement_from_id(placement)
                .expect("state key must contain a valid fox placement");
            let mut pair = (
                SYMMETRY_MAPS[symmetry.index()][first as usize],
                SYMMETRY_MAPS[symmetry.index()][second as usize],
            );
            if position_sum(pair.0) > position_sum(pair.1) {
                std::mem::swap(&mut pair.0, &mut pair.1);
            }
            transformed.foxes.push(fox_placement_id(pair.0, pair.1));
        }
        transformed.foxes.sort();
        transformed
    }

    pub fn apply_move(&self, movement: &Move) -> Result<Self, InvalidBoardError> {
        let mut successor = self.clone();

        match movement {
            Move::Bunny { from, to } => {
                let from_mask = position_mask(position_from_index(*from));
                if successor.bunnies & from_mask == 0 {
                    return Err(InvalidBoardError::InvalidPosition);
                }
                successor.bunnies &= !from_mask;
                successor.bunnies |= position_mask(position_from_index(*to));
            }
            Move::Fox { from, to } => {
                let fox = successor
                    .foxes
                    .iter_mut()
                    .find(|fox| **fox == *from)
                    .ok_or(InvalidBoardError::InvalidPosition)?;
                *fox = *to;
                successor.foxes.sort();
            }
        }

        Ok(successor)
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state_key().hash(state);
    }
}

fn position_mask(pos: (usize, usize)) -> u32 {
    1 << (pos.1 * 5 + pos.0)
}

fn position_index(pos: (usize, usize)) -> u8 {
    (pos.1 * 5 + pos.0) as u8
}

fn position_from_index(index: u8) -> (usize, usize) {
    (index as usize % 5, index as usize / 5)
}

fn fox_key(fox: &Fox) -> u8 {
    let mut key = (position_index(fox.pos1), position_index(fox.pos2));
    if position_sum(key.0) > position_sum(key.1) {
        std::mem::swap(&mut key.0, &mut key.1);
    }
    fox_placement_id(key.0, key.1)
}

const FOX_PLACEMENT_COUNT: usize = 16;

const fn fox_placements() -> [(u8, u8); FOX_PLACEMENT_COUNT] {
    let mut placements = [(0, 0); FOX_PLACEMENT_COUNT];
    let mut id = 0;
    let mut left = 0;
    while left < 25 {
        let mut right = left + 1;
        while right < 25 {
            if is_legal_fox_edge(left, right) {
                let mut pair = (left, right);
                if position_sum(pair.0) > position_sum(pair.1) {
                    let first = pair.0;
                    pair.0 = pair.1;
                    pair.1 = first;
                }
                placements[id] = pair;
                id += 1;
            }
            right += 1;
        }
        left += 1;
    }
    placements
}

const FOX_PLACEMENTS: [(u8, u8); FOX_PLACEMENT_COUNT] = fox_placements();

const fn fox_placement_map() -> [u8; 625] {
    let mut map = [u8::MAX; 625];
    let mut id = 0;
    while id < FOX_PLACEMENT_COUNT {
        let pair = FOX_PLACEMENTS[id];
        map[pair.0 as usize * 25 + pair.1 as usize] = id as u8;
        map[pair.1 as usize * 25 + pair.0 as usize] = id as u8;
        id += 1;
    }
    map
}

const FOX_PLACEMENT_BY_PAIR: [u8; 625] = fox_placement_map();

fn fox_placement_id(first: u8, second: u8) -> u8 {
    let id = FOX_PLACEMENT_BY_PAIR[first as usize * 25 + second as usize];
    if id != u8::MAX {
        return id;
    }
    panic!("invalid fox placement: {first}, {second}");
}

fn fox_placement_from_id(target: u8) -> Option<(u8, u8)> {
    FOX_PLACEMENTS.get(target as usize).copied()
}

const fn is_legal_fox_edge(first: u8, second: u8) -> bool {
    let first_position = (first as usize % 5, first as usize / 5);
    let second_position = (second as usize % 5, second as usize / 5);
    let adjacent = first_position.0.abs_diff(second_position.0)
        + first_position.1.abs_diff(second_position.1)
        == 1;
    let first_on_lane = !(first_position.0 % 2 == 0 && first_position.1 % 2 == 0);
    let second_on_lane = !(second_position.0 % 2 == 0 && second_position.1 % 2 == 0);
    adjacent && first_on_lane && second_on_lane
}

const fn position_sum(position: u8) -> usize {
    position as usize % 5 + position as usize / 5
}

const fn transform_position(position: u8, symmetry: Symmetry) -> u8 {
    let x = position as usize % 5;
    let y = position as usize / 5;
    let (x, y) = match symmetry {
        Symmetry::Identity => (x, y),
        Symmetry::Rotate90 => (y, 4 - x),
        Symmetry::Rotate180 => (4 - x, 4 - y),
        Symmetry::Rotate270 => (4 - y, x),
        Symmetry::ReflectX => (4 - x, y),
        Symmetry::ReflectY => (x, 4 - y),
        Symmetry::ReflectDiagonal => (y, x),
        Symmetry::ReflectAntiDiagonal => (4 - y, 4 - x),
    };
    (y * 5 + x) as u8
}

const fn symmetry_maps() -> [[u8; 25]; 8] {
    let mut maps = [[0; 25]; 8];
    let mut symmetry = 0;
    while symmetry < 8 {
        let mut position = 0;
        while position < 25 {
            maps[symmetry][position] = transform_position(position as u8, Symmetry::ALL[symmetry]);
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
        self.state_key() == other.state_key()
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
                    from: position_index(bun.pos),
                    to: position_index(bun_move.pos),
                });
            }
        }
        for fox in &self.foxes {
            for fox_move in self.get_possible_fox_moves_with_occupancy(fox, occupied) {
                moves.push(Move::Fox {
                    from: fox_key(fox),
                    to: fox_key(&fox_move),
                });
            }
        }

        moves
    }

    pub fn apply_move(&self, movement: &Move) -> Result<Board, InvalidBoardError> {
        match movement {
            Move::Bunny { from, to } => self.clone().move_bunny(
                &Bunny {
                    pos: position_from_index(*from),
                },
                Bunny {
                    pos: position_from_index(*to),
                },
            ),
            Move::Fox { from, to } => {
                let (from1, from2) =
                    fox_placement_from_id(*from).ok_or(InvalidBoardError::InvalidPosition)?;
                let (to1, to2) =
                    fox_placement_from_id(*to).ok_or(InvalidBoardError::InvalidPosition)?;
                self.clone().move_fox(
                    &Fox {
                        pos1: position_from_index(from1),
                        pos2: position_from_index(from2),
                    },
                    Fox {
                        pos1: position_from_index(to1),
                        pos2: position_from_index(to2),
                    },
                )
            }
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
        if !self.bunnies.contains(from) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        self.bunnies.retain(|e| e != from);
        self.add_bunny(to)?;
        Ok(self)
    }

    pub fn move_fox(mut self, from: &Fox, to: Fox) -> Result<Self, InvalidBoardError> {
        let from = from.clone().normalize();
        if !self.foxes.contains(&from) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        self.foxes.retain(|e| e != &from);
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
        if self.bunnies.len() >= 3 {
            return Err(InvalidBoardError::TooManyBunnies);
        }
        self.is_bunny_placement_position_valid(bunny.pos)?;
        self.bunnies.push(bunny);
        Ok(())
    }

    pub fn add_fox(&mut self, fox: Fox) -> Result<(), InvalidBoardError> {
        if self.foxes.len() >= 2 {
            return Err(InvalidBoardError::TooManyFoxes);
        }
        let fox = fox.normalize();
        self.is_fox_placement_position_valid(fox.pos1, fox.pos2)?;
        self.foxes.push(fox);
        Ok(())
    }

    pub fn add_mushroom(&mut self, mushroom: Mushroom) -> Result<(), InvalidBoardError> {
        if self.mushrooms.len() >= 3 {
            return Err(InvalidBoardError::TooManyMushrooms);
        }
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
        let adjacent = pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1) == 1;
        if !adjacent {
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
    use crate::graph::Graph;

    use super::*;

    #[test]
    fn recorded_solution_contains_only_legal_transitions() {
        let mut root = Board::new();
        root.add_bunny(Bunny { pos: (3, 0) }).unwrap();
        root.add_bunny(Bunny { pos: (4, 2) }).unwrap();
        root.add_bunny(Bunny { pos: (3, 3) }).unwrap();
        root.add_mushroom(Mushroom { pos: (0, 1) }).unwrap();
        root.add_mushroom(Mushroom { pos: (2, 2) }).unwrap();
        root.add_mushroom(Mushroom { pos: (3, 4) }).unwrap();
        root.add_fox(
            Fox {
                pos1: (0, 3),
                pos2: (1, 3),
            }
            .normalize(),
        )
        .unwrap();

        let (graph, winning_board) = Graph::generate_solution_graph_from_board(&root);
        let states = graph.path_to(&winning_board.unwrap()).unwrap();

        assert_eq!(states.len(), 83);
        assert_eq!(states.first(), Some(&root));
        assert!(states.last().unwrap().is_in_win_state());

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
    fn add_fox_normalizes_reversed_endpoints() {
        let mut board = Board::new();
        board
            .add_fox(Fox {
                pos1: (1, 3),
                pos2: (0, 3),
            })
            .unwrap();

        assert_eq!(
            board.foxes,
            vec![Fox {
                pos1: (0, 3),
                pos2: (1, 3),
            }]
        );
    }

    #[test]
    fn invalid_public_mutations_fail_without_changing_the_board() {
        let mut board = Board::new();
        board
            .add_fox(Fox {
                pos1: (1, 3),
                pos2: (0, 3),
            })
            .unwrap();

        let before = board.clone();
        assert!(
            board
                .add_fox(Fox {
                    pos1: (1, 1),
                    pos2: (2, 2),
                })
                .is_err()
        );
        assert_eq!(board, before);

        assert!(
            board
                .clone()
                .move_fox(
                    &Fox {
                        pos1: (3, 3),
                        pos2: (4, 3),
                    },
                    Fox {
                        pos1: (1, 3),
                        pos2: (2, 3),
                    },
                )
                .is_err()
        );
        assert_eq!(board, before);
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
        for symmetry in Symmetry::ALL {
            assert_eq!(canonical, board.state_key().transform(symmetry).canonical());
        }
    }
}
