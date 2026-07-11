use core::ops::Neg;

use crate::{
    common,
    numeric::{Abs, Bounded, MinMax, Negate},
    tuple::Tuple,
    vector::Vector,
};

// Bounded inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Bounded,
{
    /// Returns a new vector with all elements set to the minimum value of type `T`.
    pub const MIN: Self = Self::from_tuple(Tuple::MIN);

    /// Returns a new vector with all elements set to the maximum value of type `T`.
    pub const MAX: Self = Self::from_tuple(Tuple::MAX);
}

// Bounded trait
impl<T, const N: usize> Bounded for Vector<T, N>
where
    T: Copy + Bounded,
{
    const MIN: Self = Vector::MIN;
    const MAX: Self = Vector::MAX;
}

// MinMax inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + MinMax,
{
    /// Returns a new vector containing the minimum values of each corresponding element in `self` and `rhs`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Vector::from_tuple(self.into_tuple().min(rhs.into_tuple()))
    }

    /// Returns a new vector containing the maximum values of each corresponding element in `self` and `rhs`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Vector::from_tuple(self.into_tuple().max(rhs.into_tuple()))
    }
}

// MinMax trait
impl<T, const N: usize> MinMax for Vector<T, N>
where
    T: Copy + MinMax,
{
    #[inline]
    fn minimum(self, rhs: Self) -> Self {
        Vector::min(self, rhs)
    }

    #[inline]
    fn maximum(self, rhs: Self) -> Self {
        Vector::max(self, rhs)
    }
}

// Negate inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Neg<Output = T>,
{
    /// Negates the vector, returning a new vector with all components negated.
    #[inline]
    pub fn negate(self) -> Self {
        let out = self.into_tuple().negate();
        Self::from_tuple(out)
    }
}

// Negate trait
impl<T, const N: usize> Negate for Vector<T, N>
where
    T: Copy + Neg<Output = T>,
{
    #[inline]
    fn negate(self) -> Self::Output {
        Vector::negate(self)
    }
}

// Abs inherent
impl<T, const N: usize> Vector<T, N>
where
    T: Copy + Abs,
{
    /// Returns a new vector with all elements set to their absolute values.
    #[inline]
    pub fn abs(self) -> Self {
        Vector::from_tuple(self.into_tuple().abs())
    }
}

// Abs trait
impl<T, const N: usize> Abs for Vector<T, N>
where
    T: Copy + Abs,
{
    #[inline]
    fn abs(self) -> Self {
        Vector::abs(self)
    }
}

// Implement Finite/Infinite/Nan for Vector<T, N>.
common::impl_tuple_wrapper_numeric_predicates!([T, const N: usize], Vector<T, N>, item: T, len: N);
