use crate::{Vec4, VectorScalar, impl_bytemuck_pod, vector::macros::impl_vector_traits};

impl<T: VectorScalar> Vec4<T> {
    /// Creates a new vector with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl_vector_traits!(Vec4<T: VectorScalar>, 4);
impl_bytemuck_pod!(Vec4<T: VectorScalar>);
