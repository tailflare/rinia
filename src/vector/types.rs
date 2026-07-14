#[cfg(feature = "zerocopy")]
use ::zerocopy::*;

use crate::{
    scalar::{Scalard, Scalarf},
    tuple::Tuple,
};

/// A 2-dimensional vector with components of type `T`.
pub type Vec2<T> = Vector<T, 2>;

/// A 3-dimensional vector with components of type `T`.
pub type Vec3<T> = Vector<T, 3>;

/// A 4-dimensional vector with components of type `T`.
pub type Vec4<T> = Vector<T, 4>;

/// A 2-dimensional vector with single-precision floating-point components.
pub type Vec2f = Vec2<Scalarf>;

/// A 3-dimensional vector with single-precision floating-point components.
pub type Vec3f = Vec3<Scalarf>;

/// A 4-dimensional vector with single-precision floating-point components.
pub type Vec4f = Vec4<Scalarf>;

/// A 2-dimensional vector with double-precision floating-point components.
pub type Vec2d = Vec2<Scalard>;

/// A 3-dimensional vector with double-precision floating-point components.
pub type Vec3d = Vec3<Scalard>;

/// A 4-dimensional vector with double-precision floating-point components.
pub type Vec4d = Vec4<Scalard>;

/// A generic vector type with `N` dimensions and components of type `T`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(transparent)]
pub struct Vector<T, const N: usize> {
    pub(crate) data: Tuple<T, N>,
}

/// A 2-dimensional vector with named fields `x` and `y`.
#[doc(hidden)]
#[repr(C)]
pub struct Vector2Fields<T> {
    /// The x-component of the vector.
    pub x: T,

    /// The y-component of the vector.
    pub y: T,
}

/// A 3-dimensional vector with named fields `x`, `y`, and `z`.
#[doc(hidden)]
#[repr(C)]
pub struct Vector3Fields<T> {
    /// The x-component of the vector.
    pub x: T,

    /// The y-component of the vector.
    pub y: T,

    /// The z-component of the vector.
    pub z: T,
}

/// A 4-dimensional vector with named fields `x`, `y`, `z`, and `w`.
#[doc(hidden)]
#[repr(C)]
pub struct Vector4Fields<T> {
    /// The x-component of the vector.
    pub x: T,

    /// The y-component of the vector.
    pub y: T,

    /// The z-component of the vector.
    pub z: T,

    /// The w-component of the vector.
    pub w: T,
}
