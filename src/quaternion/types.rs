#[cfg(feature = "zerocopy")]
use ::zerocopy::*;

use crate::{
    scalar::{Scalard, Scalarf},
    tuple::Tuple,
};

/// A quaternion with components of type `T`.
pub type Quat<T> = Quaternion<T>;

/// A quaternion with single-precision floating-point components.
pub type Quatf = Quat<Scalarf>;

/// A quaternion with double-precision floating-point components.
pub type Quatd = Quat<Scalard>;

/// A quaternion.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(transparent)]
pub struct Quaternion<T> {
    pub(crate) data: Tuple<T, 4>,
}

/// A struct representing the fields of a quaternion.
#[doc(hidden)]
#[repr(C)]
pub struct QuaternionFields<T> {
    /// The x-component of the quaternion.
    pub x: T,

    /// The y-component of the quaternion.
    pub y: T,

    /// The z-component of the quaternion.
    pub z: T,

    /// The (scalar) w-component of the quaternion.
    pub w: T,
}
