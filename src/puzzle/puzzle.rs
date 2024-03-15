use std::collections::HashMap;
use std::fmt::{Display, Write};

use rand::Rng;

use super::board::Board;
use super::{Cell, Clue};
use crate::grid::{Direction, Distance};

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

    pub fn mut_board(&mut self) -> &mut Board {
        &mut self.board
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

pub type GeneratorFn<T> = Box<dyn Fn(&mut T) -> Puzzle>;

pub trait Generator<T: Rng> {
    fn generate(&self, rng: &mut T) -> Puzzle;
}

impl<T: Rng> Generator<T> for GeneratorFn<T> {
    fn generate(&self, rng: &mut T) -> Puzzle {
        self(rng)
    }
}
