pub mod three_d;
pub mod two_d;

pub use three_d::Point3D;
pub use two_d::Point2D;

pub trait ManhattanDistance {
    type Output;
    fn manhattan_distance(self, other: Self) -> Self::Output;
}

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
