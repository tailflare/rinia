use core::ops::{Add, Mul, Neg};

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel},
    tuple::Tuple,
};

impl<T, const N: usize> Tuple<T, N> {
    // Approx eq abs inherent
    pub(crate) fn approx_eq_abs_tol(self, rhs: Self, tolerance: T) -> bool
    where
        T: ApproxEqAbs<Tolerance = T> + Copy,
    {
        self.into_iter().zip(rhs).all(|(a, b)| a.approx_eq_abs_tol(b, tolerance))
    }

    // Approx eq rel inherent
    pub(crate) fn approx_eq_rel_tol(self, rhs: Self, tolerance: T) -> bool
    where
        T: ApproxEqRel<Tolerance = T> + Copy,
    {
        self.into_iter().zip(rhs).all(|(a, b)| a.approx_eq_rel_tol(b, tolerance))
    }

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
