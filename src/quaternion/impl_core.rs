use crate::{
    algebra::Identity,
    common,
    quaternion::Quaternion,
    scalar::{HasScalar, Scalar},
};

// Inherent impls for Quaternion<T>
impl<T> Quaternion<T> {
    /// Creates a new quaternion with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from_array([x, y, z, w])
    }

    /// Returns the length of the [Quaternion].
    /// This is always 4 for [Quaternion] since it always has 4 components.
    #[inline]
    pub const fn len(&self) -> usize {
        4
    }

    /// Returns true if the [Quaternion] is empty.
    /// This is always false for [Quaternion] since it always has a length of 4.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

// Default impl for Quaternion<T>, which is Identity
impl<T> Default for Quaternion<T>
where
    Self: Identity,
{
    fn default() -> Self {
        Self::IDENTITY
    }
}

// Bytemuck trait implementations for Quaternion<T>
common::impl_bytemuck_basic!([T], Quaternion<T>, item: T);

// Blanket impl for HasScalar for Quaternion<T> where T implements Scalar
impl<T> HasScalar for Quaternion<T>
where
    T: Scalar,
{
    type Scalar = T;
}
