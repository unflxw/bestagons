use crate::line::{Line, LineIterator};
use crate::position::{Direction, Distance, Position};

// A bounded set of points that stretch from a given start point
// in a given direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Segment {
    line: Line,
    length: Distance,
}

#[derive(Debug, Copy, Clone)]
pub enum SegmentError {
    InsufficientLength(Distance),
}

impl Segment {
    pub fn new(
        origin: Position,
        length: Distance,
        direction: Direction,
    ) -> Result<Self, SegmentError> {
        if length > 0 {
            Ok(Segment {
                line: Line::new(origin, direction),
                length,
            })
        } else {
            Err(SegmentError::InsufficientLength(length))
        }
    }

    pub fn start(&self) -> Position {
        self.line.origin()
    }

    pub fn end(&self) -> Position {
        self.line.position(self.length - 1)
    }

    pub fn length(&self) -> Distance {
        self.length
    }

    pub fn position(&self, distance: Distance) -> Option<Position> {
        if distance >= 0 && distance < self.length {
            Some(self.line.position(distance))
        } else {
            None
        }
    }

    pub fn direction(&self) -> Direction {
        self.line.direction()
    }

    pub fn line(&self) -> Line {
        self.line
    }
}

impl IntoIterator for Segment {
    type Item = Position;

    type IntoIter = std::iter::Take<LineIterator>;

    fn into_iter(self) -> Self::IntoIter {
        self.line.into_iter().take(self.length as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let segment = Segment::new(Position::new((1, 2, -3)).unwrap(), 3, Direction::XZ).unwrap();
        let mut iterator = segment.into_iter();

        assert_eq!((1, 2, -3), iterator.next().unwrap().into());
        assert_eq!((1, 2, -3), segment.start().into());
        assert_eq!((2, 2, -4), iterator.next().unwrap().into());
        assert_eq!((3, 2, -5), iterator.next().unwrap().into());
        assert_eq!((3, 2, -5), segment.end().into());
        assert!(iterator.next().is_none());
    }

    #[test]
    fn position() {
        let segment = Segment::new(Position::new((1, 2, -3)).unwrap(), 3, Direction::XZ).unwrap();

        assert_eq!(segment.start(), segment.position(0).unwrap());
        assert_eq!((1, 2, -3), segment.position(0).unwrap().into());
        assert_eq!((2, 2, -4), segment.position(1).unwrap().into());
        assert_eq!((3, 2, -5), segment.position(2).unwrap().into());
        assert_eq!(segment.end(), segment.position(2).unwrap());

        assert!(segment.position(-1).is_none());
        assert!(segment.position(-3).is_none());
    }
}
