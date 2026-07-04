use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use core_maths::CoreFloat;

/// A trait representing a scalar type with the minimal set of operations and constants required
/// for mathematical computations.
pub trait Scalar:
    Copy
    + Debug
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg
{
    /// The additive identity (zero) for the scalar type.
    const ZERO: Self;

    /// The multiplicative identity (one) for the scalar type.
    const ONE: Self;
}

/// A trait representing a floating-point scalar type with additional constants and operations.
pub trait FloatScalar:
    Scalar + CoreFloat + AbsDiffEq<Epsilon = Self> + RelativeEq<Epsilon = Self> + UlpsEq<Epsilon = Self>
{
    /// Infinity for the scalar type.
    const INFINITY: Self;

    /// Negative infinity for the scalar type.
    const NEG_INFINITY: Self;

    /// NaN (Not a Number) for the scalar type.
    const NAN: Self;
}
