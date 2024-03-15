use super::segment::Segment;
use super::{Direction, Distance, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ring {
    origin: Position,
    radius: Distance,
}

#[derive(Debug, Copy, Clone)]
pub enum RingError {
    InsufficientRadius(Distance),
}

impl Ring {
    pub fn new(origin: Position, radius: Distance) -> Result<Self, RingError> {
        if radius > 0 {
            Ok(Ring { origin, radius })
        } else {
            Err(RingError::InsufficientRadius(radius))
        }
    }

    pub fn zero(radius: Distance) -> Result<Self, RingError> {
        Self::new(Position::zero(), radius)
    }

    pub fn corner(&self, direction: Direction) -> Position {
        self.origin + (direction.position() * self.radius)
    }

    // The points forming a segment of the ring from a direction's
    // corner, included in the set of points, towards the next clockwise
    // direction's corner, not included in the set of points.
    pub fn segment(&self, direction: Direction) -> Segment {
        Segment::new(
            self.corner(direction),
            self.radius,
            direction.rotate().rotate(),
        )
        .unwrap()
    }
}

impl IntoIterator for Ring {
    type Item = Position;

    type IntoIter = RingIterator;

    fn into_iter(self) -> Self::IntoIter {
        RingIterator::new(self)
    }
}

pub struct RingIterator {
    segment: Segment,
    step: Distance,
}

impl RingIterator {
    pub fn new(ring: Ring) -> Self {
        RingIterator {
            segment: ring.segment(Direction::XY),
            step: 0,
        }
    }
}

impl Iterator for RingIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.segment.position(self.step) {
            self.step += 1;
            Some(position)
        } else {
            let direction = self.segment.direction().rotate();
            if direction == Direction::YZ {
                return None;
            }

            let origin = self.segment.line().position(self.step);
            let length = self.segment.length();
            self.segment = Segment::new(origin, length, direction).unwrap();
            self.step = 0;
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner() {
        use Direction::*;

        let ring = Ring::new(Position::new((1, -3, 2)).unwrap(), 2).unwrap();

        assert_eq!((3, -5, 2), ring.corner(XY).into());
        assert_eq!((3, -3, 0), ring.corner(XZ).into());
        assert_eq!((-1, -1, 2), ring.corner(YX).into());
        assert_eq!((1, -1, 0), ring.corner(YZ).into());
        assert_eq!((-1, -3, 4), ring.corner(ZX).into());
        assert_eq!((1, -5, 4), ring.corner(ZY).into());
    }

    #[test]
    fn segment() {
        let ring = Ring::new(Position::new((1, -3, 2)).unwrap(), 4).unwrap();
        let segment = ring.segment(Direction::XY);

        assert_eq!(ring.corner(Direction::XY), segment.position(0).unwrap());
        assert_eq!((5, -7, 2), segment.position(0).unwrap().into());
        assert_eq!((5, -6, 1), segment.position(1).unwrap().into());
        assert_eq!((5, -5, 0), segment.position(2).unwrap().into());
        assert_eq!((5, -4, -1), segment.position(3).unwrap().into());
        assert_eq!((5, -3, -2), ring.corner(Direction::XZ).into());
    }

    #[test]
    fn iterator() {
        let ring = Ring::new(Position::new((1, -3, 2)).unwrap(), 2).unwrap();
        let mut iterator = ring.into_iter();

        assert_eq!((3, -5, 2), iterator.next().unwrap().into());
        assert_eq!((3, -4, 1), iterator.next().unwrap().into());
        assert_eq!((3, -3, 0), iterator.next().unwrap().into());
        assert_eq!((2, -2, 0), iterator.next().unwrap().into());
        assert_eq!((1, -1, 0), iterator.next().unwrap().into());
        assert_eq!((0, -1, 1), iterator.next().unwrap().into());
        assert_eq!((-1, -1, 2), iterator.next().unwrap().into());
        assert_eq!((-1, -2, 3), iterator.next().unwrap().into());
        assert_eq!((-1, -3, 4), iterator.next().unwrap().into());
        assert_eq!((0, -4, 4), iterator.next().unwrap().into());
        assert_eq!((1, -5, 4), iterator.next().unwrap().into());
        assert_eq!((2, -5, 3), iterator.next().unwrap().into());
        assert!(iterator.next().is_none());
    }
}
