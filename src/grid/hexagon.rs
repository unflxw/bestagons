use super::line::Line;
use super::ring::{Ring, RingIterator};
use super::segment::Segment;
use super::{Direction, Distance, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hexagon {
    origin: Position,
    radius: Distance,
}

#[derive(Debug, Copy, Clone)]
pub enum HexagonError {
    InsufficientRadius(Distance),
}

impl Hexagon {
    pub fn new(origin: Position, radius: Distance) -> Result<Self, HexagonError> {
        if radius > 0 {
            Ok(Hexagon { origin, radius })
        } else {
            Err(HexagonError::InsufficientRadius(radius))
        }
    }

    pub fn origin(&self) -> Position {
        self.origin
    }

    pub fn radius(&self) -> Distance {
        self.radius
    }

    pub fn ring(&self, radius: Distance) -> Option<Ring> {
        if radius <= 0 || radius > self.radius {
            None
        } else {
            Some(Ring::new(self.origin, radius).unwrap())
        }
    }

    pub fn contains(&self, position: Position) -> bool {
        (position - self.origin).distance() <= self.radius
    }

    pub fn segment(&self, distance: Distance, direction: Direction) -> Option<Segment> {
        if distance.abs() > self.radius {
            None
        } else {
            let position = self.origin + (direction.rotate().position() * distance);
            let line = Line::new(position, direction);
            let iterator = line.into_iter().rev();
            let start = iterator
                .take_while(|position| self.contains(*position))
                .last()
                .unwrap_or(position);
            let length = self.radius * 2 - distance.abs() + 1;
            Some(Segment::new(start, length, direction).unwrap())
        }
    }

    pub fn segments(&self, direction: Direction) -> impl Iterator<Item = (Distance, Segment)> {
        let hexagon = self.clone();
        (-self.radius..=self.radius)
            .into_iter()
            .map(move |distance| (distance, hexagon.segment(distance, direction).unwrap()))
    }
}

impl IntoIterator for Hexagon {
    type Item = Position;

    type IntoIter = HexagonIterator;

    fn into_iter(self) -> Self::IntoIter {
        HexagonIterator::new(self)
    }
}

pub struct HexagonIterator {
    hexagon: Hexagon,
    ring_iterator: RingIterator,
    step: Distance,
}

impl HexagonIterator {
    pub fn new(hexagon: Hexagon) -> Self {
        Self {
            hexagon,
            ring_iterator: hexagon.ring(1).unwrap().into_iter(),
            step: 0,
        }
    }
}

impl Iterator for HexagonIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > self.hexagon.radius {
            return None;
        }

        if self.step == 0 {
            self.step = 1;
            return Some(self.hexagon.origin);
        }

        match self.ring_iterator.next() {
            None => {
                self.step += 1;

                if let Some(ring) = self.hexagon.ring(self.step) {
                    self.ring_iterator = ring.into_iter();
                }

                self.next()
            }
            some => some,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let hexagon = Hexagon::new(Position::new((3, -4, 1)).unwrap(), 3).unwrap();

        assert!(hexagon.contains(Position::new((3, -4, 1)).unwrap()));
        assert!(hexagon.contains(Position::new((3, -5, 2)).unwrap()));
        assert!(hexagon.contains(Position::new((3, -1, -2)).unwrap()));
        assert!(hexagon.contains(Position::new((2, -1, -1)).unwrap()));
        assert!(!hexagon.contains(Position::new((3, 0, -3)).unwrap()));
        assert!(!hexagon.contains(Position::new((2, 0, -2)).unwrap()));

        assert!(Ring::new(hexagon.origin, 3)
            .unwrap()
            .into_iter()
            .all(|position| hexagon.contains(position)));
        assert!(Ring::new(hexagon.origin, 4)
            .unwrap()
            .into_iter()
            .all(|position| !hexagon.contains(position)));
    }

    #[test]
    fn segment() {
        let hexagon = Hexagon::new(Position::new((3, -4, 1)).unwrap(), 3).unwrap();

        let segment = hexagon.segment(0, Direction::XY).unwrap();
        assert_eq!((0, -1, 1), segment.start().into());
        assert_eq!((6, -7, 1), segment.end().into());
        assert_eq!(segment.length(), 7);

        let segment = hexagon.segment(3, Direction::ZY).unwrap();
        assert_eq!((6, -4, -2), segment.start().into());
        assert_eq!((6, -7, 1), segment.end().into());
        assert_eq!(segment.length(), 4);

        let segment = hexagon.segment(-3, Direction::ZY).unwrap();
        assert_eq!((0, -1, 1), segment.start().into());
        assert_eq!((0, -4, 4), segment.end().into());
        assert_eq!(segment.length(), 4);
    }

    #[test]
    fn segments() {
        let hexagon = Hexagon::new(Position::new((3, -4, 1)).unwrap(), 3).unwrap();
        let mut iterator = hexagon.segments(Direction::XZ);

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(-3, distance);
        assert_eq!((3, -7, 4), segment.start().into());
        assert_eq!((6, -7, 1), segment.end().into());
        assert_eq!(segment.length(), 4);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(-2, distance);
        assert_eq!((2, -6, 4), segment.start().into());
        assert_eq!((6, -6, 0), segment.end().into());
        assert_eq!(segment.length(), 5);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(-1, distance);
        assert_eq!((1, -5, 4), segment.start().into());
        assert_eq!((6, -5, -1), segment.end().into());
        assert_eq!(segment.length(), 6);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(0, distance);
        assert_eq!((0, -4, 4), segment.start().into());
        assert_eq!((6, -4, -2), segment.end().into());
        assert_eq!(segment.length(), 7);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(1, distance);
        assert_eq!((0, -3, 3), segment.start().into());
        assert_eq!((5, -3, -2), segment.end().into());
        assert_eq!(segment.length(), 6);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(2, distance);
        assert_eq!((0, -2, 2), segment.start().into());
        assert_eq!((4, -2, -2), segment.end().into());
        assert_eq!(segment.length(), 5);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        let (distance, segment) = iterator.next().unwrap();
        assert_eq!(3, distance);
        assert_eq!((0, -1, 1), segment.start().into());
        assert_eq!((3, -1, -2), segment.end().into());
        assert_eq!(segment.length(), 4);
        assert!(segment
            .into_iter()
            .all(|position| hexagon.contains(position)));

        assert!(iterator.next().is_none());
    }

    #[test]
    fn iterator() {
        let hexagon = Hexagon::new(Position::new((3, -4, 1)).unwrap(), 2).unwrap();
        let mut iterator = hexagon.into_iter();

        assert_eq!((3, -4, 1), iterator.next().unwrap().into());

        assert_eq!((4, -5, 1), iterator.next().unwrap().into());
        assert_eq!((4, -4, 0), iterator.next().unwrap().into());
        assert_eq!((3, -3, 0), iterator.next().unwrap().into());
        assert_eq!((2, -3, 1), iterator.next().unwrap().into());
        assert_eq!((2, -4, 2), iterator.next().unwrap().into());
        assert_eq!((3, -5, 2), iterator.next().unwrap().into());

        assert_eq!((5, -6, 1), iterator.next().unwrap().into());
        assert_eq!((5, -5, 0), iterator.next().unwrap().into());
        assert_eq!((5, -4, -1), iterator.next().unwrap().into());
        assert_eq!((4, -3, -1), iterator.next().unwrap().into());
        assert_eq!((3, -2, -1), iterator.next().unwrap().into());
        assert_eq!((2, -2, 0), iterator.next().unwrap().into());
        assert_eq!((1, -2, 1), iterator.next().unwrap().into());
        assert_eq!((1, -3, 2), iterator.next().unwrap().into());
        assert_eq!((1, -4, 3), iterator.next().unwrap().into());
        assert_eq!((2, -5, 3), iterator.next().unwrap().into());
        assert_eq!((3, -6, 3), iterator.next().unwrap().into());
        assert_eq!((4, -6, 2), iterator.next().unwrap().into());

        assert!(iterator.next().is_none());

        assert!(hexagon
            .into_iter()
            .all(|position| hexagon.contains(position)));
    }
}
