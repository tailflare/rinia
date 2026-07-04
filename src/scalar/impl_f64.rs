#[allow(unused_imports)]
use core_maths::*;

use crate::{FloatScalar, Scalar};

impl Scalar for f64 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}

impl FloatScalar for f64 {
    const INFINITY: Self = f64::INFINITY;

    const NEG_INFINITY: Self = f64::NEG_INFINITY;

    const NAN: Self = f64::NAN;
}
