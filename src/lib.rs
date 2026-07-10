#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "std")]
extern crate std;

// Our modules
pub mod common;
pub mod ops;
pub mod quaternion;
pub mod scalar;
pub mod tuple;
pub mod vector;

// Our surface-level re-exports
pub use self::{
    quaternion::{Quat, QuatScalar, Quaternion},
    scalar::{Float, FloatScalar, IntScalar, SIntScalar, Scalar, UIntScalar},
    tuple::ScalarTuple,
    vector::{Vec2, Vec3, Vec4, Vector, VectorScalar},
};

// External dependencies re-exported for convenience
pub mod external {
    pub use approx;
    pub use libm;
    #[cfg(feature = "mint")]
    pub use mint;
}

// Convenience types
pub mod types {
    use super::*;

    pub type Scalarf = f32;
    pub type Vec2f = Vec2<f32>;
    pub type Vec3f = Vec3<f32>;
    pub type Vec4f = Vec4<f32>;
    pub type Quatf = Quat<f32>;

    pub type Scalard = f64;
    pub type Vec2d = Vec2<f64>;
    pub type Vec3d = Vec3<f64>;
    pub type Vec4d = Vec4<f64>;
    pub type Quatd = Quat<f64>;
}

pub use self::types::*;

// Prelude module for convenient imports
pub mod prelude {
    pub use super::{
        ops::*,
        quaternion::{Quat, Quaternion},
        scalar::{Float, Scalar},
        types::*,
        vector::{Vec2, Vec3, Vec4, Vector},
    };
}
