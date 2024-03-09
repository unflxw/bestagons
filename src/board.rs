use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

use crate::hexagon::{Hexagon, HexagonError};
use crate::position::{Direction, Distance, Position};

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
    pub fn random() -> Self {
        *CELLS.choose(&mut thread_rng()).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Clue(Count, Count, Count);

impl Clue {
    pub fn new(red: Count, green: Count, blue: Count) -> Self {
        Clue(red, green, blue)
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

    pub fn counts(&self) -> [(Cell, Count); 3] {
        use Cell::*;

        [
            (Red, self.red()),
            (Green, self.green()),
            (Blue, self.blue()),
        ]
    }
}

#[derive(Debug)]
pub struct Board {
    hexagon: Hexagon,
    cells: HashMap<Position, Cell>,
}

impl Board {
    pub fn new(radius: Distance) -> Result<Self, HexagonError> {
        Ok(Board {
            hexagon: Hexagon::new(Position::zero(), radius)?,
            cells: HashMap::new(),
        })
    }

    pub fn from_cells(
        radius: Distance,
        cells: impl Iterator<Item = (Position, Cell)>,
    ) -> Result<Self, HexagonError> {
        let mut board = Board::new(radius)?;
        for (position, cell) in cells {
            board.insert(position, cell)
        }

        Ok(board)
    }

    pub fn random(radius: Distance) -> Result<Self, HexagonError> {
        let mut board = Self::new(radius)?;

        for position in board.hexagon() {
            board.insert(position, Cell::random())
        }

        Ok(board)
    }

    pub fn insert(&mut self, position: Position, cell: Cell) {
        self.cells.insert(position, cell);
    }

    pub fn segment(
        &self,
        distance: Distance,
        direction: Direction,
    ) -> Option<impl Iterator<Item = (Position, Option<Cell>)> + '_> {
        self.hexagon.segment(distance, direction).map(|segment| {
            segment
                .into_iter()
                .map(|position| (position, self.cells.get(&position).cloned()))
        })
    }

    pub fn segments(
        &self,
        direction: Direction,
    ) -> impl Iterator<
        Item = (
            Distance,
            impl Iterator<Item = (Position, Option<Cell>)> + '_,
        ),
    > {
        self.hexagon.segments(direction).map(|(distance, segment)| {
            (
                distance,
                segment
                    .into_iter()
                    .map(|position| (position, self.cells.get(&position).cloned())),
            )
        })
    }

    pub fn normalized_segments(
        &self,
    ) -> impl Iterator<
        Item = (
            (Direction, Distance),
            impl Iterator<Item = (Position, Option<Cell>)> + '_,
        ),
    > {
        Direction::normalized().into_iter().flat_map(|direction| {
            self.segments(direction)
                .map(move |(distance, segment)| ((direction, distance), segment))
        })
    }

    pub fn clues(&self) -> impl Iterator<Item = ((Direction, Distance), Clue)> + '_ {
        self.normalized_segments().map(|(key, segment)| {
            (
                key,
                Clue::from_cells(
                    segment
                        .map(|(_position, cell)| cell)
                        .filter(Option::is_some)
                        .map(Option::unwrap),
                ),
            )
        })
    }

    pub fn hexagon(&self) -> Hexagon {
        self.hexagon
    }
}

#[derive(Debug)]
pub struct Puzzle {
    board: Board,
    clues: HashMap<(Direction, Distance), Clue>,
}

impl Puzzle {
    pub fn new(board: Board) -> Self {
        let clues = board.clues().collect();
        Puzzle { board, clues }
    }
}
