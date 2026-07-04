#[allow(unused_imports)]
use core_maths::*;

use super::macros::impl_float_repr;
use crate::{FloatScalar, Scalar};

impl Scalar for f32 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}

impl_float_repr!(f32, [f32, f64]);

impl FloatScalar for f32 {
    const INFINITY: Self = f32::INFINITY;

    const NEG_INFINITY: Self = f32::NEG_INFINITY;

    const NAN: Self = f32::NAN;
}
