use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Point2D<T, U = T>(pub T, pub U);

impl<T, U> From<(T, U)> for Point2D<T, U> {
    fn from((t, u): (T, U)) -> Self {
        Self(t, u)
    }
}

impl<T: Debug, U: Debug> Debug for Point2D<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("P").field(&self.0).field(&self.1).finish()
    }
}

impl<T: Copy, U: Copy> Point2D<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> U {
        self.1
    }
}

impl<T, U> Add for Point2D<T, U>
where
    T: Add<Output = T>,
    U: Add<Output = U>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct Point3D<T, U = T, V = T>(pub T, pub U, pub V);

impl<T, U, V> From<(T, U, V)> for Point3D<T, U, V> {
    fn from((t, u, v): (T, U, V)) -> Self {
        Self(t, u, v)
    }
}

pub trait ManhattanDistance {
    type Output;
    fn manhattan_distance(self, other: Self) -> Self::Output;
}

macro_rules! impl_manhattan_distance_2d {
    ($($ty:ty),+) => {
        $(
        impl ManhattanDistance for Point2D<$ty> {
            type Output = $ty;
            fn manhattan_distance(self, other: Self) -> Self::Output {
                self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
            }
        })+
    }
}

impl_manhattan_distance_2d!(u32, usize);

pub trait Neighbours {
    fn neighbours_direct_bounded(
        self,
        lower_bound: Self,
        upper_bound: Self,
    ) -> impl Iterator<Item = Self>;

    fn neighbours_diagonal_bounded(
        self,
        lower_bound: Self,
        upper_bound: Self,
    ) -> impl Iterator<Item = Self>;

    fn neighbours_direct_upper_bounded(self, upper_bound: Self) -> impl Iterator<Item = Self>
    where
        Self: Default,
    {
        self.neighbours_direct_bounded(Default::default(), upper_bound)
    }

    fn neighbours_diagonal_upper_bounded(self, upper_bound: Self) -> impl Iterator<Item = Self>
    where
        Self: Default,
    {
        self.neighbours_diagonal_bounded(Default::default(), upper_bound)
    }
}

macro_rules! impl_neighbours_2d_unsigned {
    ($($ty:ty),+) => {
        $(impl Neighbours for Point2D<$ty> {
            fn neighbours_direct_bounded(self, Self(x_min, y_min): Self, Self(x_max, y_max): Self) -> impl Iterator<Item = Self> {
                let Self(x, y) = self;

                core::iter::empty()
                    .chain((x > x_min).then(|| Self(x - 1, y)))
                    .chain((y > y_min).then(|| Self(x, y - 1)))
                    .chain((x < x_max - 1).then(|| Self(x + 1, y)))
                    .chain((y < y_max - 1).then(|| Self(x, y + 1)))
            }

            fn neighbours_diagonal_bounded(self, min @ Self(x_min, y_min): Self, max @ Self(x_max, y_max): Self) -> impl Iterator<Item = Self> {
                let Self(x, y) = self;

                self.neighbours_direct_bounded(min, max)
                    .chain((x > x_min && y > y_min).then(|| Self(x - 1, y - 1)))
                    .chain((x > x_min && y < y_max - 1).then(|| Self(x - 1, y + 1)))
                    .chain((x < x_max - 1 && y > y_min).then(|| Self(x + 1, y - 1)))
                    .chain((x < x_max - 1 && y < y_max - 1).then(|| Self(x + 1, y + 1)))
            }
        })+
    }
}

impl_neighbours_2d_unsigned!(u32, usize);

macro_rules! impl_neighbours_3d_unsigned {
    ($($ty:ty),+) => {
        $(impl Neighbours for Point3D<$ty> {
            fn neighbours_direct_bounded(self, Self(x_min, y_min, z_min): Self, Self(x_max, y_max, z_max): Self) -> impl Iterator<Item = Self> {
                let Self(x, y, z) = self;

                core::iter::empty()
                    .chain((x > x_min).then(|| Self(x - 1, y, z)))
                    .chain((y > y_min).then(|| Self(x, y - 1, z)))
                    .chain((z > z_min).then(|| Self(x, y, z - 1)))
                    .chain((x < x_max - 1).then(|| Self(x + 1, y, z)))
                    .chain((y < y_max - 1).then(|| Self(x, y + 1, z)))
                    .chain((z < z_max - 1).then(|| Self(x, y, z + 1)))
            }

            fn neighbours_diagonal_bounded(self, min @ Self(x_min, y_min, z_min): Self, max @ Self(x_max, y_max, z_max): Self) -> impl Iterator<Item = Self> {
                let Self(x, y, z) = self;

                self.neighbours_direct_bounded(min, max)
                    .chain((y > y_min && z > z_min).then(|| Self(x, y - 1, z - 1)))
                    .chain((y > y_min && z < z_max - 1).then(|| Self(x, y - 1, z + 1)))
                    .chain((y < y_max - 1 && z > z_min).then(|| Self(x, y + 1, z - 1)))
                    .chain((y < y_max - 1 && z < z_max - 1).then(|| Self(x, y + 1, z + 1)))

                    .chain((x > x_min && z > z_min).then(|| Self(x - 1, y, z - 1)))
                    .chain((x > x_min && z < z_max - 1).then(|| Self(x - 1, y, z + 1)))
                    .chain((x < x_max - 1 && z > z_min).then(|| Self(x + 1, y, z - 1)))
                    .chain((x < x_max - 1 && z < z_max - 1).then(|| Self(x + 1, y, z + 1)))

                    .chain((x > x_min && y > y_min).then(|| Self(x - 1, y - 1, z)))
                    .chain((x > x_min && y < y_max - 1).then(|| Self(x - 1, y + 1, z)))
                    .chain((x < x_max - 1 && y > y_min).then(|| Self(x + 1, y - 1, z)))
                    .chain((x < x_max - 1 && y < y_max - 1).then(|| Self(x + 1, y + 1, z)))

                    .chain((x > x_min && y > y_min && z > z_min).then(|| Self(x - 1, y - 1, z - 1)))
                    .chain((x > x_min && y > y_min && z < z_max - 1).then(|| Self(x - 1, y - 1, z + 1)))
                    .chain((x > x_min && y < y_max - 1 && z > z_min).then(|| Self(x - 1, y + 1, z - 1)))
                    .chain((x > x_min && y < y_max - 1 && z < z_max - 1).then(|| Self(x - 1, y + 1, z + 1)))

                    .chain((x < x_max - 1 && y > y_min && z > z_min).then(|| Self(x + 1, y - 1, z - 1)))
                    .chain((x < x_max - 1 && y > y_min && z < z_max - 1).then(|| Self(x + 1, y - 1, z + 1)))
                    .chain((x < x_max - 1 && y < y_max - 1 && z > z_min).then(|| Self(x + 1, y + 1, z - 1)))
                    .chain((x < x_max - 1 && y < y_max - 1 && z < z_max - 1).then(|| Self(x + 1, y + 1, z + 1)))
            }
        })+
    }
}

impl_neighbours_3d_unsigned!(u32);

impl<T: Copy, U: Copy, V: Copy> Point3D<T, U, V> {
    pub fn new(x: T, y: U, z: V) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> U {
        self.1
    }

    pub fn z(&self) -> V {
        self.2
    }
}

impl<T, U, V> Add for Point3D<T, U, V>
where
    T: Add<Output = T>,
    U: Add<Output = U>,
    V: Add<Output = V>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
