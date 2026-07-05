#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "std")]
extern crate std;

// Our modules
pub mod core;
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

// Prelude module for convenient imports
pub mod prelude {
    pub use super::{
        external::{approx, libm},
        ops::*,
        quaternion::*,
        scalar::{Float, *},
        tuple::*,
        vector::*,
    };
}
