use crate::{prelude::*, Number};

#[derive(Clone, Debug)]
pub struct Box2d<U>
where U : Number
{
    center: Point<U>,
    half_size: U,
}

impl<U> Box2d<U> 
where U : Number
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

    pub fn intersects_box(&self, other: &Box2d<U>) -> bool {
        let half_size = self.half_size;
        let center = &self.center;
        let other_half_size = other.half_size;
        let other_center = &other.center;
        center.x - half_size <= other_center.x + other_half_size
            && center.x + half_size >= other_center.x - other_half_size
            && center.y - half_size <= other_center.y + other_half_size
            && center.y + half_size >= other_center.y - other_half_size
    }

    /// Subdivide the current box into 4 new boxes north west, north east, south east, south west
    pub(crate) fn subdivide(&self) -> Vec<Box2d<U>> {
        let half_size = self.half_size / U::from(2);
        let center = &self.center;
        vec![
            Box2d::new(Point::new(center.x - half_size, center.y + half_size), half_size),
            Box2d::new(Point::new(center.x + half_size, center.y + half_size), half_size),
            Box2d::new(Point::new(center.x + half_size, center.y - half_size), half_size),
            Box2d::new(Point::new(center.x - half_size, center.y - half_size), half_size),
        ]
    }

}