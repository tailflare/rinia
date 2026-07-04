#![no_std]
#![deny(rustdoc::broken_intra_doc_links)]

pub(crate) mod scalar;

pub use self::scalar::{FloatScalar, IntScalar, Scalar};
