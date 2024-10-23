use core::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;

use crate::{direction::Direction4, points::Point2D};

pub struct Map2D<T>(Vec<Vec<T>>);

impl<T> Map2D<T> {
    pub fn new(v: Vec<Vec<T>>) -> Self {
        Self(v)
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn rows(&self) -> &Vec<Vec<T>> {
        &self.0
    }
    pub fn rows_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.0
    }

    pub fn try_go(&self, point: Point2D<usize>, direction: Direction4) -> Option<Point2D<usize>> {
        let Point2D(x, y) = point;
        match direction {
            Direction4::Left => (x > 0).then(|| Point2D(x - 1, y)),
            Direction4::Up => (y > 0).then(|| Point2D(x, y - 1)),
            Direction4::Right => (x < self.width() - 1).then_some(Point2D(x + 1, y)),
            Direction4::Down => (y < self.height() - 1).then_some(Point2D(x, y + 1)),
        }
    }
}

/// ``` rust
/// use libaoc::map::Map2D;
/// use core::str::FromStr;
///
/// struct Foo;
///
/// impl TryFrom<char> for Foo {
///     type Error = ();
///     fn try_from(_: char) -> Result<Self, Self::Error> {
///         Ok(Self)
///     }
/// }
///
/// let v = "aaa\nbbb\nccc";
/// let m: Map2D<Foo> = Map2D::from_str(v).unwrap();
/// ```
impl<T> FromStr for Map2D<T>
where
    T: TryFrom<char>,
{
    type Err = <T as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|row| row.chars().map(T::try_from).try_collect())
            .try_collect()?;

        Ok(Self(map))
    }
}

/// ```rust
/// # use libaoc::{map::Map2D, points::Point2D};
/// let v = vec![
///           vec![1, 2, 3],
///           vec![4, 5, 6],
///           vec![7, 8, 9]
///         ];
/// let m = Map2D::new(v);
/// let pos: Point2D<usize> = Point2D(1, 2);
///
/// assert_eq!(m[pos], 8);
/// ```
impl<T, U> Index<Point2D<T>> for Map2D<U>
where
    T: Into<usize>,
{
    type Output = U;

    fn index(&self, index: Point2D<T>) -> &Self::Output {
        let Point2D(x, y) = index;
        &self.0[y.into()][x.into()]
    }
}

impl<T, U> IndexMut<Point2D<T>> for Map2D<U>
where
    T: Into<usize>,
{
    fn index_mut(&mut self, index: Point2D<T>) -> &mut Self::Output {
        let Point2D(x, y) = index;
        &mut self.0[y.into()][x.into()]
    }
}
