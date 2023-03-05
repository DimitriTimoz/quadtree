use quadtree::prelude::*;

#[test]
fn test_box() {
    let box2d = Box2d::new(Point::new(0.0, 0.0), 1.0);
    assert!(box2d.contains(&Point::new(0.0, 0.0)));
    assert!(box2d.contains(&Point::new(0.5, 0.5)));
    assert!(box2d.contains(&Point::new(-0.5, -0.5)));
    assert!(!box2d.contains(&Point::new(1.5, 1.5)));
    assert!(!box2d.contains(&Point::new(-1.5, -1.5)));
}