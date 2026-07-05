use core::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use super::type_lists::with_all_scalar_types;
use crate::{
    ops::{Abs, HasScalar, Negate, One, Select, Zero},
    scalar::Float,
};

/// A trait representing a scalar type with the minimal set of operations and constants required
/// for mathematical computations.
pub trait Scalar:
    Copy
    + Debug
    + HasScalar
    + Zero
    + One
    + Select
    + PartialEq
    + PartialOrd
    + ScalarCasts
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
    /// The maximum finite value for the scalar type.
    const MAX: Self;

    /// The minimum finite value for the scalar type.
    const MIN: Self;

    /// Converts a value of type `T` into the implementing scalar type.
    #[inline]
    fn from_scalar<T: Scalar>(value: T) -> Self
    where
        Self: ScalarCast<T>,
    {
        <Self as ScalarCast<T>>::from_scalar_impl(value)
    }

    /// Converts the implementing scalar type into a value of type `T`.
    #[inline]
    fn as_scalar<T: Scalar>(&self) -> T
    where
        Self: ScalarCast<T>,
    {
        <Self as ScalarCast<T>>::as_scalar_impl(self)
    }
}

/// A trait representing a floating-point scalar type with additional constants and operations.
pub trait FloatScalar:
    Scalar + Float + AbsDiffEq<Epsilon = Self> + RelativeEq<Epsilon = Self> + UlpsEq<Epsilon = Self>
{
}

/// A trait representing an integer scalar type.
pub trait IntScalar: Scalar + Ord + Eq {}

/// A trait representing an unsigned integer scalar type.
pub trait UIntScalar: IntScalar {}

/// A trait representing a signed integer scalar type.
pub trait SIntScalar: IntScalar + Negate + Abs {}

/// A trait for converting between different scalar types.
pub trait ScalarCast<T>: Sized {
    fn from_scalar_impl(value: T) -> Self;
    fn as_scalar_impl(&self) -> T;
}

macro_rules! define_scalar_casts_trait {
    ($($ty:ty),+ $(,)?) => {
        pub trait ScalarCasts: $(ScalarCast<$ty> +)+ {}

        impl<T> ScalarCasts for T where T: $(ScalarCast<$ty> +)+ {}
    };
}

with_all_scalar_types!(define_scalar_casts_trait);
