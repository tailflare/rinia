use crate::{Vec3, VectorScalar, impl_bytemuck_pod, vector::macros::impl_vector_traits};

impl<T: VectorScalar> Vec3<T> {
    /// Creates a new vector with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl_vector_traits!(Vec3<T: VectorScalar>, 3);
impl_bytemuck_pod!(Vec3<T: VectorScalar>);
