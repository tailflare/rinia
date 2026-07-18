use core::ops::{Add, Div, Mul, Sub};

use crate::{
    algebra::{ApproxEqAbs, Dot, IsNormalized, Length, LengthSquared, Lerp, Normalize},
    numeric::{One, Sqrt},
    vector::Vector,
};

crate::impl_approx_eq_wrapper!(
    [T, const N: usize],
    impl: Vector<T, N>,
    item: T,
    field: data,
);

// Length, length squared and normalize inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    /// Computes the squared length of the vector.
    #[inline]
    pub fn length_squared(self) -> T {
        self.into_tuple().length_squared()
    }

    /// Computes the length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> T
    where
        T: Sqrt,
    {
        self.length_squared().sqrt()
    }

    /// Returns a normalized vector with length one.
    #[inline]
    pub fn normalize(self) -> Self
    where
        T: Sqrt + Div<Output = T>,
    {
        self / self.length()
    }

    /// Checks if the vector is normalized.
    #[inline]
    pub fn is_normalized(self) -> bool
    where
        T: One + ApproxEqAbs<Tolerance = T>,
    {
        self.length_squared().approx_eq_abs(&T::ONE)
    }

    /// Checks if the vector is normalized within a given tolerance.
    #[inline]
    pub fn is_normalized_tol(self, tol: T) -> bool
    where
        T: One + ApproxEqAbs<Tolerance = T>,
    {
        self.length_squared().approx_eq_abs_tol(&T::ONE, tol)
    }
}

// Length squared trait
impl<T, const N: usize> LengthSquared for Vector<T, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    #[inline]
    fn length_squared(self) -> Self::Output {
        Vector::length_squared(self)
    }
}

// Length trait
impl<T, const N: usize> Length for Vector<T, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Sqrt,
{
    #[inline]
    fn length(self) -> Self::Output {
        Vector::length(self)
    }
}

// Normalize trait
impl<T, const N: usize> Normalize for Vector<T, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Sqrt,
{
    #[inline]
    fn normalize(self) -> Self {
        Vector::normalize(self)
    }
}

// IsNormalized trait
impl<T, const N: usize> IsNormalized for Vector<T, N>
where
    T: Copy + One + Mul<Output = T> + Add<Output = T> + ApproxEqAbs<Tolerance = T> + Sqrt,
{
    type Tolerance = T;

    #[inline]
    fn is_normalized(self) -> bool {
        Vector::is_normalized(self)
    }

    #[inline]
    fn is_normalized_tol(self, tol: Self::Tolerance) -> bool {
        Vector::is_normalized_tol(self, tol)
    }
}

// Dot inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(self, rhs: Self) -> T {
        self.into_tuple().dot(rhs.into_tuple())
    }
}

// Dot trait
impl<T, const N: usize> Dot for Vector<T, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    type Output = T;

    #[inline]
    fn dot(self, rhs: Self) -> Self::Output {
        Vector::dot(self, rhs)
    }
}

// Lerp inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    /// Performs linear interpolation between two vectors.
    #[inline]
    fn lerp(self, rhs: Self, t: T) -> Self {
        self + (rhs - self) * t
    }
}

// Lerp trait
impl<T, const N: usize> Lerp for Vector<T, N>
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    type Scalar = T;

    #[inline]
    fn lerp(self, rhs: Self, t: Self::Scalar) -> Self {
        Vector::lerp(self, rhs, t)
    }
}
