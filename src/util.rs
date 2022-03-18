
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<usize> {
    pub fn shift(self, direction: Direction) -> Option<Point<usize>> {
        Some(match direction {
            Direction::Up => Point { x: self.x, y: self.y.checked_sub(1)? },
            Direction::Down => Point { x: self.x, y: self.y.checked_add(1)? },
            Direction::Left => Point { x: self.x.checked_sub(1)?, y: self.y },
            Direction::Right => Point { x: self.x.checked_add(1)?, y: self.y },
        })
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(Point { x, y }: Point<T>) -> Self {
        (x, y)
    }
}

#[derive(Clone, Debug)]
pub struct Region<T> {
    tl: Point<T>,
    br: Point<T>,
}

impl Region<usize> {
    pub(crate) fn new(tl: Point<usize>, br: Point<usize>) -> Region<usize> {
        if !(tl.x <= br.x) || !(tl.y <= br.y) {
            panic!("Invalid Region: ({:?}, {:?})", tl, br);
        }
        Region { tl, br }
    }

    pub fn top_left(&self) -> Point<usize> {
        self.tl
    }

    pub fn top_right(&self) -> Point<usize> {
        self.br
    }
}
