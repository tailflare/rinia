mod impl_access;
mod impl_algebra;
mod impl_compat_mint;
mod impl_core;
mod impl_numeric;
mod impl_ops;
mod impl_vector2;
mod impl_vector3;
mod impl_vector4;
mod macros;
mod tests;
mod types;

pub(crate) use self::macros::*;
pub use self::types::{
    Vec2, Vec2d, Vec2f, Vec3, Vec3d, Vec3f, Vec4, Vec4d, Vec4f, Vector, Vector2Fields,
    Vector3Fields, Vector4Fields,
};
