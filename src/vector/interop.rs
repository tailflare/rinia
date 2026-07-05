#![cfg(feature = "mint")]

use crate::{Vec2, Vec3, Vec4, VectorScalar};

impl<T: VectorScalar> From<mint::Vector2<T>> for Vec2<T> {
    #[inline]
    fn from(v: mint::Vector2<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl<T: VectorScalar> From<Vec2<T>> for mint::Vector2<T> {
    #[inline]
    fn from(v: Vec2<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl<T: VectorScalar> From<mint::Vector3<T>> for Vec3<T> {
    #[inline]
    fn from(v: mint::Vector3<T>) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl<T: VectorScalar> From<Vec3<T>> for mint::Vector3<T> {
    #[inline]
    fn from(v: Vec3<T>) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl<T: VectorScalar> From<mint::Vector4<T>> for Vec4<T> {
    #[inline]
    fn from(v: mint::Vector4<T>) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w: v.w }
    }
}

impl<T: VectorScalar> From<Vec4<T>> for mint::Vector4<T> {
    #[inline]
    fn from(v: Vec4<T>) -> Self {
        Self { x: v.x, y: v.y, z: v.z, w: v.w }
    }
}
