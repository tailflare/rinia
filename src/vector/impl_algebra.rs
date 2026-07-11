use core::ops::{Add, Div, Mul, Sub};

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel, Dot, Length, LengthSquared, Lerp, Normalize},
    numeric::{One, Sqrt, Zero},
    vector::Vector,
};

// Zero
impl<T, const N: usize> Vector<T, N>
where
    T: Zero,
{
    pub const ZERO: Self = Self::from_array([T::ZERO; N]);
}

impl<T, const N: usize> Zero for Vector<T, N>
where
    T: Zero,
{
    const ZERO: Self = Vector::ZERO;
}

// One
impl<T, const N: usize> Vector<T, N>
where
    T: One,
{
    pub const ONE: Self = Self::from_array([T::ONE; N]);
}

impl<T, const N: usize> One for Vector<T, N>
where
    T: One,
{
    const ONE: Self = Vector::ONE;
}

// Approx eq abs inherent
impl<T, const N: usize> Vector<T, N>
where
    T: ApproxEqAbs<Tolerance = T> + Copy,
{
    /// Default absolute tolerance for approximate equality checks.
    const DEFAULT_TOLERANCE_ABS: T = T::DEFAULT_TOLERANCE_ABS;

    /// Checks if two vectors are approximately equal within a given absolute tolerance.
    #[inline]
    pub fn approx_eq_abs_tol(self, rhs: Self, tolerance: T) -> bool {
        self.into_tuple().approx_eq_abs_tol(rhs.into_tuple(), tolerance)
    }

    /// Checks if two vectors are approximately equal within a given default absolute tolerance.
    #[inline]
    pub fn approx_eq_abs(self, rhs: Self) -> bool {
        self.approx_eq_abs_tol(rhs, Self::DEFAULT_TOLERANCE_ABS)
    }
}

// Approx eq abs trait
impl<T, const N: usize> ApproxEqAbs for Vector<T, N>
where
    T: ApproxEqAbs<Tolerance = T> + Copy,
{
    type Tolerance = T;

    const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

    #[inline]
    fn approx_eq_abs_tol(self, other: Self, tol: Self::Tolerance) -> bool {
        Vector::approx_eq_abs_tol(self, other, tol)
    }
}

// Approx eq rel inherent
impl<T, const N: usize> Vector<T, N>
where
    T: ApproxEqRel<Tolerance = T> + Copy,
{
    /// Default relative tolerance for approximate equality checks.
    const DEFAULT_TOLERANCE_REL: T = T::DEFAULT_TOLERANCE_REL;

    /// Checks if two vectors are approximately equal within a given relative tolerance.
    #[inline]
    pub fn approx_eq_rel_tol(self, rhs: Self, tolerance: T) -> bool {
        self.into_tuple().approx_eq_rel_tol(rhs.into_tuple(), tolerance)
    }

    /// Checks if two vectors are approximately equal within a given default relative tolerance.
    #[inline]
    pub fn approx_eq_rel(self, rhs: Self) -> bool {
        self.approx_eq_rel_tol(rhs, Self::DEFAULT_TOLERANCE_REL)
    }
}

// Approx eq rel trait
impl<T, const N: usize> ApproxEqRel for Vector<T, N>
where
    T: ApproxEqRel<Tolerance = T> + Copy,
{
    type Tolerance = T;
    const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

    #[inline]
    fn approx_eq_rel_tol(self, other: Self, tol: Self::Tolerance) -> bool {
        Vector::approx_eq_rel_tol(self, other, tol)
    }
}
// Length and length squared inherent
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
