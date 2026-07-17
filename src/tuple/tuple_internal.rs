use core::ops::{Add, Mul, Neg};

use crate::tuple::Tuple;

impl<T, const N: usize> Tuple<T, N> {
    // Length squared inherent
    pub(crate) fn length_squared(self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.into_iter()
            .map(|x| x * x)
            .reduce(|a, b| a + b)
            .expect("length_squared requires non-empty tuple")
    }

    // Negate inherent
    pub(crate) fn negate(self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        self.map(|x| -x)
    }

    // Dot inherent
    pub(crate) fn dot(self, rhs: Self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.into_iter()
            .zip(rhs)
            .map(|(a, b)| a * b)
            .reduce(|a, b| a + b)
            .expect("dot product requires non-empty tuple")
    }
}

crate::impl_approx_eq_wrapper!(
    [T, const N: usize],
    impl: Tuple<T, N>,
    item: T,
    field: inner,
);
