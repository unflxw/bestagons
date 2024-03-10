use rand::{Rng, seq::SliceRandom};
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::ops::{Add, BitAnd, Sub};

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
    pub fn random(rng: &mut impl Rng) -> Self {
        *CELLS.choose(rng).unwrap()
    }

    pub fn all() -> [Cell; 3] {
        CELLS.clone()
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

    pub fn is_empty(&self) -> bool {
        self.red() == 0 && self.green() == 0 && self.blue() == 0
    }

    pub fn count(&self) -> Count {
        self.red() + self.blue() + self.green()
    }

    // Returns the cell type with the lowest non-zero value.
    pub fn min_cell(&self) -> Option<Cell> {
        Cell::all().into_iter()
            .filter(|cell| self.cell(*cell) > 0)
            .min_by_key(|cell| self.cell(*cell))
    }

    // Returns the cell type with the highest non-zero value.
    pub fn max_cell(&self) -> Option<Cell> {
        Cell::all().into_iter()
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

    pub fn random(rng: &mut impl Rng, radius: Distance) -> Result<Self, HexagonError> {
        let mut board = Self::new(radius)?;

        for position in board.hexagon() {
            board.insert(position, Cell::random(rng))
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

#[derive(Debug, Clone)]
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

    pub fn solve_hints(&mut self) -> bool {
        let mut did_solve: bool = false;
        for (position, hint) in self.computed_hints() {
            if let Some(cell) = hint.solution() {
                if !self.solution.cells.contains_key(&position) {
                    self.solution.insert(position, cell);
                    did_solve = true;
                    println!("Solved hint for {position:?} to {cell:?}")
                }
            }
        }

        if did_solve {
            println!("Done solving hints")
        } else {
            println!("Could not solve hints")
        }

        did_solve
    }

    pub fn solve_clues(&mut self) -> bool {
        let mut did_solve: bool = false;

        let hints = self.computed_hints();
        let mut new: HashMap<Position, Cell> = HashMap::new();

        for ((direction, distance), computed_clue) in self.computed_clues() {
            let segment = self
                .puzzle
                .board()
                .hexagon()
                .segment(distance, direction)
                .unwrap();

            let mut hinted_clue = Clue::zero();

            for position in segment {
                if self.solution.cells.contains_key(&position) {
                    continue;
                }

                hinted_clue = hinted_clue + hints.get(&position).unwrap().clue()
            }

            for cell in Cell::all() {
                if hinted_clue.cell(cell) == computed_clue.cell(cell) {
                    for position in segment {
                        if self.solution.cells.contains_key(&position) {
                            continue;
                        }

                        if hints.get(&position).unwrap().cell(cell) {
                            new.insert(position, cell);
                            did_solve = true;
                            println!("Solved clue ({direction:?}, {distance:?}) for {position:?} to {cell:?}")
                        }
                    }
                }
            }
        }

        for (position, cell) in new {
            self.solution.insert(position, cell);
        }

        if did_solve {
            println!("Done solving clues")
        } else {
            println!("Could not solve clues")
        }

        did_solve
    }

    pub fn solve(&mut self) -> bool {
        while self.solve_hints() || self.solve_clues() {}

        let is_solved = self.solution.is_solved();

        if !is_solved {
            println!("Could not solve")
        } else {
            println!("Solved")
        }

        is_solved
    }

    pub fn solution(&self) -> &Board {
        &self.solution
    }

    pub fn computed_hints(&self) -> HashMap<Position, Hint> {
        let mut hints = HashMap::new();

        for ((direction, distance), clue) in self.computed_clues() {
            let clue_hint = clue.hint();
            let segment = self
                .puzzle
                .board()
                .hexagon()
                .segment(distance, direction)
                .unwrap();

            for position in segment {
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

pub fn generate(rng: &mut impl Rng, radius: Distance) -> bool {
    let mut solution = Puzzle::with_clues(Board::random(rng, radius).unwrap());
    println!("solution:\n{solution}");
    let mut puzzle = solution.clone();
    puzzle.clear();

    let mut solver = Solver::new(puzzle);
    let no_solved_clues = solver.computed_clues().into_iter().all(|(_key, clue)| {
        [clue.red(), clue.green(), clue.blue()].into_iter().filter(|count| *count > 0).count() > 1
    });

    if !no_solved_clues {
        println!("Not generating puzzle, has solved clues");
        return false;
    }

    let added_clue_limit = ((radius - 1) * (radius - 2) / 2).max(0) as usize;
    let mut added_clue_count = 0;

    while !solver.solve() {
        let computed_clues = solver.computed_clues();
        // Find the computed clue with the lowest total count
        let ((direction, distance), clue) = computed_clues
            .iter()
            .filter(|(key, clue)| !clue.is_empty())
            .max_by_key(|(key, clue)| clue.count())
            .unwrap();

        // Find the cell type with the lowest value in the clue
        let max_cell = clue.max_cell().unwrap();

        // Find one of the cells of that type in the solution
        let (position, _) = solution
            .board
            .segment(*distance, *direction)
            .unwrap()
            .find(|(position, cell)| {
                !solver.solution.cells.contains_key(position) && cell.clone() == Some(max_cell)
            })
            .unwrap();

        solver.puzzle.board.insert(position, max_cell);
        solver.solution.insert(position, max_cell);

        println!("Added clue {max_cell:?} at {position:?}");
        added_clue_count += 1;
        if added_clue_count > added_clue_limit {
            println!("Not generating puzzle, too many added clues");
            return false;
        }
    }

    let mut solved = Puzzle::with_clues(solver.solution().clone());
    println!("solved solution:\n{solved}");
    println!("puzzle:\n{}", solver.puzzle);

    let mut resolver = Solver::new(solver.puzzle.clone());
    let no_solved_clues = resolver.computed_clues().into_iter().all(|(_key, clue)| {
        [clue.red(), clue.green(), clue.blue()].into_iter().filter(|count| *count > 0).count() > 1
    });

    println!("re-solving:");

    let mut requires_clue_solving = false;
    let mut complexity = 0;

    while !resolver.solution.is_solved() {
        if resolver.solve_hints() {
            complexity += 1;
            continue;
        }
        resolver.solve_clues();
        complexity += 2;
        requires_clue_solving = true;
    }

    println!("requires clue solving? {requires_clue_solving}");
    println!("complexity: {complexity}");
    println!("no solved clues? {no_solved_clues}");

    no_solved_clues && requires_clue_solving
}

pub fn generate_good(rng: &mut impl Rng, radius: Distance) {
    while !generate(rng, radius) {}
}
