use crate::{GRID_X_SIZE, GRID_Y_SIZE};

use std::cmp::Ordering;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Hash)]
pub struct Point(pub i32, pub i32);

impl Add for Point {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0 + other.0 < 0
            || self.0 + other.0 >= GRID_X_SIZE as i32
            || self.1 + other.1 < 0
            || self.1 + other.1 >= GRID_Y_SIZE as i32
        {
            None
        } else {
            Some(Self(self.0 + other.0, self.1 + other.1))
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).then(self.0.cmp(&other.0))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}
