#[cfg(feature = "zerocopy")]
use ::zerocopy::*;

use crate::QuatScalar;

/// A quaternion type, representing a rotation in 3D space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(C)]
pub struct Quat<T: QuatScalar> {
    /// The x component of the quaternion.
    pub x: T,

    /// The y component of the quaternion.
    pub y: T,

    /// The z component of the quaternion.
    pub z: T,

    /// The w component of the quaternion.
    pub w: T,
}
