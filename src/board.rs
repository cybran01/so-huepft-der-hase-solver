use std::{collections::HashSet, fmt::Display, hash::Hash};

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

#[derive(Debug, Clone, Eq, Hash)]
pub struct Board {
    pub bunnies: Vec<Bunny>,
    pub foxes: Vec<Fox>,
    pub mushrooms: Vec<Mushroom>,
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

impl Board {
    pub fn new() -> Self {
        Board {
            bunnies: Vec::new(),
            foxes: Vec::new(),
            mushrooms: Vec::new(),
        }
    }

    pub fn get_all_possible_moves(&self) -> Vec<Board> {
        let mut res = Vec::new();

        for bun in &self.bunnies {
            for bun_move in self.get_possible_bunny_moves(bun) {
                res.push(self.clone().move_bunny(&bun, bun_move).unwrap())
            }
        }
        for fox in &self.foxes {
            for fox_move in self.get_possible_fox_moves(fox) {
                res.push(self.clone().move_fox(&fox, fox_move).unwrap())
            }
        }

        res
    }

    pub fn get_possible_bunny_moves(&self, bunny: &Bunny) -> Vec<Bunny> {
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
            if !self.is_position_filled((pos.0 as usize, pos.1 as usize)) {
                continue;
            }

            pos = (pos.0 as isize + direction.0, pos.1 as isize + direction.1);
            loop {
                if pos.1 < 0 || pos.1 >= 5 || pos.0 < 0 || pos.0 >= 5 {
                    break;
                }
                if self.is_position_filled((pos.0 as usize, pos.1 as usize)) {
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
        let mut moves = Vec::new();

        let directions = match fox.orientation() {
            FoxOrientation::Horizontal => vec![(1, 0), (-1, 0)],
            FoxOrientation::Vertical => vec![(0, 1), (0, -1)],
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
                if self.is_position_filled((pos.0 as usize, pos.1 as usize)) {
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
        self.get_filled_positions().contains(&pos)
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
        self.bunnies
            .iter()
            .all(|bun| [(0, 0), (0, 4), (2, 2), (0, 4), (4, 4)].contains(&bun.pos))
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
