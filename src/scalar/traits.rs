use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel},
    numeric::{
        Abs, BoundedMax, BoundedMin, Cbrt, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg,
        CheckedRem, CheckedSub, Clamp, Exponential, Half, Hyperbolic, Hypot, Infinite, IsFinite,
        IsInfinite, IsNan, MinMax, MulAdd, Nan, NegOne, One, Power, Rounding, SaturatingAdd,
        SaturatingDiv, SaturatingMul, SaturatingNeg, SaturatingSub, SignedEquivalent, Sqrt,
        Trigonometry, Two, UnsignedEquivalent, WrappingAdd, WrappingDiv, WrappingMul, WrappingNeg,
        WrappingRem, WrappingSub, Zero,
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
pub trait Int: Scalar + SignedEquivalent + UnsignedEquivalent {}

/// Marker trait for signed integer scalar types.
pub trait SignedInt: Signed + Int {}

/// Marker trait for unsigned integer scalar types.
pub trait UnsignedInt: Unsigned + Int {}

/// Marker trait for types that have an associated scalar type.
pub trait HasScalar {
    type Scalar: Scalar;
}

/// Marker trait for scalar types that implement the basic set of scalar math ops.
pub trait ScalarOps:
    Scalar
    + Copy
    + core::fmt::Debug
    + PartialEq
    + PartialOrd
    + MinMax
    + Clamp
    + BoundedMin
    + BoundedMax
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
pub trait SignedOps: ScalarOps + Signed + Abs + Neg<Output = Self> + NegOne {}

// Marker trait for integer scalar types that implement the basic set of integer scalar math ops.
pub trait IntOps: Int + ScalarOps {}

// Marker trait for signed integer scalar types that implement the basic set of signed integer scalar math ops.
pub trait SignedIntOps: SignedInt + IntOps + SignedOps {}

// Marker trait for checked integer scalar types that implement the basic set of checked integer scalar math ops.
pub trait CheckedIntOps:
    IntOps + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem
{
}

// Marker trait for saturating integer scalar types that implement the basic set of saturating integer scalar math ops.
pub trait SaturatingIntOps:
    IntOps + SaturatingAdd + SaturatingSub + SaturatingMul + SaturatingDiv
{
}

// Marker trait for wrapping integer scalar types that implement the basic set of wrapping integer scalar math ops.
pub trait WrappingIntOps:
    IntOps + WrappingAdd + WrappingSub + WrappingMul + WrappingDiv + WrappingRem
{
}

// Marker trait for checked signed scalar types that can be negated.
pub trait CheckedIntNegOps: CheckedNeg {}

// Marker trait for saturating signed scalar types that can be negated.
pub trait SaturatingIntNegOps: SaturatingNeg {}

// Marker trait for wrapping signed scalar types that can be negated.
pub trait WrappingIntNegOps: WrappingNeg {}

/// Marker trait for floating-point scalar types that implement the basic set of floating-point math ops.
pub trait FloatOps:
    Float
    + SignedOps
    + Rounding
    + IsFinite
    + Infinite
    + IsInfinite
    + Nan
    + IsNan
    + MulAdd
    + Sqrt
    + Trigonometry
    + Exponential
    + Power
    + Cbrt
    + Hyperbolic
    + Hypot
    + ApproxEqAbs<Tolerance = Self>
    + ApproxEqRel<Tolerance = Self>
    + Half
{
}
