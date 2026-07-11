#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(test)]
extern crate alloc;

// Our modules
pub mod algebra;
mod common;
pub mod numeric;
pub mod quaternion;
pub mod scalar;
pub mod tuple;
pub mod vector;

// Our surface-level re-exports
pub use self::{
    quaternion::{Quat, Quatd, Quaternion, Quatf},
    scalar::{Scalar, Scalard, Scalarf},
    vector::{Vec2, Vec2d, Vec2f, Vec3, Vec3d, Vec3f, Vec4, Vec4d, Vec4f, Vector},
};

// External dependencies re-exported for convenience
pub mod external {
    pub use libm;
    #[cfg(feature = "mint")]
    pub use mint;
}

// Prelude module for convenient imports
pub mod prelude {
    pub use super::{
        algebra::*,
        numeric::*,
        quaternion::{Quat, Quatd, Quaternion, Quatf},
        scalar::{
            Float, FloatMath, HasScalar, Int, Scalar, ScalarMath, Scalard, Scalarf, Signed,
            SignedInt, SignedMath, Unsigned, UnsignedInt,
        },
        vector::{Vec2, Vec2d, Vec2f, Vec3, Vec3d, Vec3f, Vec4, Vec4d, Vec4f, Vector},
    };
}
