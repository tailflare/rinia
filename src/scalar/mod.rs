mod impl_algebra;
mod impl_casts;
mod impl_core;
mod impl_markers;
mod impl_numeric;
mod macros_algebra;
mod macros_casting;
mod macros_numeric;
mod macros_scalar;
mod tests;
mod traits;
mod types;

pub(crate) use self::{macros_algebra::*, macros_casting::*, macros_numeric::*, macros_scalar::*};
pub use self::{
    traits::{
        Float, FloatOps, HasScalar, Int, Scalar, ScalarOps, Signed, SignedInt, SignedOps, Unsigned,
        UnsignedInt,
    },
    types::{Scalard, Scalarf},
};
