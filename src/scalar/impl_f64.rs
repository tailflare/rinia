#[allow(unused_imports)]
use core_maths::*;

use super::macros::impl_float_repr;
use crate::{FloatScalar, Scalar};

impl Scalar for f64 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}

impl_float_repr!(f64, [f32, f64]);

impl FloatScalar for f64 {
    const INFINITY: Self = f64::INFINITY;

    const NEG_INFINITY: Self = f64::NEG_INFINITY;

    const NAN: Self = f64::NAN;
}
