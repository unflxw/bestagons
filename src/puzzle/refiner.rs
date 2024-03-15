use std::collections::HashMap;

use rand::Rng;

use crate::grid::{Direction, Distance, Position};

use super::{
    puzzle::{Generator, Puzzle},
    solver::Solver,
    validator::Validator,
    Cell, Clue,
};

// Attempts to refine a solution into a puzzle that meets the criteria
// of the given validator.
pub struct Refiner {
    validator: Validator,
}

impl Refiner {
    pub fn new(validator: Validator) -> Self {
        Refiner { validator }
    }

    pub fn refined<T: Rng>(&self, rng: &mut T, generator: impl Generator<T>) -> Puzzle {
        let mut refined = None;

        while refined.is_none() {
            let solution = generator.generate(rng);
            refined = self.refine(solution);
        }

        refined.unwrap()
    }

    pub fn refine(&self, solution: Puzzle) -> Option<Puzzle> {
        let mut puzzle = solution.clone();
        puzzle.clear();
        let mut solver = Solver::new(puzzle.clone());

        if !self.validator.is_not_invalid(puzzle.clone()) {
            return None;
        }

        while !solver.solve() {
            self.solve_cell(&solution, &mut puzzle, &mut solver);
            // if !self.validator.is_not_invalid(puzzle.clone()) {
            //     return None;
            // }
        }

        if !self.validator.is_valid(puzzle.clone()) {
            return None;
        }

        Some(puzzle)
    }

    fn lowest_computed_clue(
        computed_clues: HashMap<(Direction, Distance), Clue>,
    ) -> Option<((Direction, Distance), Clue)> {
        computed_clues
            .iter()
            .filter(|(_key, clue)| !clue.is_empty())
            .min_by_key(|(_key, clue)| clue.count())
            .map(|(key, clue)| (*key, *clue))
    }

    fn find_segment_unsolved_cell_position(
        solution: &Puzzle,
        solver: &Solver,
        direction: Direction,
        distance: Distance,
        cell: Cell,
    ) -> Option<Position> {
        solution
            .board()
            .segment(distance, direction)
            .unwrap()
            .find(|(position, found_cell)| {
                !solver.solution().cells().contains_key(position) && found_cell == &Some(cell)
            })
            .map(|(position, _)| position)
    }

    fn solve_cell(&self, solution: &Puzzle, _puzzle: &mut Puzzle, solver: &mut Solver) {
        let computed_clues = solver.computed_clues();
        let ((direction, distance), clue) = Self::lowest_computed_clue(computed_clues).unwrap();

        let max_cell = clue.max_cell().unwrap();
        let position = Self::find_segment_unsolved_cell_position(
            solution, solver, direction, distance, max_cell,
        )
        .unwrap();

        // Add that cell to the puzzle
        solver.mut_puzzle().mut_board().insert(position, max_cell);
        solver.mut_solution().insert(position, max_cell);
    }
}
