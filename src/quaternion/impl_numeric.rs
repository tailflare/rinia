use core::ops::Neg;

use crate::{
    common,
    numeric::{IsZero, Negate, Zero},
    quaternion::Quaternion,
};

// Implement casts
crate::impl_numeric_casts_transparent!([T], wrapper: Quaternion, Quaternion<T> => Quaternion<U>, item: T, field: data);

// Zero inherent
impl<T> Quaternion<T>
where
    T: Zero,
{
    /// The zero quaternion (0, 0, 0, 0).
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

// Zero trait
impl<T> Zero for Quaternion<T>
where
    T: Zero,
{
    const ZERO: Self = Quaternion::ZERO;
}

// IsZero inherent
impl<T> Quaternion<T>
where
    T: Copy + Zero + PartialEq,
{
    /// Checks if the quaternion is equal to the zero quaternion.
    #[inline]
    pub fn is_zero(self) -> bool {
        self == Self::ZERO
    }
}

// IsZero trait
impl<T> IsZero for Quaternion<T>
where
    T: Copy + Zero + PartialEq,
{
    #[inline]
    fn is_zero(self) -> bool {
        Quaternion::is_zero(self)
    }
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
