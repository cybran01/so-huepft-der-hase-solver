use std::fmt::Display;

pub struct Bunny {
    pub pos: (usize, usize),
}

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

pub struct Mushroom {
    pub pos: (usize, usize),
}

pub struct Board {
    pub bunnies: Vec<Bunny>,
    pub foxes: Vec<Fox>,
    pub mushrooms: Vec<Mushroom>,
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

    pub fn get_filled_positions(&self) -> Vec<(usize, usize)> {
        let mut filled_positions = Vec::new();

        for bunny in &self.bunnies {
            filled_positions.push(bunny.pos);
        }

        for fox in &self.foxes {
            filled_positions.push(fox.pos1);
            filled_positions.push(fox.pos2);
        }

        for mushroom in &self.mushrooms {
            filled_positions.push(mushroom.pos);
        }

        filled_positions
    }

    pub fn add_bunny(&mut self, bunny: Bunny) -> Result<(), InvalidBoardError> {
        if bunny.pos.0 >= 5 || bunny.pos.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.get_filled_positions().contains(&bunny.pos) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        if self.bunnies.len() >= 3 {
            return Err(InvalidBoardError::TooManyBunnies);
        }
        self.bunnies.push(bunny);
        Ok(())
    }

    pub fn add_fox(&mut self, fox: Fox) -> Result<(), InvalidBoardError> {
        if fox.pos1.0 >= 5 || fox.pos1.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if fox.pos2.0 >= 5 || fox.pos2.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.get_filled_positions().contains(&fox.pos1)
            || self.get_filled_positions().contains(&fox.pos2)
        {
            return Err(InvalidBoardError::InvalidPosition);
        }
        if fox.pos1.0.abs_diff(fox.pos2.0) != 1 && fox.pos1.1.abs_diff(fox.pos2.1) == 0
            || fox.pos1.1.abs_diff(fox.pos2.1) != 1 && fox.pos1.0.abs_diff(fox.pos2.0) == 0
        {
            return Err(InvalidBoardError::FoxNotConnected);
        }
        if fox.pos1.0 % 2 == 0 && fox.pos1.1 % 2 == 0 || fox.pos2.0 % 2 == 0 && fox.pos2.1 % 2 == 0
        {
            return Err(InvalidBoardError::FoxNotOnOddLane);
        }
        if fox.pos1.0 + fox.pos1.1 > fox.pos2.0 + fox.pos2.1 {
            return Err(InvalidBoardError::FoxNotNormalized);
        }
        if self.foxes.len() >= 2 {
            return Err(InvalidBoardError::TooManyFoxes);
        }
        self.foxes.push(fox);
        Ok(())
    }

    pub fn add_mushroom(&mut self, mushroom: Mushroom) -> Result<(), InvalidBoardError> {
        if mushroom.pos.0 >= 5 || mushroom.pos.1 >= 5 {
            return Err(InvalidBoardError::OutOfBounds);
        }
        if self.get_filled_positions().contains(&mushroom.pos) {
            return Err(InvalidBoardError::InvalidPosition);
        }
        if self.mushrooms.len() >= 3 {
            return Err(InvalidBoardError::TooManyMushrooms);
        }
        self.mushrooms.push(mushroom);
        Ok(())
    }
}
