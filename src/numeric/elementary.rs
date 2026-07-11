use core::ops::Neg;

/// Trait for computing the minimum or maximum of two numbers.
pub trait MinMax {
    /// Computes the minimum of two numbers.
    fn minimum(self, other: Self) -> Self;

    /// Computes the maximum of two numbers.
    fn maximum(self, other: Self) -> Self;
}

/// Trait for clamping a number between a minimum and maximum value.
pub trait Clamp {
    /// Clamps the number between a minimum and maximum value.
    fn clamp_value(self, min: Self, max: Self) -> Self;
}

/// A trait that represents a type which can be negated.
pub trait Negate: Neg {
    fn negate(self) -> Self::Output;
}

/// Trait for computing the absolute value of a number.
pub trait Abs {
    /// Computes the absolute value of the number.
    fn abs(self) -> Self;
}

/// Trait for computing the square root of a number.
pub trait Sqrt {
    /// Computes the square root of the number.
    fn sqrt(self) -> Self;
}

/// Trait for trigonometry functions.
pub trait Trigonometry: Sized {
    /// Compute the sine of the number (in radians).
    fn sin(self) -> Self;

    /// Compute the cosine of the number (in radians).
    fn cos(self) -> Self;

    /// Compute the tangent of the number (in radians).
    fn tan(self) -> Self;

    /// Compute the arcsine of the number (in radians).
    fn asin(self) -> Self;

    /// Compute the arccosine of the number (in radians).
    fn acos(self) -> Self;

    /// Compute the arctangent of the number (in radians).
    fn atan(self) -> Self;

    /// Compute the arctangent of the number (in radians) given y and x coordinates.
    fn atan2(self, other: Self) -> Self;

    /// Compute the sine and cosine of the number (in radians) simultaneously.
    fn sin_cos(self) -> (Self, Self);
}

/// Trait for exponential functions.
pub trait Exponential {
    /// Computes the exponential of the number.
    fn exp(self) -> Self;

    /// Computes the base 2 exponential of the number.
    fn exp2(self) -> Self;

    /// Computes the exponential of the number minus 1.
    fn exp_m1(self) -> Self;

    /// Computes the natural logarithm of the number.
    fn ln(self) -> Self;

    /// Computes the natural logarithm of 1 plus the number.
    fn ln_1p(self) -> Self;

    /// Computes the logarithm of the number with respect to a given base.
    fn log(self, base: Self) -> Self;

    /// Computes the base 2 logarithm of the number.
    fn log2(self) -> Self;

    /// Computes the base 10 logarithm of the number.
    fn log10(self) -> Self;
}

/// Trait for power functions.
pub trait Power {
    /// Computes the number raised to the power of another number.
    fn powf(self, exp: Self) -> Self;

    /// Computes the number raised to the power of an integer.
    fn powi(self, n: i32) -> Self;
}

/// Trait for the cube root function.
pub trait Cbrt {
    /// Computes the cube root of the number.
    fn cbrt(self) -> Self;
}

/// Trait for hyperbolic functions.
pub trait Hyperbolic {
    /// Computes the hyperbolic sine of the number.
    fn sinh(self) -> Self;

    /// Computes the hyperbolic cosine of the number.
    fn cosh(self) -> Self;

    /// Computes the hyperbolic tangent of the number.
    fn tanh(self) -> Self;

    /// Computes the inverse hyperbolic sine of the number.
    fn asinh(self) -> Self;

    /// Computes the inverse hyperbolic cosine of the number.
    fn acosh(self) -> Self;

    /// Computes the inverse hyperbolic tangent of the number.
    fn atanh(self) -> Self;
}

/// Trait for hypotenuse calculation.
pub trait Hypot {
    /// Computes the hypotenuse of a right triangle given the lengths of the other two sides.
    fn hypot(self, other: Self) -> Self;
}
