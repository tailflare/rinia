use core::ops::Neg;

use crate::{
    common,
    numeric::{Negate, Zero},
    quaternion::Quaternion,
};

// Zero inherent
impl<T> Quaternion<T>
where
    T: Zero,
{
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

// Zero trait
impl<T> Zero for Quaternion<T>
where
    T: Zero,
{
    const ZERO: Self = Quaternion::ZERO;
}

// Implement Finite/Infinite/Nan for Quaternion<T>.
common::impl_tuple_wrapper_numeric_predicates!([T], Quaternion<T>, item: T, len: 4);

// Negate inherent
impl<T> Quaternion<T>
where
    T: Copy + Neg<Output = T>,
{
    /// Negates the quaternion, returning a new quaternion with all components negated.
    #[inline]
    pub fn negate(self) -> Self {
        let out = self.into_tuple().negate();
        Self::from_tuple(out)
    }
}

// Negate trait
impl<T> Negate for Quaternion<T>
where
    T: Copy + Neg<Output = T>,
{
    #[inline]
    fn negate(self) -> Self::Output {
        Quaternion::negate(self)
    }
}
