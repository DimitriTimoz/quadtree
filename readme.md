# Quad3

[![Crates.io](https://img.shields.io/crates/v/quad3.svg)](https://crates.io/crates/quad3)
[![Docs.rs](https://docs.rs/quad3/badge.svg)](https://docs.rs/quad3)

A quadtree implementation in rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
quad3 = "0.1"
```


## Example

```rust
use quad3::prelude::*;
// Create a new quadtree with a bounding box of 20x20 centred in (0, 0) and a max amount of 4 items per node
let mut quadtree = QuadTree::new(Box2d::new(Point::new(0.0, 0.0), 10.0), 4);

assert!(quadtree.insert(&Point::new(0.0, 0.0)));
assert!(quadtree.insert(&Point::new(0.5, 0.5)));
assert!(quadtree.insert(&Point::new(-0.5, -0.5)));
assert!(quadtree.insert(&Point::new(1.5, 1.5)));
assert!(quadtree.insert(&Point::new(-1.5, -1.5)));
assert!(quadtree.insert(&Point::new(2.5, 2.5)));
assert!(quadtree.insert(&Point::new(-2.5, -2.5)));

// Query a range of 2x2 centred in (0, 0)
let points = quadtree.query_range(Box2d::new(Point::new(0.0, 0.0), 1.0));
assert_eq!(points.len(), 3);
```