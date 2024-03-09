use crate::position::{Direction, Distance, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Line {
    origin: Position,
    direction: Direction,
}

impl Line {
    pub fn new(origin: Position, direction: Direction) -> Self {
        Line { origin, direction }
    }

    pub fn normalized(origin: Position, direction: Direction) -> Self {
        Line::new(origin, direction).normalize()
    }

    pub fn position(&self, distance: Distance) -> Position {
        let direction: Position = self.direction.position();
        self.origin + (direction * distance)
    }

    pub fn origin(&self) -> Position {
        self.origin
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    // Transforms the origin position of the line such that its
    // value for the direction's positive coordinate is zero, and
    // normalizes the orientation of the direction of the line.
    // This means equality comparisons will compare whether the line
    // is equivalent (in that it refers to the same set of points on the
    // plane) regardless of whether it's been created from the same
    // direction and origin point.
    pub fn normalize(&self) -> Line {
        let direction = self.direction.normalize();
        let deviation = self.origin.axis(direction.positive_axis());
        let origin = self.origin - (direction.position() * deviation);

        Line { origin, direction }
    }
}

impl IntoIterator for Line {
    type Item = Position;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator::new(self)
    }
}

pub struct LineIterator {
    line: Line,
    distance: Distance,
    distance_back: Distance,
}

impl LineIterator {
    pub fn new(line: Line) -> Self {
        Self {
            line,
            distance: 0,
            distance_back: -1,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let distance = self.distance;
        self.distance += 1;

        Some(self.line.position(distance))
    }
}

impl DoubleEndedIterator for LineIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let distance_back = self.distance_back;
        self.distance_back -= 1;

        Some(self.line.position(distance_back))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let line = Line::new(Position::new((0, 1, -1)).unwrap(), Direction::XY);

        assert_eq!((0, 1, -1), line.position(0).into());
        assert_eq!((1, 0, -1), line.position(1).into());
        assert_eq!((-1, 2, -1), line.position(-1).into());
        assert_eq!((5, -4, -1), line.position(5).into());
        assert_eq!((-5, 6, -1), line.position(-5).into());
    }

    #[test]
    fn iterator() {
        let line = Line::new(Position::new((0, 1, -1)).unwrap(), Direction::XY);

        let mut iterator = line.into_iter();
        assert_eq!((0, 1, -1), iterator.next().unwrap().into());
        assert_eq!((1, 0, -1), iterator.next().unwrap().into());
        assert_eq!((-1, 2, -1), iterator.next_back().unwrap().into());
        assert_eq!((2, -1, -1), iterator.next().unwrap().into());
        assert_eq!((-2, 3, -1), iterator.next_back().unwrap().into());
    }

    #[test]
    fn normalize() {
        let line = Line::new(Position::new((-3, 4, -1)).unwrap(), Direction::ZY);

        assert_eq!((-3, 0, 3), line.normalize().position(0).into());
        assert_eq!(Direction::YZ, line.normalize().direction());
        assert_eq!(line.normalize().position(0), line.position(4));

        let other_line: Line = Line::new(Position::new((-3, -2, 5)).unwrap(), Direction::YZ);

        assert_eq!(line.normalize(), other_line.normalize());
    }
}
