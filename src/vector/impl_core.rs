use crate::{
    common,
    numeric::Zero,
    scalar::{HasScalar, Scalar},
    vector::Vector,
};

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
common::impl_bytemuck_basic!([T, const N: usize], Vector<T, N>, item: T);

// Blanket impl for HasScalar for Vector<T, N> where T implements Scalar
impl<T, const N: usize> HasScalar for Vector<T, N>
where
    T: Scalar,
{
    type Scalar = T;
}
