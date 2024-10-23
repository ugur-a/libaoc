use crate::points::Point2D;
use itertools::Itertools;

#[derive(Debug)]
pub struct Border2D<T, U = T> {
    pub left: T,
    pub right: T,
    pub top: U,
    pub down: U,
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
