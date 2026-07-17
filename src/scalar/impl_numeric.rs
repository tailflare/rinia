use crate::scalar;

// Implement scalar constants for all scalars (ZERO, ONE, TWO, NEG_ONE, HALF)
scalar::_impl_scalar_constants!(
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

// Implement scalar constants for all floating-point scalars (INFINITY, NEG_INFINITY, NAN)
scalar::_impl_scalar_constants_float!(f32, f64);

// Implement predicates for all floating-point scalars (is_zero, is_nan, is_infinite, is_finite)
scalar::_impl_scalar_predicates_float!(f32, f64);

// Implement predicates for all integer scalars (is_zero, is_finite, is_infinite, is_nan)
scalar::_impl_scalar_predicates_int!(
    usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
);

// Implement Bounded for all scalar types (MIN, MAX).
scalar::_impl_scalar_bounded!(
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

// Implement Min/Max for all scalar types (minimum, maximum).
scalar::_impl_scalar_min_max!(
    usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64
);

// Implement clamping for all scalar types.
scalar::_impl_scalar_clamp!(
    usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64
);

// Implement signed/unsigned equivalent scalar mappings.
scalar::_impl_scalar_signed_equivalent!(
    usize => isize,
    isize => isize,
    u8 => i8,
    i8 => i8,
    u16 => i16,
    i16 => i16,
    u32 => i32,
    i32 => i32,
    u64 => i64,
    i64 => i64,
    u128 => i128,
    i128 => i128
);

scalar::_impl_scalar_unsigned_equivalent!(
    usize => usize,
    isize => usize,
    u8 => u8,
    i8 => u8,
    u16 => u16,
    i16 => u16,
    u32 => u32,
    i32 => u32,
    u64 => u64,
    i64 => u64,
    u128 => u128,
    i128 => u128
);

// Implement saturating arithmetic for integer scalar types.
scalar::_impl_scalar_arithmetic_trait!(SaturatingAdd, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, saturating_add, output: Self});
scalar::_impl_scalar_arithmetic_trait!(SaturatingSub, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, saturating_sub, output: Self});
scalar::_impl_scalar_arithmetic_trait!(SaturatingMul, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, saturating_mul, output: Self});
scalar::_impl_scalar_arithmetic_trait!(SaturatingDiv, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, saturating_div, output: Self});
scalar::_impl_scalar_arithmetic_trait!(SaturatingNeg, [isize, i8, i16, i32, i64, i128], {unary, saturating_neg, output: Self});

// Implement wrapping arithmetic for integer scalar types.
scalar::_impl_scalar_arithmetic_trait!(WrappingAdd, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, wrapping_add, output: Self});
scalar::_impl_scalar_arithmetic_trait!(WrappingSub, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, wrapping_sub, output: Self});
scalar::_impl_scalar_arithmetic_trait!(WrappingMul, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, wrapping_mul, output: Self});
scalar::_impl_scalar_arithmetic_trait!(WrappingDiv, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, wrapping_div, output: Self});
scalar::_impl_scalar_arithmetic_trait!(WrappingRem, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, wrapping_rem, output: Self});
scalar::_impl_scalar_arithmetic_trait!(WrappingNeg, [isize, i8, i16, i32, i64, i128], {unary, wrapping_neg, output: Self});

// Implement checked arithmetic for integer scalar types.
scalar::_impl_scalar_arithmetic_trait!(CheckedAdd, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, checked_add, output: Option<Self>});
scalar::_impl_scalar_arithmetic_trait!(CheckedSub, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, checked_sub, output: Option<Self>});
scalar::_impl_scalar_arithmetic_trait!(CheckedMul, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, checked_mul, output: Option<Self>});
scalar::_impl_scalar_arithmetic_trait!(CheckedDiv, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, checked_div, output: Option<Self>});
scalar::_impl_scalar_arithmetic_trait!(CheckedRem, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128], {binary, checked_rem, output: Option<Self>});
scalar::_impl_scalar_arithmetic_trait!(CheckedNeg, [isize, i8, i16, i32, i64, i128], {unary, checked_neg, output: Option<Self>});

// Implement fused multiply-add for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    MulAdd,
    [f32, f64],
    {ternary, mul_add, Self, Self, [f32: libm::fmaf, f64: libm::fma]}
);

// Implement Abs for signed scalar types.
scalar::_impl_scalar_elementary_trait!(Abs, [f32, f64, i8, i16, i32, i64, i128, isize], {unary, abs});

// Implement Sqrt for floating scalar types.
scalar::_impl_scalar_elementary_trait!(Sqrt, [f32, f64], {unary, sqrt});

// Implement Trigonometry for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
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
scalar::_impl_scalar_elementary_trait!(
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
scalar::_impl_scalar_elementary_trait!(
    Power,
    [f32, f64],
    {binary, powf, Self, [f32: libm::powf, f64: libm::pow]},
    {binary, powi, i32, [f32: |x: f32, n: i32| libm::powf(x, n as f32), f64: |x: f64, n: i32| libm::pow(x, n as f64)]}
);

// Implement Cbrt (cube root) for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    Cbrt,
    [f32, f64],
    {unary, cbrt, [f32: libm::cbrtf, f64: libm::cbrt]});

// Implement Hyperbolic for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
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
scalar::_impl_scalar_elementary_trait!(
    Hypot,
    [f32, f64],
    {binary, hypot, Self, [f32: libm::hypotf, f64: libm::hypot]}
);

// Implement Floor for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    Floor,
    [f32, f64],
    {unary, floor, [f32: libm::floorf, f64: libm::floor]}
);

// Implement Ceil for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    Ceil,
    [f32, f64],
    {unary, ceil, [f32: libm::ceilf, f64: libm::ceil]}
);

// Implement Trunc for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    Trunc,
    [f32, f64],
    {unary, trunc, [f32: libm::truncf, f64: libm::trunc]}
);

// Implement Round for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    Round,
    [f32, f64],
    {unary, round, [f32: libm::roundf, f64: libm::round]}
);

// Implement Fract for floating scalar types.
scalar::_impl_scalar_elementary_trait!(
    Fract,
    [f32, f64],
    {
        unary,
        fract,
        [f32: |x| libm::modff(x).0, f64: |x| libm::modf(x).0]
    }
);
