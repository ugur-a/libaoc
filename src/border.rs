use crate::{map::Map2D, points::Point2D};
use itertools::Itertools;
use num_traits::PrimInt;

#[derive(Debug)]
pub struct Border2D<T, U = T> {
    pub left: T,
    pub right: T,
    pub top: U,
    pub down: U,
}

impl<T, U> Border2D<T, U>
where
    T: PrimInt,
    U: PrimInt,
{
    /// ``` rust
    /// use libaoc::{border::Border2D, points::Point2D};
    ///
    /// let ps = [Point2D(1,3), Point2D(2,2)];
    /// let b = Border2D::from_points(ps);
    ///
    /// assert_eq!(b, Border2D {left: 1, right: 2, top: 2, down: 3});
    /// ```
    pub fn from_points<const N: usize>(points: [Point2D<T, U>; N]) -> Self {
        match N {
            0 => Self {
                left: T::min_value(),
                right: T::max_value(),
                top: U::min_value(),
                down: U::max_value(),
            },
            1 => Self::from_one_point(points[0]),
            2 => Self::from_two_points(points[0], points[1]),
            _ => Self::from_many_points(points),
        }
    }

    pub fn from_one_point(point: Point2D<T, U>) -> Self {
        let Point2D(x, y) = point;

        Self {
            left: T::zero(),
            right: x,
            top: U::zero(),
            down: y,
        }
    }

    pub fn from_two_points(p1: Point2D<T, U>, p2: Point2D<T, U>) -> Self {
        let Point2D(x1, y1) = p1;
        let Point2D(x2, y2) = p2;

        let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        Self {
            left: x_min,
            right: x_max,
            top: y_min,
            down: y_max,
        }
    }

    pub fn from_many_points<const N: usize>(points: [Point2D<T, U>; N]) -> Self {
        min_enclosing_rectangle(points.iter(), points.iter())
    }
}

impl<T> From<Map2D<T>> for Border2D<usize> {
    fn from(map: Map2D<T>) -> Self {
        let lower_right = Point2D(map.width() - 1, map.height() - 1);
        Self::from_one_point(lower_right)
    }
}

pub fn min_enclosing_rectangle<'a, I, T, U>(positions1: I, positions2: I) -> Border2D<T, U>
where
    T: Copy + PartialOrd + 'a,
    U: Copy + PartialOrd + 'a,
    I: Iterator<Item = &'a Point2D<T, U>>,
{
    let (left, right) = positions1.map(Point2D::x).minmax().into_option().unwrap();
    let (top, down) = positions2.map(Point2D::y).minmax().into_option().unwrap();

    Border2D {
        left,
        right,
        top,
        down,
    }
}
