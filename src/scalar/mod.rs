mod impl_algebra;
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
        Float, FloatMath, HasScalar, Int, Scalar, ScalarMath, Signed, SignedInt, SignedMath,
        Unsigned, UnsignedInt,
    },
    types::{Scalard, Scalarf},
};
