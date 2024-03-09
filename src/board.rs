use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::ops::{BitAnd, Sub};

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

const ZERO: Clue = Clue(0, 0, 0);

impl Clue {
    pub fn new(red: Count, green: Count, blue: Count) -> Self {
        Clue(red, green, blue)
    }

    pub fn zero() -> Self {
        ZERO.clone()
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

    pub fn hint(&self) -> Hint {
        Hint(self.red() > 0, self.green() > 0, self.blue() > 0)
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

#[derive(Debug, Clone)]
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

    pub fn is_solved(&self) -> bool {
        self.hexagon
            .into_iter()
            .all(|position| self.cells.get(&position).is_some())
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
    pub fn new(
        board: Board,
        clue_iterator: impl Iterator<Item = ((Direction, Distance), Clue)>,
    ) -> Self {
        let mut clues: HashMap<(Direction, Distance), Clue> = HashMap::new();

        for (key, clue) in clue_iterator {
            clues.insert(key, clue);
        }

        Puzzle {
            board,
            clues: HashMap::new(),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn clues(&self) -> &HashMap<(Direction, Distance), Clue> {
        &self.clues
    }

    pub fn clear(&mut self) {
        self.board = Board::new(self.board().hexagon().radius()).unwrap();
    }

    pub fn with_clues(board: Board) -> Self {
        let mut clues: HashMap<(Direction, Distance), Clue> = HashMap::new();

        for (key, clue) in board.clues() {
            clues.insert(key, clue);
        }

        Puzzle { board, clues }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for direction in Direction::normalized() {
            for _ in 0..self.board.hexagon().radius() * 3 + 1 {
                f.write_char(' ')?;
            }

            f.write_str(match direction {
                Direction::XY => "XY",
                Direction::YZ => "YZ",
                Direction::ZX => "ZX",
                _ => unreachable!(),
            })?;

            f.write_char('\n')?;

            for _ in 0..self.board.hexagon().radius() * 3 {
                f.write_char(' ')?;
            }
            f.write_str("--->\n")?;

            let segments = self.board.segments(direction);
            for (distance, segment) in segments {
                let padding = distance.abs();
                for _ in 0..padding {
                    f.write_char(' ')?;
                }
                for (_position, cell) in segment {
                    use Cell::*;

                    f.write_char(match cell {
                        Some(Red) => 'R',
                        Some(Green) => 'G',
                        Some(Blue) => 'B',
                        None => '?',
                    })?;
                    f.write_char(' ')?;
                }

                let clue = self
                    .clues
                    .get(&(direction, distance))
                    .cloned()
                    .unwrap_or(Clue::zero());

                f.write_str(&format!(
                    "- ({} {} {})",
                    clue.red(),
                    clue.green(),
                    clue.blue()
                ))?;

                f.write_char('\n')?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hint(bool, bool, bool);

impl Hint {
    fn any() -> Self {
        Hint(true, true, true)
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

    fn solution(&self) -> Option<Cell> {
        use Cell::*;

        match (self.red(), self.green(), self.blue()) {
            (true, false, false) => Some(Red),
            (false, true, false) => Some(Green),
            (false, false, true) => Some(Blue),
            _ => None,
        }
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

pub struct Solver {
    puzzle: Puzzle,
    solution: Board,
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Self {
        let solution = puzzle.board.clone();
        Solver {
            puzzle,
            solution: solution,
        }
    }

    pub fn solve_once(&mut self) {
        for (position, hint) in self.computed_hints() {
            if let Some(cell) = hint.solution() {
                self.solution.insert(position, cell);
            }
        }
    }

    pub fn solution(&self) -> &Board {
        &self.solution
    }

    pub fn computed_hints(&self) -> HashMap<Position, Hint> {
        let mut hints = HashMap::new();

        for ((direction, distance), clue) in self.computed_clues() {
            let clue_hint = clue.hint();
            for position in self
                .puzzle
                .board()
                .hexagon()
                .segment(distance, direction)
                .unwrap()
            {
                let hint = hints.get(&position).cloned().unwrap_or(Hint::any());
                hints.insert(position, hint & clue_hint);
            }
        }

        hints
    }

    pub fn computed_clues(&self) -> HashMap<(Direction, Distance), Clue> {
        let mut clues = self.puzzle.clues().clone();

        for (key, solution_clue) in self.solution.clues() {
            let puzzle_clue = clues.get(&key).cloned().unwrap();
            let clue = puzzle_clue - solution_clue;
            clues.insert(key, clue);
        }

        clues
    }
}
