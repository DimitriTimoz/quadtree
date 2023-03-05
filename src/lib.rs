use std::fmt::Debug;

pub mod coordinate;
pub mod box2d;
pub mod prelude;
pub mod quadtree;

pub trait Number: std::ops::Sub<Output = Self> + std::ops::Add<Output = Self> + std::ops::Div<Output = Self> + std::cmp::PartialOrd + Copy + std::convert::From<u8> + Debug {}

impl Number for f32 {}

impl Number for f64 {}

impl Number for i16 {}

impl Number for i32 {}

impl Number for i64 {}

impl Number for i128 {}

impl Number for isize {}

impl Number for u8 {}

impl Number for u16 {}

impl Number for u32 {}

impl Number for u64 {}

impl Number for u128 {}

impl Number for usize {}


