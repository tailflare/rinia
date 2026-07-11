use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel, Lerp},
    numeric::{
        Abs, Bounded, Cbrt, Clamp, Exponential, Finite, Half, Hyperbolic, Hypot, Infinite, MinMax,
        Nan, NegOne, One, Power, Rounding, Sqrt, Trigonometry, Two, Zero,
    },
};

/// Marker trait for scalar types.
pub trait Scalar {}

/// Marker trait for signed scalar types.
pub trait Signed: Scalar {}

/// Marker trait for unsigned scalar types.
pub trait Unsigned: Scalar {}

/// Marker trait for floating-point scalar types.
pub trait Float: Signed {}

/// Marker trait for integer scalar types.
pub trait Int: Scalar {}

/// Marker trait for signed integer scalar types.
pub trait SignedInt: Signed + Int {}

/// Marker trait for unsigned integer scalar types.
pub trait UnsignedInt: Unsigned + Int {}

/// Marker trait for types that have an associated scalar type.
pub trait HasScalar {
    type Scalar: Scalar;
}

/// Marker trait for scalar types that implement the basic set of scalar math ops.
pub trait ScalarMath:
    Scalar
    + Copy
    + Default
    + core::fmt::Debug
    + PartialEq
    + PartialOrd
    + MinMax
    + Clamp
    + Bounded
    + Zero
    + One
    + Two
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

/// Marker trait for signed scalar types that implement the basic set of signed scalar math ops.
pub trait SignedMath: ScalarMath + Signed + Abs + Neg<Output = Self> + NegOne {}

/// Marker trait for floating-point scalar types that implement the basic set of floating-point math ops.
pub trait FloatMath:
    Float
    + SignedMath
    + Rounding
    + Finite
    + Infinite
    + Nan
    + Sqrt
    + Trigonometry
    + Exponential
    + Power
    + Cbrt
    + Hyperbolic
    + Hypot
    + ApproxEqAbs<Tolerance = Self>
    + ApproxEqRel<Tolerance = Self>
    + Lerp<Scalar = Self>
    + Half
{
}
