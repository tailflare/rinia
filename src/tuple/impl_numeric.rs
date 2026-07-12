use crate::{
    numeric::{Abs, Bounded, Infinite, IsFinite, IsInfinite, IsNan, MinMax, Nan},
    tuple::Tuple,
};

// Bounded inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + Bounded,
{
    /// Returns a new tuple with all elements set to the minimum value of type `T`.
    pub const MIN: Self = Self { inner: [T::MIN; N] };

    /// Returns a new tuple with all elements set to the maximum value of type `T`.
    pub const MAX: Self = Self { inner: [T::MAX; N] };
}

// Bounded trait
impl<T, const N: usize> Bounded for Tuple<T, N>
where
    T: Copy + Bounded,
{
    const MIN: Self = Tuple::MIN;

    const MAX: Self = Tuple::MAX;
}

// MinMax inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + MinMax,
{
    /// Returns a new tuple containing the minimum values of each corresponding element in `self` and `rhs`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Tuple::from_array(core::array::from_fn(|i| self.inner[i].minimum(rhs.inner[i])))
    }

    /// Returns a new tuple containing the maximum values of each corresponding element in `self` and `rhs`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Tuple::from_array(core::array::from_fn(|i| self.inner[i].maximum(rhs.inner[i])))
    }
}

// MinMax trait
impl<T, const N: usize> MinMax for Tuple<T, N>
where
    T: Copy + MinMax,
{
    #[inline]
    fn minimum(self, rhs: Self) -> Self {
        Tuple::min(self, rhs)
    }

    #[inline]
    fn maximum(self, rhs: Self) -> Self {
        Tuple::max(self, rhs)
    }
}

// Abs inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + Abs,
{
    /// Returns a new tuple containing the absolute values of each element in `self`.
    #[inline]
    pub fn abs(self) -> Self {
        self.map(|x| x.abs())
    }
}

// Abs trait
impl<T, const N: usize> Abs for Tuple<T, N>
where
    T: Copy + Abs,
{
    #[inline]
    fn abs(self) -> Self {
        Tuple::abs(self)
    }
}

// Finite inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + IsFinite,
{
    /// Returns true if all elements in the tuple are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.inner.iter().all(|&x| x.is_finite())
    }
}

// Finite trait
impl<T, const N: usize> IsFinite for Tuple<T, N>
where
    T: Copy + IsFinite,
{
    #[inline]
    fn is_finite(self) -> bool {
        Tuple::is_finite(self)
    }
}

// Infinite inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + Infinite,
{
    /// Returns a new tuple with all elements set to positive infinity.
    pub const INFINITY: Self = Self { inner: [T::INFINITY; N] };

    /// Returns a new tuple with all elements set to negative infinity.
    pub const NEG_INFINITY: Self = Self { inner: [T::NEG_INFINITY; N] };
}

// IsInfinite inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + IsInfinite,
{
    /// Returns true if any element in the tuple is infinite.
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.inner.iter().any(|&x| x.is_infinite())
    }
}

// Infinite trait
impl<T, const N: usize> Infinite for Tuple<T, N>
where
    T: Copy + Infinite,
{
    const INFINITY: Self = Tuple::INFINITY;

    const NEG_INFINITY: Self = Tuple::NEG_INFINITY;
}

// IsInfinite trait
impl<T, const N: usize> IsInfinite for Tuple<T, N>
where
    T: Copy + IsInfinite,
{
    #[inline]
    fn is_infinite(self) -> bool {
        Tuple::is_infinite(self)
    }
}

// Nan inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + Nan,
{
    /// Returns a new tuple with all elements set to NaN.
    pub const NAN: Self = Self { inner: [T::NAN; N] };
}

// Nan trait
impl<T, const N: usize> Nan for Tuple<T, N>
where
    T: Copy + Nan,
{
    const NAN: Self = Tuple::NAN;
}

// IsNan inherent
impl<T, const N: usize> Tuple<T, N>
where
    T: Copy + IsNan,
{
    /// Returns true if any element in the tuple is NaN.
    #[inline]
    pub fn is_nan(self) -> bool {
        self.inner.iter().any(|&x| x.is_nan())
    }
}

// IsNan trait
impl<T, const N: usize> IsNan for Tuple<T, N>
where
    T: Copy + IsNan,
{
    #[inline]
    fn is_nan(self) -> bool {
        Tuple::is_nan(self)
    }
}
