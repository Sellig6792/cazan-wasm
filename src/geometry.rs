use num_integer::Roots;
use std::cmp::{Ordering, PartialEq};
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Triangle(pub Point, pub Point, pub Point);

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let mut self_points = vec![self.0, self.1, self.2];
        let mut other_points = vec![other.0, other.1, other.2];
        self_points.sort();
        other_points.sort();
        self_points == other_points
    }
}

pub type Polygon = Vec<Point>;

#[wasm_bindgen]
pub fn distance(a: Point, b: Point) -> u16 {
    let dx = (a.x as i32 - b.x as i32).unsigned_abs() as u16;
    let dy = (a.y as i32 - b.y as i32).unsigned_abs() as u16;
    (dx * dx + dy * dy).sqrt()
}

pub(crate) fn cross_product(a: Point, b: Point, c: Point) -> i32 {
    (b.x as i32 - a.x as i32) * (c.y as i32 - b.y as i32)
        - (b.y as i32 - a.y as i32) * (c.x as i32 - b.x as i32)
}

pub(crate) fn is_point_inside_triangle(triangle: &Triangle, point: Point) -> bool {
    let a = triangle.0;
    let b = triangle.1;
    let c = triangle.2;

    matches!(
        (
            cross_product(a, b, point).cmp(&0),
            cross_product(b, c, point).cmp(&0),
            cross_product(c, a, point).cmp(&0),
        ),
        (Ordering::Less, Ordering::Less, Ordering::Less)
            | (Ordering::Greater, Ordering::Greater, Ordering::Greater)
    )
}

pub(crate) fn is_convex(polygon: &Polygon) -> bool {
    let mut sign = 0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        let c = polygon[(i + 2) % polygon.len()];
        let cross = cross_product(a, b, c);
        if cross == 0 {
            continue;
        }
        if sign == 0 {
            sign = cross;
        } else if sign * cross < 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(distance(a, b), 5);
    }

    #[test]
    fn test_cross_product() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 3, y: 4 };
        let c = Point { x: 4, y: 3 };
        assert_eq!(cross_product(a, b, c), -7);
    }

    #[test]
    fn test_is_point_inside_triangle_on_side() {
        let triangle = Triangle(
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        );
        let point = Point { x: 2, y: 2 };
        assert!(is_point_inside_triangle(&triangle, point));
    }

    #[test]
    fn test_is_point_inside_triangle() {
        let triangle = Triangle(
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        );
        let point = Point { x: 2, y: 1 };
        assert!(is_point_inside_triangle(&triangle, point));
    }

    #[test]
    fn test_is_point_outside_triangle() {
        let triangle = Triangle(
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        );
        let point = Point { x: 2, y: 3 };
        assert!(!is_point_inside_triangle(&triangle, point));
    }

    #[test]
    fn test_is_convex() {
        let polygon: Polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 1 },
            Point { x: 3, y: 0 },
        ];
        assert!(is_convex(&polygon));
    }

    #[test]
    fn test_is_not_convex() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
            Point { x: 3, y: 2 },
        ];
        assert!(!is_convex(&polygon));
    }
}
