pub struct Point<U> {
    pub x: U,
    pub y: U,
}

impl<U> Point<U> {
    pub fn new(x: U, y: U) -> Point<U> {
        Point { x, y }
    }
}
