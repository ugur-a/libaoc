use crate::points::Neighbours;
use core::ops::Add;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct Point3D<T, U = T, V = T>(pub T, pub U, pub V);

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
impl<T, U, V> From<(T, U, V)> for Point3D<T, U, V> {
    fn from((t, u, v): (T, U, V)) -> Self {
        Self(t, u, v)
    }
}

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
