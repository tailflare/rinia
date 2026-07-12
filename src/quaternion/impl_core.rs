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
