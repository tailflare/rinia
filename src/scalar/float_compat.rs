use crate::ops::{Abs, Infinity, NaN, Negate, One, Rounding, Select, Zero};

/// A trait for types that can perform floating-point mathematical operations.
pub trait Float:
    Sized + Copy + Zero + One + Select + NaN + Infinity + Negate + Abs + Rounding
{
    /// Computes the sine of the value.
    fn sin(self) -> Self;

    /// Computes the cosine of the value.
    fn cos(self) -> Self;

    /// Computes the tangent of the value.
    fn tan(self) -> Self;

    /// Computes the square root of the value.
    fn sqrt(self) -> Self;

    /// Computes the exponential of the value.
    fn exp(self) -> Self;

    /// Computes the natural logarithm of the value.
    fn ln(self) -> Self;

    /// Raises the value to the power of `exp`.
    fn powf(self, exp: Self) -> Self;

    /// Returns the sign of the value.
    fn signum(self) -> Self;

    /// Returns a number with the magnitude of `self` and the sign of `sign`.
    fn copysign(self, sign: Self) -> Self;

    /// Fused multiply-add: `(self * a) + b` with a single rounding.
    fn mul_add(self, a: Self, b: Self) -> Self;

    /// Euclidean division counterpart to `rem_euclid`.
    fn div_euclid(self, rhs: Self) -> Self;

    /// Least nonnegative remainder modulo `rhs`.
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Raises `self` to an integer power.
    fn powi(self, n: i32) -> Self;

    /// Returns `2^self`.
    fn exp2(self) -> Self;

    /// Logarithm with an arbitrary base.
    fn log(self, base: Self) -> Self;

    /// Base-2 logarithm.
    fn log2(self) -> Self;

    /// Base-10 logarithm.
    fn log10(self) -> Self;

    /// Cube root.
    fn cbrt(self) -> Self;

    /// Hypotenuse length `sqrt(self^2 + other^2)`.
    fn hypot(self, other: Self) -> Self;

    /// Arc sine.
    fn asin(self) -> Self;

    /// Arc cosine.
    fn acos(self) -> Self;

    /// Arc tangent.
    fn atan(self) -> Self;

    /// Four-quadrant arc tangent.
    fn atan2(self, other: Self) -> Self;

    /// Simultaneously computes sine and cosine.
    fn sin_cos(self) -> (Self, Self);

    /// Returns `e^self - 1`.
    fn exp_m1(self) -> Self;

    /// Returns `ln(1 + self)`.
    fn ln_1p(self) -> Self;

    /// Hyperbolic sine.
    fn sinh(self) -> Self;

    /// Hyperbolic cosine.
    fn cosh(self) -> Self;

    /// Hyperbolic tangent.
    fn tanh(self) -> Self;

    /// Inverse hyperbolic sine.
    fn asinh(self) -> Self;

    /// Inverse hyperbolic cosine.
    fn acosh(self) -> Self;

    /// Inverse hyperbolic tangent.
    fn atanh(self) -> Self;
}
