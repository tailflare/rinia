use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core_maths::CoreFloat;

use super::macros::with_all_scalar_types;

macro_rules! define_from_all_scalars_trait {
    ($($ty:ty),+ $(,)?) => {
        pub trait FromAllScalars: $(FromScalar<$ty> +)+ {}

        impl<T> FromAllScalars for T where T: $(FromScalar<$ty> +)+ {}
    };
}

with_all_scalar_types!(define_from_all_scalars_trait);

/// A trait representing a scalar type with the minimal set of operations and constants required
/// for mathematical computations.
pub trait Scalar:
    Copy
    + Debug
    + PartialEq
    + PartialOrd
    + MinMax
    + FromAllScalars
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    /// The additive identity (zero) for the scalar type.
    const ZERO: Self;

    /// The multiplicative identity (one) for the scalar type.
    const ONE: Self;

    #[inline]
    fn from_scalar<T: Scalar>(value: T) -> Self
    where
        Self: FromScalar<T>,
    {
        <Self as FromScalar<T>>::from_scalar_impl(value)
    }

    #[inline]
    fn as_scalar<T: Scalar>(&self) -> T
    where
        Self: FromScalar<T>,
    {
        <Self as FromScalar<T>>::as_scalar_impl(self)
    }
}

/// A trait representing a floating-point scalar type with additional constants and operations.
pub trait FloatScalar:
    Scalar
    + CoreFloat
    + Neg
    + AbsDiffEq<Epsilon = Self>
    + RelativeEq<Epsilon = Self>
    + UlpsEq<Epsilon = Self>
{
    /// Infinity for the scalar type.
    const INFINITY: Self;

    /// Negative infinity for the scalar type.
    const NEG_INFINITY: Self;

    /// NaN (Not a Number) for the scalar type.
    const NAN: Self;

    /// Checks if the scalar value is NaN (Not a Number).
    fn is_nan(self) -> bool;

    /// Checks if the scalar value is finite (not infinite or NaN).
    fn is_finite(self) -> bool;

    /// Checks if the scalar value is infinite (positive or negative infinity).
    fn is_infinite(self) -> bool;

    /// Checks if the scalar value is zero.
    fn is_zero(self) -> bool;
}

/// A trait representing an integer scalar type.
pub trait IntScalar: Scalar {}

pub trait MinMax {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

/// A trait for converting between different scalar types.
pub trait FromScalar<T>: Sized {
    fn from_scalar_impl(value: T) -> Self;
    fn as_scalar_impl(&self) -> T;
}
