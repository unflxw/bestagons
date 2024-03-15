pub mod board;
pub mod heart;
pub mod puzzle;
pub mod refiner;
pub mod solver;
pub mod validator;

use rand::{seq::IteratorRandom, seq::SliceRandom, Rng};
use std::ops::{Add, BitAnd, Sub};

type Count = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Red,
    Green,
    Blue,
}

const CELLS: [Cell; 3] = {
    use Cell::*;

    [Red, Green, Blue]
};

impl Cell {
    pub fn random(rng: &mut impl Rng) -> Self {
        *CELLS.choose(rng).unwrap()
    }

    pub fn all() -> [Cell; 3] {
        CELLS
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Clue(Count, Count, Count);

const ZERO: Clue = Clue(0, 0, 0);

impl Clue {
    pub fn new(red: Count, green: Count, blue: Count) -> Self {
        Clue(red, green, blue)
    }

    pub fn zero() -> Self {
        ZERO
    }

    pub fn is_empty(&self) -> bool {
        self.red() == 0 && self.green() == 0 && self.blue() == 0
    }

    pub fn count(&self) -> Count {
        self.red() + self.blue() + self.green()
    }

    // Returns the cell type with the lowest non-zero value.
    pub fn min_cell(&self) -> Option<Cell> {
        Cell::all()
            .into_iter()
            .filter(|cell| self.cell(*cell) > 0)
            .min_by_key(|cell| self.cell(*cell))
    }

    // Returns the cell type with the highest non-zero value.
    pub fn max_cell(&self) -> Option<Cell> {
        Cell::all()
            .into_iter()
            .filter(|cell| self.cell(*cell) > 0)
            .max_by_key(|cell| self.cell(*cell))
    }

    pub fn from_cells(cells: impl Iterator<Item = Cell>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cell in cells {
            use Cell::*;

            match cell {
                Red => red += 1,
                Green => green += 1,
                Blue => blue += 1,
            }
        }

        Clue::new(red, green, blue)
    }

    pub fn red(&self) -> Count {
        self.0
    }

    pub fn green(&self) -> Count {
        self.1
    }

    pub fn blue(&self) -> Count {
        self.2
    }

    pub fn cell(&self, cell: Cell) -> Count {
        use Cell::*;

        match cell {
            Red => self.red(),
            Green => self.green(),
            Blue => self.blue(),
        }
    }

    pub fn hint(&self) -> Hint {
        Hint(self.red() > 0, self.green() > 0, self.blue() > 0)
    }

    pub fn is_solved(&self) -> bool {
        [self.red(), self.green(), self.blue()]
            .into_iter()
            .filter(|count| *count > 0)
            .count()
            == 1
    }
}

impl Add for Clue {
    type Output = Clue;

    fn add(self, other: Self) -> Self::Output {
        Clue(
            self.red() + other.red(),
            self.green() + other.green(),
            self.blue() + other.blue(),
        )
    }
}

impl Sub for Clue {
    type Output = Clue;

    fn sub(self, other: Self) -> Self::Output {
        Clue(
            self.red() - other.red(),
            self.green() - other.green(),
            self.blue() - other.blue(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hint(bool, bool, bool);

impl Hint {
    fn any() -> Self {
        Hint(true, true, true)
    }

    fn none() -> Self {
        Hint(false, false, false)
    }

    fn red(&self) -> bool {
        self.0
    }

    fn green(&self) -> bool {
        self.1
    }

    fn blue(&self) -> bool {
        self.2
    }

    fn random(&self, rng: &mut impl Rng) -> Option<Cell> {
        use Cell::*;
        [
            (Red, self.red()),
            (Green, self.green()),
            (Blue, self.blue()),
        ]
        .into_iter()
        .filter(|(_cell, hint)| *hint)
        .map(|(cell, _hint)| cell)
        .choose(rng)
    }

    fn cell(&self, cell: Cell) -> bool {
        use Cell::*;
        match cell {
            Red => self.red(),
            Green => self.green(),
            Blue => self.blue(),
        }
    }

    fn solution(&self) -> Option<Cell> {
        use Cell::*;

        match (self.red(), self.green(), self.blue()) {
            (true, false, false) => Some(Red),
            (false, true, false) => Some(Green),
            (false, false, true) => Some(Blue),
            _ => None,
        }
    }

    fn clue(&self) -> Clue {
        Clue(
            if self.red() { 1 } else { 0 },
            if self.green() { 1 } else { 0 },
            if self.blue() { 1 } else { 0 },
        )
    }
}

impl BitAnd for Hint {
    type Output = Hint;

    fn bitand(self, other: Self) -> Self::Output {
        Hint(
            self.red() && other.red(),
            self.green() && other.green(),
            self.blue() && other.blue(),
        )
    }
}
