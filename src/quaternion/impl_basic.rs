use core::ops::Neg;

use crate::{
    FloatScalar, Quat, QuatScalar, Quaternion, impl_bytemuck_pod,
    ops::{Negate, impl_dot_product_via_scalar_tuple},
    tuple::impl_scalar_tuple_traits,
};

impl<T> QuatScalar for T where T: FloatScalar {}

impl<T: QuatScalar> Quat<T> {
    /// The identity quaternion, representing no rotation.
    pub const IDENTITY: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO, w: T::ONE };

    /// Creates a new quaternion with the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> Quaternion for Quat<T> where T: QuatScalar {}

impl<T> Default for Quat<T>
where
    T: QuatScalar,
{
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<T> Neg for Quat<T>
where
    T: QuatScalar,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl<T> Negate for Quat<T>
where
    T: QuatScalar,
{
    #[inline]
    fn negate(self) -> Self {
        -self
    }
}

impl_scalar_tuple_traits!(Quat<T: QuatScalar>, 4);
impl_bytemuck_pod!(Quat<T: QuatScalar>);
impl_dot_product_via_scalar_tuple!(Quat<T: QuatScalar>);
