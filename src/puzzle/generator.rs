use crate::grid::Distance;
use rand::Rng;

use super::board::Board;
use super::puzzle::Puzzle;
use super::solver::Solver;

pub fn generate(rng: &mut impl Rng, radius: Distance) -> bool {
    let mut solution = Puzzle::with_clues(Board::random(rng, radius).unwrap());
    println!("solution:\n{solution}");
    let mut puzzle = solution.clone();
    puzzle.clear();

    let mut solver = Solver::new(puzzle);
    let no_solved_clues = solver.computed_clues().into_iter().all(|(_key, clue)| {
        [clue.red(), clue.green(), clue.blue()]
            .into_iter()
            .filter(|count| *count > 0)
            .count()
            > 1
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
            .board()
            .segment(*distance, *direction)
            .unwrap()
            .find(|(position, cell)| {
                !solver.solution().cells().contains_key(position) && cell.clone() == Some(max_cell)
            })
            .unwrap();

        solver.mut_puzzle().mut_board().insert(position, max_cell);
        solver.mut_solution().insert(position, max_cell);

        println!("Added clue {max_cell:?} at {position:?}");
        added_clue_count += 1;
        if added_clue_count > added_clue_limit {
            println!("Not generating puzzle, too many added clues");
            return false;
        }
    }

    let mut solved = Puzzle::with_clues(solver.solution().clone());
    println!("solved solution:\n{solved}");
    println!("puzzle:\n{}", solver.puzzle());

    let mut resolver = Solver::new(solver.puzzle().clone());
    let no_solved_clues = resolver.computed_clues().into_iter().all(|(_key, clue)| {
        [clue.red(), clue.green(), clue.blue()]
            .into_iter()
            .filter(|count| *count > 0)
            .count()
            > 1
    });

    println!("re-solving:");

    let mut requires_clue_solving = false;
    let mut complexity = -(added_clue_count as i32);

    while !resolver.solution().is_solved() {
        if resolver.solve_hints() {
            complexity += 1;
            continue;
        }
        resolver.solve_clues();
        complexity += 3;
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
