#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod ops;
pub mod scalar;

pub use self::scalar::{FloatScalar, IntScalar, SIntScalar, Scalar, UIntScalar};

pub mod prelude {
    pub use super::{ops::*, scalar::*};
}
