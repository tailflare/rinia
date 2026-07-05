#![cfg(feature = "mint")]

use crate::{Quat, QuatScalar};

impl<T: QuatScalar> From<mint::Quaternion<T>> for Quat<T> {
    #[inline]
    fn from(q: mint::Quaternion<T>) -> Self {
        Self { x: q.v.x, y: q.v.y, z: q.v.z, w: q.s }
    }
}

impl<T: QuatScalar> From<Quat<T>> for mint::Quaternion<T> {
    #[inline]
    fn from(q: Quat<T>) -> Self {
        Self { v: mint::Vector3 { x: q.x, y: q.y, z: q.z }, s: q.w }
    }
}
