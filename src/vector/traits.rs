use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{
    Scalar,
    ops::{DotProduct, Negate, One, Select, Zero},
    tuple::ScalarTuple,
};

/// A trait representing a scalar type that can be used in vector operations.
pub trait VectorScalar: Scalar + Negate {}

/// A trait representing a vector type with a fixed dimension.
pub trait Vector:
    Copy
    + ScalarTuple
    + Zero
    + One
    + Select
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Negate
    + DotProduct<Output = Self::Scalar>
{
}
