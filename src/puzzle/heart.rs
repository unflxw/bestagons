use std::collections::HashMap;

use crate::grid::hexagon::Hexagon;
use crate::grid::Position;
use rand::Rng;

use super::board::Board;
use super::puzzle::Generator;
use super::puzzle::Puzzle;
use super::Hint;

pub struct HeartGenerator;

impl<T: Rng> Generator<T> for HeartGenerator {
    fn generate(&self, rng: &mut T) -> Puzzle {
        let radius = 5;

        let mut hints = HashMap::new();

        /*
             X X X X X X
            X X X X X X X
           X R R X X R R X
          X R R R X R R R X
         X X R R R R R R X X
        X X X R R R R R X X X
         X X X R R R R X X X
          X X X R R R X X X
           X X X R R X X X
            X X X R X X X
             X X X X X X
        */

        for heart_position in [
            Position::new((-1, 4, -3)).unwrap(),
            Position::new((0, 3, -3)).unwrap(),
            Position::new((3, 0, -3)).unwrap(),
            Position::new((4, -1, -3)).unwrap(),
            Position::new((-2, 4, -2)).unwrap(),
            Position::new((-1, 3, -2)).unwrap(),
            Position::new((0, 2, -2)).unwrap(),
            Position::new((2, 0, -2)).unwrap(),
            Position::new((3, -1, -2)).unwrap(),
            Position::new((4, -2, -2)).unwrap(),
            Position::new((-2, 3, -1)).unwrap(),
            Position::new((-1, 2, -1)).unwrap(),
            Position::new((0, 1, -1)).unwrap(),
            Position::new((1, 0, -1)).unwrap(),
            Position::new((2, -1, -1)).unwrap(),
            Position::new((3, -2, -1)).unwrap(),
            Position::new((-2, 2, 0)).unwrap(),
            Position::new((-1, 1, 0)).unwrap(),
            Position::new((0, 0, 0)).unwrap(),
            Position::new((1, -1, 0)).unwrap(),
            Position::new((2, -2, 0)).unwrap(),
            Position::new((-2, 1, 1)).unwrap(),
            Position::new((-1, 0, 1)).unwrap(),
            Position::new((0, -1, 1)).unwrap(),
            Position::new((1, -2, 1)).unwrap(),
            Position::new((-2, 0, 2)).unwrap(),
            Position::new((-1, -1, 2)).unwrap(),
            Position::new((0, -2, 2)).unwrap(),
            Position::new((-2, -1, 3)).unwrap(),
            Position::new((-1, -2, 3)).unwrap(),
            Position::new((-2, -2, 4)).unwrap(),
        ] {
            hints.insert(-heart_position, Hint(true, false, false));
        }

        for position in Hexagon::new(Position::zero(), radius).unwrap() {
            hints.entry(position).or_insert(Hint(false, true, true));
        }

        let board = Board::random_from_hints(rng, radius, hints.into_iter());
        Puzzle::with_clues(board.unwrap())
    }
}
