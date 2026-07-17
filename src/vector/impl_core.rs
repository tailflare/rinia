use crate::{
    common,
    numeric::Zero,
    scalar::{HasScalar, Scalar},
    vector::Vector,
};

impl<T, const N: usize> Vector<T, N> {
    /// Returns the length of the [Vector].
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns true if the [Vector] is empty (i.e., has length 0).
    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

// Default impl for Vector<T, N>, which is Zero
impl<T, const N: usize> Default for Vector<T, N>
where
    Self: Zero,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Bytemuck trait implementations for Vector<T, N>
common::_impl_bytemuck_basic!([T, const N: usize], Vector<T, N>, item: T);

// Blanket impl for HasScalar for Vector<T, N> where T implements Scalar
impl<T, const N: usize> HasScalar for Vector<T, N>
where
    T: Scalar,
{
    type Scalar = T;
}
