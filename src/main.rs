mod board;
mod hexagon;
mod line;
mod position;
mod ring;
mod segment;

use board::{Board, Puzzle, Solver};

fn main() {
    let mut puzzle = Puzzle::with_clues(Board::random(2).unwrap());
    println!("solution:\n {puzzle}");
    puzzle.clear();
    println!("cleared:\n {puzzle}");
    let mut solver = Solver::new(puzzle);
    println!("{:?}", solver.computed_hints());
    solver.solve_once();
    let mut solved = Puzzle::with_clues(solver.solution().clone());
    println!("solved 1:\n {solved}");
    solver.solve_once();
    let mut solved = Puzzle::with_clues(solver.solution().clone());
    println!("solved 2:\n {solved}");
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4)
    // }
}
