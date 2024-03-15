mod grid;
mod puzzle;

use puzzle::board::Board;
use rand::thread_rng;

use crate::puzzle::{
    puzzle::GeneratorFn,
    refiner::Refiner,
    validator::{
        MaximumSolvedClues, MaximumSolvedPositions, RequireClueSolving, RequireHintSolving,
        Validator,
    },
};

fn main() {
    let mut rng = thread_rng();
    // let generator = HeartGenerator;
    let generator: GeneratorFn<_> = Board::generator(5);
    let validator: Validator = Validator::new(vec![
        Box::new(RequireClueSolving(true)),
        Box::new(RequireHintSolving(true)),
        Box::new(MaximumSolvedClues(0)),
        Box::new(MaximumSolvedPositions(0)),
    ]);
    let refiner = Refiner::new(validator);
    let puzzle = refiner.refined(&mut rng, generator);
    println!("{puzzle}");
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4)
    // }
}
