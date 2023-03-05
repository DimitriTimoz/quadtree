use crate::{prelude::*, Number};

pub struct QuadTree<U>
where U : Number
{
    capacity: usize,
    boundary: Box2d<U>,
    points: Vec<Point<U>>,

    north_west: Option<Box<QuadTree<U>>>,
    north_east: Option<Box<QuadTree<U>>>,
    south_west: Option<Box<QuadTree<U>>>,
    south_east: Option<Box<QuadTree<U>>>,
}

impl<U> QuadTree<U> 
where U : Number
{
    pub fn new(boundary: Box2d<U>, capacity: usize) -> QuadTree<U> {
        QuadTree {
            capacity,
            boundary,
            points: Vec::new(),
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }
    
    pub fn insert(&mut self, point: &Point<U>) -> bool {
       
        // if point is not in boundary, return false
        if !self.boundary.contains(point) {
            return false;
        }

        // if there is space in the current quadtree, add the point and return true
        if self.points.len() < self.capacity && self.north_east.is_none() {
            self.points.push(point.clone());
            return true;
        }

        // if there is no space in the current quadtree, subdivide and try to insert the point
        if self.north_east.is_none() {
            self.subdivide();
        }

        // try to insert the point in the sub quadtree
        if self.north_west.as_mut().unwrap().insert(point) {
            return true;
        }
        if self.north_east.as_mut().unwrap().insert(point) {
            return true;
        }
        if self.south_west.as_mut().unwrap().insert(point) {
            return true;
        }
        if self.south_east.as_mut().unwrap().insert(point) {
            return true;
        }

        // Nothing can be done, return false
        false
    }

    fn subdivide(&mut self) {
        let sub_boxes = self.boundary.subdivide();
        
        self.north_west = Some(Box::new(QuadTree::new(sub_boxes[0].clone(), self.capacity)));
        self.north_east = Some(Box::new(QuadTree::new(sub_boxes[1].clone(), self.capacity)));
        self.south_east = Some(Box::new(QuadTree::new(sub_boxes[2].clone(), self.capacity)));
        self.south_west = Some(Box::new(QuadTree::new(sub_boxes[3].clone(), self.capacity)));
    }

    pub fn query_range(&self, range: Box2d<U>) -> Vec<Point<U>> {
        let mut points = Vec::new();
        
        if !self.boundary.intersects_box(&range) {
            return points;
        }

        for point in &self.points {
            if range.contains(point) {
                points.push(point.clone());
            }
        }

        if self.north_east.is_none() {
            return points;
        }

        // query the sub quadtree
        points.append(&mut self.north_west.as_ref().unwrap().query_range(range.clone()));
        points.append(&mut self.north_east.as_ref().unwrap().query_range(range.clone()));
        points.append(&mut self.south_west.as_ref().unwrap().query_range(range.clone()));
        points.append(&mut self.south_east.as_ref().unwrap().query_range(range));

        points
    }
}