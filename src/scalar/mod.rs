mod impl_algebra;
mod impl_casts;
mod impl_core;
mod impl_markers;
mod impl_numeric;
mod macros;
mod tests;
mod traits;
mod types;

pub(crate) use self::macros::*;
pub use self::{
    traits::{
        Float, FloatOps, HasScalar, Int, Scalar, ScalarOps, Signed, SignedInt, SignedOps, Unsigned,
        UnsignedInt,
    },
    types::{Scalard, Scalarf},
};
