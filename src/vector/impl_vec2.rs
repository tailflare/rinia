use crate::{Vec2, VectorScalar, impl_bytemuck_pod, vector::macros::impl_vector_traits};

impl<T: VectorScalar> Vec2<T> {
    /// Creates a new vector with the given components.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl_vector_traits!(Vec2<T: VectorScalar>, 2);
impl_bytemuck_pod!(Vec2<T: VectorScalar>);
