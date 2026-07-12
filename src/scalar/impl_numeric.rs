use crate::{common, numeric::Rounding, scalar};

// Implement scalar constants for all scalars
scalar::impl_scalar_constants!(
    [usize, zero: 0, one: 1, two: 2],
    [isize, zero: 0, one: 1, two: 2, neg_one: -1],
    [u8, zero: 0, one: 1, two: 2],
    [u16, zero: 0, one: 1, two: 2],
    [u32, zero: 0, one: 1, two: 2],
    [u64, zero: 0, one: 1, two: 2],
    [u128, zero: 0, one: 1, two: 2],
    [i8, zero: 0, one: 1, two: 2, neg_one: -1],
    [i16, zero: 0, one: 1, two: 2, neg_one: -1],
    [i32, zero: 0, one: 1, two: 2, neg_one: -1],
    [i64, zero: 0, one: 1, two: 2, neg_one: -1],
    [i128, zero: 0, one: 1, two: 2, neg_one: -1],
    [f32, zero: 0.0, one: 1.0, two: 2.0, neg_one: -1.0, half: 0.5],
    [f64, zero: 0.0, one: 1.0, two: 2.0, neg_one: -1.0, half: 0.5],
);

// Implement Bounded for all scalar types.
scalar::impl_scalar_bounded!(
    [usize, usize::MIN, usize::MAX],
    [isize, isize::MIN, isize::MAX],
    [u8, u8::MIN, u8::MAX],
    [u16, u16::MIN, u16::MAX],
    [u32, u32::MIN, u32::MAX],
    [u64, u64::MIN, u64::MAX],
    [u128, u128::MIN, u128::MAX],
    [i8, i8::MIN, i8::MAX],
    [i16, i16::MIN, i16::MAX],
    [i32, i32::MIN, i32::MAX],
    [i64, i64::MIN, i64::MAX],
    [i128, i128::MIN, i128::MAX],
    [f32, f32::MIN, f32::MAX],
    [f64, f64::MIN, f64::MAX],
);

// Implement Min/Max for all scalar types.
scalar::impl_scalar_min_max!(
    usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64
);

// Implement clamping for all scalar types.
scalar::impl_scalar_clamp!(
    usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64
);

// Implement Abs for signed scalar types.
scalar::impl_scalar_trait!(Abs, [f32, f64, i8, i16, i32, i64, i128, isize], {unary, abs});

// Implement Sqrt for floating scalar types.
scalar::impl_scalar_trait!(Sqrt, [f32, f64], {unary, sqrt});

// Implement Trigonometry for floating scalar types.
scalar::impl_scalar_trait!(
    Trigonometry,
    [f32, f64],
    {unary, sin, [f32: libm::sinf, f64: libm::sin]},
    {unary, cos, [f32: libm::cosf, f64: libm::cos]},
    {unary, tan, [f32: libm::tanf, f64: libm::tan]},
    {unary, asin, [f32: libm::asinf, f64: libm::asin]},
    {unary, acos, [f32: libm::acosf, f64: libm::acos]},
    {unary, atan, [f32: libm::atanf, f64: libm::atan]},
    {binary, atan2, Self, [f32: libm::atan2f, f64: libm::atan2]},
    {unary_pair, sin_cos, [f32: libm::sincosf, f64: libm::sincos]}
);

// Implement Exponential for floating scalar types.
scalar::impl_scalar_trait!(
    Exponential,
    [f32, f64],
    {unary, exp, [f32: libm::expf, f64: libm::exp]},
    {unary, exp2, [f32: libm::exp2f, f64: libm::exp2]},
    {unary, exp_m1, [f32: libm::expm1f, f64: libm::expm1]},
    {unary, ln, [f32: libm::logf, f64: libm::log]},
    {unary, ln_1p, [f32: libm::log1pf, f64: libm::log1p]},
    {binary, log, Self, [f32: |x: f32, b: f32| libm::logf(x) / libm::logf(b), f64: |x: f64, b: f64| libm::log(x) / libm::log(b)]},
    {unary, log2, [f32: libm::log2f, f64: libm::log2]},
    {unary, log10, [f32: libm::log10f, f64: libm::log10]}
);

// Implement Power for floating scalar types.
scalar::impl_scalar_trait!(
    Power,
    [f32, f64],
    {binary, powf, Self, [f32: libm::powf, f64: libm::pow]},
    {binary, powi, i32, [f32: |x: f32, n: i32| libm::powf(x, n as f32), f64: |x: f64, n: i32| libm::pow(x, n as f64)]}
);

// Implement Root for floating scalar types.
scalar::impl_scalar_trait!(
    Cbrt,
    [f32, f64],
    {unary, cbrt, [f32: libm::cbrtf, f64: libm::cbrt]});

// Implement Hyperbolic for floating scalar types.
scalar::impl_scalar_trait!(
    Hyperbolic,
    [f32, f64],
    {unary, sinh, [f32: libm::sinhf, f64: libm::sinh]},
    {unary, cosh, [f32: libm::coshf, f64: libm::cosh]},
    {unary, tanh, [f32: libm::tanhf, f64: libm::tanh]},
    {unary, asinh, [f32: libm::asinhf, f64: libm::asinh]},
    {unary, acosh, [f32: libm::acoshf, f64: libm::acosh]},
    {unary, atanh, [f32: libm::atanhf, f64: libm::atanh]}
);

// Implement Euclidean for floating scalar types.
scalar::impl_scalar_trait!(
    Hypot,
    [f32, f64],
    {binary, hypot, Self, [f32: libm::hypotf, f64: libm::hypot]}
);

// Implement Finite/Infinite/Nan for floating scalar types.
scalar::impl_scalar_float_predicates!(f32, f64);

// Implement Floor for floating scalar types.
scalar::impl_scalar_trait!(
    Floor,
    [f32, f64],
    {unary, floor, [f32: libm::floorf, f64: libm::floor]}
);

// Implement Ceil for floating scalar types.
scalar::impl_scalar_trait!(
    Ceil,
    [f32, f64],
    {unary, ceil, [f32: libm::ceilf, f64: libm::ceil]}
);

// Implement Trunc for floating scalar types.
scalar::impl_scalar_trait!(
    Trunc,
    [f32, f64],
    {unary, trunc, [f32: libm::truncf, f64: libm::trunc]}
);

// Implement Round for floating scalar types.
scalar::impl_scalar_trait!(
    Round,
    [f32, f64],
    {unary, round, [f32: libm::roundf, f64: libm::round]}
);

// Implement Fract for floating scalar types.
scalar::impl_scalar_trait!(
    Fract,
    [f32, f64],
    {
        unary,
        fract,
        [f32: |x| libm::modff(x).0, f64: |x| libm::modf(x).0]
    }
);

// Implement Rounding marker for floating scalar types as all of the rounding traits are
// implemented above for them.
common::impl_marker_trait!(Rounding, [f32, f64]);
