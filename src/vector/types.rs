#[cfg(feature = "zerocopy")]
use ::zerocopy::*;

use crate::VectorScalar;

/// A 2D vector.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(C)]
pub struct Vec2<T: VectorScalar> {
    /// The x component of the vector.
    pub x: T,

    /// The y component of the vector.
    pub y: T,
}

/// A 3D vector.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(C)]
pub struct Vec3<T: VectorScalar> {
    /// The x component of the vector.
    pub x: T,

    /// The y component of the vector.
    pub y: T,

    /// The z component of the vector.
    pub z: T,
}

/// A 4D vector.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(C)]
pub struct Vec4<T: VectorScalar> {
    /// The x component of the vector.
    pub x: T,

    /// The y component of the vector.
    pub y: T,

    /// The z component of the vector.
    pub z: T,

    /// The w component of the vector.
    pub w: T,
}
