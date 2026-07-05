use crate::{
    FloatScalar, ScalarTuple,
    ops::{DotProduct, Negate},
};

/// A trait representing a scalar type that can be used in quaternions.
pub trait QuatScalar: FloatScalar {}

/// A trait representing a quaternion type with the minimal set of operations required
/// for mathematical computations.
pub trait Quaternion: Copy + ScalarTuple + DotProduct + Negate {}
