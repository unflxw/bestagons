pub mod hexagon;
pub mod line;
pub mod ring;
pub mod segment;

use std::ops::{Add, Mul, Neg, Sub};

pub type Coordinate = i32;
pub type Coordinates = (i32, i32, i32);

pub type Distance = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(Coordinate, Coordinate);

const ZERO: Position = Position(0, 0);

#[derive(Debug, Copy, Clone)]
pub enum PositionError {
    InvalidCoordinates(Coordinates),
}

impl Position {
    pub fn zero() -> Self {
        ZERO
    }

    pub fn new(coordinates: Coordinates) -> Result<Self, PositionError> {
        let (x, y, z) = coordinates;

        if x + y + z != 0 {
            Err(PositionError::InvalidCoordinates(coordinates))
        } else {
            Ok(Position(x, y))
        }
    }

    pub fn x(&self) -> Coordinate {
        self.0
    }

    pub fn y(&self) -> Coordinate {
        self.1
    }

    pub fn z(&self) -> Coordinate {
        -self.0 - self.1
    }

    pub fn axis(&self, axis: Axis) -> Coordinate {
        use Axis::*;

        match axis {
            X => self.x(),
            Y => self.y(),
            Z => self.z(),
        }
    }

    pub fn distance(&self) -> Distance {
        self.x().abs().max(self.y().abs()).max(self.z().abs())
    }

    pub fn coordinates(&self) -> Coordinates {
        (*self).into()
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Self::Output {
        Position(self.x() + other.x(), self.y() + other.y())
    }
}

impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position(-self.x(), -self.y())
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Self::Output {
        Position(self.x() - other.x(), self.y() - other.y())
    }
}

impl Mul<Distance> for Position {
    type Output = Position;

    fn mul(self, other: Distance) -> Self::Output {
        Position(self.x() * other, self.y() * other)
    }
}

impl From<Position> for Coordinates {
    fn from(position: Position) -> Self {
        (position.x(), position.y(), position.z())
    }
}

impl TryFrom<Coordinates> for Position {
    type Error = PositionError;

    fn try_from(coordinates: Coordinates) -> Result<Self, Self::Error> {
        Position::new(coordinates)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    XY,
    XZ,
    YX,
    YZ,
    ZX,
    ZY,
}

const DIRECTIONS: [Direction; 6] = {
    use Direction::*;

    [XY, XZ, YX, YZ, ZX, ZY]
};

const NORMALIZED_DIRECTIONS: [Direction; 3] = {
    use Direction::*;

    [XY, YZ, ZX]
};

impl Direction {
    pub fn position(&self) -> Position {
        (*self).into()
    }

    pub fn all() -> [Direction; 6] {
        DIRECTIONS
    }

    pub fn normalized() -> [Direction; 3] {
        NORMALIZED_DIRECTIONS
    }

    // Returns a tuple of positive, neutral and negative axes.
    pub fn axes(&self) -> (Axis, Axis, Axis) {
        use Axis::*;
        use Direction::*;

        match self {
            XY => (X, Z, Y),
            XZ => (X, Y, Z),
            YX => (Y, Z, X),
            YZ => (Y, X, Z),
            ZX => (Z, Y, X),
            ZY => (Z, X, Y),
        }
    }

    pub fn positive_axis(&self) -> Axis {
        self.axes().0
    }

    pub fn neutral_axis(&self) -> Axis {
        self.axes().1
    }

    pub fn negative_axis(&self) -> Axis {
        self.axes().2
    }

    // Normalizes directions that have opposite orientations
    // but equal alignment, such that, in the cyclic sequence
    // `... -> X -> Y -> Z -> X -> ...`, the positive axis is
    // the immediate predecessor of the negative axis.
    pub fn normalize(&self) -> Self {
        use Direction::*;

        match self {
            YX | ZY | XZ => self.opposite(),
            other => *other,
        }
    }

    pub fn opposite(&self) -> Self {
        use Direction::*;

        match self {
            XY => YX,
            XZ => ZX,
            YX => XY,
            YZ => ZY,
            ZX => XZ,
            ZY => YZ,
        }
    }

    pub fn rotate(&self) -> Self {
        use Direction::*;

        match self {
            XY => XZ,
            XZ => YZ,
            YZ => YX,
            YX => ZX,
            ZX => ZY,
            ZY => XY,
        }
    }

    pub fn rotate_back(&self) -> Self {
        self.opposite().rotate().rotate()
    }
}

const XY_UNIT: Position = Position(1, -1);
const XZ_UNIT: Position = Position(1, 0);
const YX_UNIT: Position = Position(-1, 1);
const YZ_UNIT: Position = Position(0, 1);
const ZX_UNIT: Position = Position(-1, 0);
const ZY_UNIT: Position = Position(0, -1);

impl From<Direction> for Position {
    fn from(direction: Direction) -> Self {
        use Direction::*;

        match direction {
            XY => XY_UNIT,
            XZ => XZ_UNIT,
            YX => YX_UNIT,
            YZ => YZ_UNIT,
            ZX => ZX_UNIT,
            ZY => ZY_UNIT,
        }
    }
}
