#[allow(unused_imports)]
use core_maths::*;

use crate::{FloatScalar, Scalar};

impl Scalar for f32 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}

impl FloatScalar for f32 {
    const INFINITY: Self = f32::INFINITY;

    const NEG_INFINITY: Self = f32::NEG_INFINITY;

    const NAN: Self = f32::NAN;
}
