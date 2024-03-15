use super::{puzzle::Puzzle, solver::Solver};

pub trait ValidatorStrategy {
    fn is_valid(&self, puzzle: Puzzle) -> Option<bool>;
}

// Check that the puzzle requires (or does not require) solving through
// clue-wide constraints in order to be solved.
pub struct RequireClueSolving(pub bool);

impl ValidatorStrategy for RequireClueSolving {
    fn is_valid(&self, puzzle: Puzzle) -> Option<bool> {
        let mut solver = Solver::new(puzzle);
        while !solver.solution().is_solved() {
            if solver.solve_hints() {
                continue;
            }

            if solver.solve_clues() {
                return Some(self.0);
            } else {
                return None;
            }
        }

        Some(!self.0)
    }
}

// Check that the puzzle requires (or does not require) solving through
// overlapping hints in order to be solved.
pub struct RequireHintSolving(pub bool);

impl ValidatorStrategy for RequireHintSolving {
    fn is_valid(&self, puzzle: Puzzle) -> Option<bool> {
        let mut solver = Solver::new(puzzle);
        while !solver.solution().is_solved() {
            if solver.solve_clues() {
                continue;
            }
            if solver.solve_hints() {
                return Some(self.0);
            } else {
                return None;
            }
        }

        Some(!self.0)
    }
}

// Check that at most the given number of computed clues (the clues after
// factoring in the already placed cells) have less than two colors.
pub struct MaximumSolvedClues(pub usize);

impl ValidatorStrategy for MaximumSolvedClues {
    fn is_valid(&self, puzzle: Puzzle) -> Option<bool> {
        let solver = Solver::new(puzzle);
        Some(
            solver
                .computed_clues()
                .into_iter()
                .filter(|(_key, clue)| clue.is_solved())
                .count()
                <= self.0,
        )
    }
}

// Check that at most the given number of positions are already solved.
pub struct MaximumSolvedPositions(pub usize);

impl ValidatorStrategy for MaximumSolvedPositions {
    fn is_valid(&self, puzzle: Puzzle) -> Option<bool> {
        Some(puzzle.board().cells().len() <= self.0)
    }
}

pub struct Validator(Vec<Box<dyn ValidatorStrategy>>);

impl Validator {
    pub fn new(strategies: Vec<Box<dyn ValidatorStrategy>>) -> Self {
        Validator(strategies)
    }

    pub fn is_not_invalid(&self, puzzle: Puzzle) -> bool {
        self.0
            .iter()
            .all(|strategy| strategy.is_valid(puzzle.clone()) != Some(false))
    }

    pub fn is_valid(&self, puzzle: Puzzle) -> bool {
        self.0
            .iter()
            .all(|strategy| strategy.is_valid(puzzle.clone()) == Some(true))
    }
}
