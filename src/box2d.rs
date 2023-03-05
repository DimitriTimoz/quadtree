use crate::prelude::*;

pub struct Box2d<U> {
    center: Point<U>,
    half_size: U,
}

impl<U> Box2d<U> 
where 
    U: std::ops::Sub<Output = U> + std::ops::Add<Output = U> + std::cmp::PartialOrd + Copy
{
    pub fn new(center: Point<U>, half_size: U) -> Box2d<U> {
        Box2d { center, half_size }
    }

    pub fn contains(&self, point: &Point<U>) -> bool {
        let half_size = self.half_size;
        let center = &self.center;
        point.x >= center.x - half_size && point.x <= center.x + half_size
            && point.y >= center.y - half_size && point.y <= center.y + half_size
    }
}