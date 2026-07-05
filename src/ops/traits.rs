use core::ops::Neg;

use crate::Scalar;

/// A trait representing the additive identity for a type.
pub trait Zero {
    /// The additive identity for the type (typically 0).
    const ZERO: Self;

    /// Checks if the value is equal to the additive identity (zero).
    #[allow(clippy::wrong_self_convention)]
    fn is_zero(self) -> bool;
}

/// A trait representing the multiplicative identity for a type.
pub trait One {
    /// The multiplicative identity for the type (typically 1).
    const ONE: Self;
}

/// A trait for selecting values based on comparisons.
pub trait Select {
    /// Returns the minimum of two values.
    fn min_val(self, other: Self) -> Self;

    /// Returns the maximum of two values.
    fn max_val(self, other: Self) -> Self;
}

/// A trait representing a type that has an associated scalar type.
pub trait HasScalar {
    /// The associated scalar type for the implementing type.
    type Scalar: Scalar;
}

/// A trait for types that can be negated.
pub trait Negate: Neg<Output = Self> {
    /// Returns the additive inverse of the value.
    fn negate(self) -> Self;
}

/// A trait for types that can perform dot product operations.
pub trait DotProduct<Rhs = Self> {
    type Output;

    /// Computes the dot product of two values.
    fn dot(self, rhs: Rhs) -> Self::Output;
}

/// A trait for types that can provide an absolute value.
///
/// Implemented for signed integers and floating point primitives.
pub trait Abs {
    type Output;

    /// Returns the absolute value of the type.
    fn abs(self) -> Self::Output;
}

/// A trait for types that can perform rounding operations.
pub trait Rounding {
    /// Returns the largest integer less than or equal to the value.
    fn floor(self) -> Self;

    /// Returns the smallest integer greater than or equal to the value.
    fn ceil(self) -> Self;

    /// Returns the integer part of the value truncated toward zero.
    fn trunc(self) -> Self;

    /// Returns the nearest integer to the value. Rounds half-way cases away from zero.
    fn round(self) -> Self;

    /// Returns the fractional part of the value.
    fn fract(self) -> Self;
}

/// A trait for types that can represent positive and negative infinity, as well as check for
/// finiteness.
pub trait Infinity {
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    /// Checks if the value is finite.
    #[allow(clippy::wrong_self_convention)]
    fn is_finite(self) -> bool;

    /// Checks if the value is infinite.
    #[allow(clippy::wrong_self_convention)]
    fn is_infinite(self) -> bool;
}

/// A trait for types that can represent "Not a Number" (NaN) values and check for NaN.
pub trait NaN {
    const NAN: Self;

    /// Checks if the value is NaN.
    #[allow(clippy::wrong_self_convention)]
    fn is_nan(self) -> bool;
}
