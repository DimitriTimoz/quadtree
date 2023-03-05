use quadtree::prelude::*;

#[test]
fn test_quadtree_create() {
    let mut quadtree = QuadTree::new(Box2d::new(Point::new(0.0, 0.0), 10.0), 4);
    assert!(quadtree.insert(&Point::new(0.0, 0.0)));
    assert!(quadtree.insert(&Point::new(0.5, 0.5)));
    assert!(quadtree.insert(&Point::new(-0.5, -0.5)));
    assert!(quadtree.insert(&Point::new(1.5, 1.5)));
    assert!(quadtree.insert(&Point::new(-1.5, -1.5)));
    assert!(quadtree.insert(&Point::new(2.5, 2.5)));
    assert!(quadtree.insert(&Point::new(-2.5, -2.5)));
}

#[test]
fn test_quadtree_query() {
    let mut quadtree = QuadTree::new(Box2d::new(Point::new(0.0, 0.0), 10.0), 4);
    assert!(quadtree.insert(&Point::new(0.0, 0.0)));
    assert!(quadtree.insert(&Point::new(0.5, 0.5)));
    assert!(quadtree.insert(&Point::new(-0.5, -0.5)));
    assert!(quadtree.insert(&Point::new(1.5, 1.5)));
    assert!(quadtree.insert(&Point::new(-1.5, -1.5)));
    assert!(quadtree.insert(&Point::new(2.5, 2.5)));
    assert!(quadtree.insert(&Point::new(-2.5, -2.5)));

    let points = quadtree.query_range(Box2d::new(Point::new(0.0, 0.0), 1.0));
    assert_eq!(points.len(), 3);

}