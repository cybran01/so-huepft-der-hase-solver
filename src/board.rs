use std::{collections::HashSet, fmt::Display, hash::Hash};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Bunny {
    pub pos: (usize, usize),
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Fox {
    pub pos1: (usize, usize),
    pub pos2: (usize, usize),
}

impl Fox {
    pub fn normalize(mut self) -> Self {
        if self.pos1.0 + self.pos1.1 > self.pos2.0 + self.pos2.1 {
            std::mem::swap(&mut self.pos1, &mut self.pos2);
        }
        self
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Mushroom {
    pub pos: (usize, usize),
}

#[derive(Clone)]
pub struct Board {
    pub bunnies: HashSet<Bunny>,
    pub foxes: HashSet<Fox>,
    pub mushrooms: HashSet<Mushroom>,
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

pub struct Move<'a, T> {
    pub board: &'a Board,
    pub from: &'a T,
    pub to: T,
}

impl<'a> Move<'a, Bunny> {
    pub fn new(board: &'a Board, from: &'a Bunny, to: Bunny) -> Self {
        Move { board, from, to }
    }

    pub fn is_valid(&self) -> Result<(), InvalidBoardError> {
        self.board.is_bunny_placement_position_valid(self.to.pos)?;
        if self.from.pos.0 != self.to.pos.0 && self.from.pos.1 != self.to.pos.1 {
            // at least one coordinate must be the same, only axis movement is allowed
            return Err(InvalidBoardError::InvalidPosition); //TODO better error
        }

        let distance =
            self.from.pos.0.abs_diff(self.to.pos.0) + self.from.pos.1.abs_diff(self.to.pos.1);
        if distance <= 1 {
            // must move at least 2 spaces
            return Err(InvalidBoardError::InvalidPosition); //TODO better error
        }

        // all fields between the from and to positions must be filled
        if self.from.pos.0 == self.to.pos.0 {
            let pos_diff = self.to.pos.1.checked_signed_diff(self.from.pos.1).unwrap();

            let mut range = if pos_diff.signum() > 0 {
                1..pos_diff
            } else {
                (pos_diff + 1)..0
            };

            if !range.all(|pos| {
                self.board.is_position_filled((
                    self.from.pos.0,
                    (self.from.pos.1 as isize + pos) as usize,
                ))
            }) {
                return Err(InvalidBoardError::InvalidPosition); //TODO better error
            }
        } else if self.from.pos.1 == self.to.pos.1 {
            let pos_diff = self.to.pos.0.checked_signed_diff(self.from.pos.0).unwrap();

            let mut range = if pos_diff.signum() > 0 {
                1..pos_diff
            } else {
                (pos_diff + 1)..0
            };

            if !range.all(|pos| {
                self.board.is_position_filled((
                    (self.from.pos.0 as isize + pos) as usize,
                    self.from.pos.1,
                ))
            }) {
                return Err(InvalidBoardError::InvalidPosition); //TODO better error
            }
        }
        Ok(())
    }
}

impl<'a> Move<'a, Fox> {
    pub fn new(board: &'a Board, from: &'a Fox, to: Fox) -> Self {
        Move { board, from, to }
    }

    pub fn is_valid(&self) -> Result<(), InvalidBoardError> {
        if self.to.pos1.0 + self.to.pos1.1 > self.to.pos2.0 + self.to.pos2.1 {
            return Err(InvalidBoardError::FoxNotNormalized);
        }
        if self.to == *self.from {
            return Err(InvalidBoardError::InvalidPosition);
        }
        if self.to.pos1.0 >= 5 || self.to.pos1.1 >= 5 || self.to.pos2.0 >= 5 || self.to.pos2.1 >= 5
        {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.to.pos1.0.abs_diff(self.to.pos2.0) != 1
            && self.to.pos1.1.abs_diff(self.to.pos2.1) == 0
            || self.to.pos1.1.abs_diff(self.to.pos2.1) != 1
                && self.to.pos1.0.abs_diff(self.to.pos2.0) == 0
        {
            return Err(InvalidBoardError::FoxNotConnected);
        }
        if self.to.pos1.0 % 2 == 0 && self.to.pos1.1 % 2 == 0
            || self.to.pos2.0 % 2 == 0 && self.to.pos2.1 % 2 == 0
        {
            return Err(InvalidBoardError::FoxNotOnOddLane);
        }

        // check that all fields between the from and to positions are empty (note that fox can intersect with itself)
        if self.from.pos1.0 == self.to.pos1.0
            && self.from.pos1.0 == self.to.pos2.0
            && self.from.pos1.0 == self.from.pos2.0
        {
            let pos_diff = self
                .to
                .pos1
                .1
                .checked_signed_diff(self.from.pos1.1)
                .unwrap();

            let mut range = if pos_diff.signum() > 0 {
                1..pos_diff
            } else {
                pos_diff..0
            };

            if !range.all(|pos| {
                self.board
                    .get_filled_positions()
                    .intersection(&HashSet::from([
                        (self.from.pos1.0, (self.from.pos1.1 as isize + pos) as usize),
                        (self.from.pos2.0, (self.from.pos2.1 as isize + pos) as usize),
                    ]))
                    .collect::<HashSet<&(usize, usize)>>()
                    .difference(&HashSet::from([&self.from.pos1, &self.from.pos2]))
                    .collect::<HashSet<&&(usize, usize)>>()
                    .is_empty()
            }) {
                return Err(InvalidBoardError::InvalidPosition); //TODO better error
            }
        } else if self.from.pos1.1 == self.to.pos1.1
            && self.from.pos1.1 == self.to.pos2.1
            && self.from.pos1.1 == self.from.pos2.1
        {
            let pos_diff = self
                .to
                .pos1
                .0
                .checked_signed_diff(self.from.pos1.0)
                .unwrap();

            let mut range = if pos_diff.signum() > 0 {
                1..pos_diff
            } else {
                pos_diff..0
            };

            if !range.all(|pos| {
                self.board
                    .get_filled_positions()
                    .intersection(&HashSet::from([
                        ((self.from.pos1.0 as isize + pos) as usize, self.from.pos1.1),
                        ((self.from.pos2.0 as isize + pos) as usize, self.from.pos2.1),
                    ]))
                    .collect::<HashSet<&(usize, usize)>>()
                    .difference(&HashSet::from([&self.from.pos1, &self.from.pos2]))
                    .collect::<HashSet<&&(usize, usize)>>()
                    .is_empty()
            }) {
                return Err(InvalidBoardError::InvalidPosition); //TODO better error
            }
        } else {
            // check that fox moves in a straight line
            return Err(InvalidBoardError::InvalidPosition);
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            bunnies: HashSet::new(),
            foxes: HashSet::new(),
            mushrooms: HashSet::new(),
        }
    }

    pub fn move_bunny(&self, mv: Move<Bunny>) -> Result<Self, InvalidBoardError> {
        mv.is_valid()?;
        let mut new_board = self.clone();
        new_board.bunnies.remove(&mv.from);
        new_board.add_bunny(mv.to)?;
        Ok(new_board)
    }

    pub fn move_fox(&self, mv: Move<Fox>) -> Result<Self, InvalidBoardError> {
        mv.is_valid()?;
        let mut new_board = self.clone();
        new_board.foxes.remove(&mv.from);
        new_board.add_fox(mv.to)?;
        Ok(new_board)
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
        self.bunnies.insert(bunny);
        Ok(())
    }

    pub fn add_fox(&mut self, fox: Fox) -> Result<(), InvalidBoardError> {
        self.is_fox_placement_position_valid(fox.pos1, fox.pos2)?;
        self.foxes.insert(fox);
        Ok(())
    }

    pub fn add_mushroom(&mut self, mushroom: Mushroom) -> Result<(), InvalidBoardError> {
        self.is_mushroom_placement_position_valid(mushroom.pos)?;
        self.mushrooms.insert(mushroom);
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
}
