use std::collections::HashMap;

use crate::grid::{Direction, Distance, Position};

use super::board::Board;
use super::puzzle::Puzzle;
use super::{Cell, Clue, Hint};

pub struct Solver {
    puzzle: Puzzle,
    solution: Board,
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Self {
        let solution = puzzle.board().clone();
        Solver { puzzle, solution }
    }

    pub fn puzzle(&self) -> &Puzzle {
        &self.puzzle
    }

    pub fn solution(&self) -> &Board {
        &self.solution
    }

    pub fn mut_puzzle(&mut self) -> &mut Puzzle {
        &mut self.puzzle
    }

    pub fn mut_solution(&mut self) -> &mut Board {
        &mut self.solution
    }

    pub fn solve_hints(&mut self) -> bool {
        let mut did_solve: bool = false;
        for (position, hint) in self.computed_hints() {
            if let Some(cell) = hint.solution() {
                if !self.solution.cells().contains_key(&position) {
                    self.solution.insert(position, cell);
                    did_solve = true;
                }
            }
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
                if self.solution.cells().contains_key(&position) {
                    continue;
                }

                hinted_clue = hinted_clue + hints.get(&position).unwrap().clue()
            }

            for cell in Cell::all() {
                if hinted_clue.cell(cell) == computed_clue.cell(cell) {
                    for position in segment {
                        if self.solution.cells().contains_key(&position) {
                            continue;
                        }

                        if hints.get(&position).unwrap().cell(cell) {
                            new.insert(position, cell);
                            did_solve = true;
                        }
                    }
                }
            }
        }

        for (position, cell) in new {
            self.solution.insert(position, cell);
        }

        did_solve
    }

    pub fn solve(&mut self) -> bool {
        while self.solve_hints() || self.solve_clues() {}

        self.solution.is_solved()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grid::ring::Ring, puzzle::board::Board};

    #[test]
    fn test_solver() {
        let mut board = Board::new(2).unwrap();

        board.insert(Position::zero(), Cell::Red);

        for position in Ring::zero(1).unwrap() {
            board.insert(position, Cell::Green);
        }

        for position in Ring::zero(2).unwrap() {
            board.insert(position, Cell::Blue);
        }

        let mut puzzle = Puzzle::with_clues(board);
        puzzle.clear();
        assert!(puzzle.board().cells().is_empty());
        let mut solver = Solver::new(puzzle);
        assert!(solver.solve());
    }
}
