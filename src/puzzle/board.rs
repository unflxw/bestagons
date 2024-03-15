use rand::Rng;
use std::collections::HashMap;

use super::puzzle::GeneratorFn;
use super::puzzle::Puzzle;
use super::{Cell, Clue, Hint};
use crate::grid::hexagon::{Hexagon, HexagonError};
use crate::grid::{Direction, Distance, Position};

#[derive(Debug, Clone)]
pub struct Board {
    hexagon: Hexagon,
    cells: HashMap<Position, Cell>,
}

impl Board {
    pub fn new(radius: Distance) -> Result<Self, HexagonError> {
        Ok(Board {
            hexagon: Hexagon::zero(radius)?,
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

    pub fn generator<T: Rng>(radius: Distance) -> GeneratorFn<T> {
        Box::new(move |rng: &mut T| Puzzle::with_clues(Board::random(rng, radius).unwrap()))
    }

    pub fn random_from_hints(
        rng: &mut impl Rng,
        radius: Distance,
        hints: impl Iterator<Item = (Position, Hint)>,
    ) -> Result<Self, HexagonError> {
        let mut board = Self::new(radius)?;

        for (position, hint) in hints {
            board.insert(position, hint.random(rng).unwrap())
        }

        Ok(board)
    }

    pub fn generator_from_hints<T: Rng>(
        radius: Distance,
        hints: impl Iterator<Item = (Position, Hint)>,
    ) -> GeneratorFn<T> {
        let hints = hints.collect::<Vec<_>>();
        Box::new(move |rng: &mut T| {
            Puzzle::with_clues(
                Board::random_from_hints(rng, radius, hints.clone().into_iter()).unwrap(),
            )
        })
    }

    pub fn is_solved(&self) -> bool {
        self.hexagon
            .into_iter()
            .all(|position| self.cells.get(&position).is_some())
    }

    pub fn insert(&mut self, position: Position, cell: Cell) {
        self.cells.insert(position, cell);
    }

    pub fn cells(&self) -> &HashMap<Position, Cell> {
        &self.cells
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
                Clue::from_cells(segment.filter_map(|(_position, cell)| cell)),
            )
        })
    }

    pub fn hexagon(&self) -> Hexagon {
        self.hexagon
    }
}
