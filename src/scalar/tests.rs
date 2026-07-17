#![cfg(test)]

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel, Lerp},
    approx_ne_abs, approx_ne_rel, assert_approx_eq_abs, assert_approx_eq_rel, assert_approx_ne_abs,
    assert_approx_ne_rel,
    numeric::{
        Abs, BoundedMax, BoundedMin, Cast, CastError, CastFrom, Cbrt, Ceil, CheckedAdd, CheckedDiv,
        CheckedMul, CheckedNeg, CheckedRem, CheckedSub, Exponential, Floor, Fract, Half,
        Hyperbolic, Hypot, Infinite, IsFinite, IsInfinite, IsNan, IsZero, LossyCast, LossyCastFrom,
        MinMax, MulAdd, Nan, NegOne, One, Power, Round, Rounding, SaturatingAdd, SaturatingCast,
        SaturatingCastFrom, SaturatingDiv, SaturatingMul, SaturatingNeg, SaturatingSub, SignedCast,
        SignedCastFrom, Sqrt, Trigonometry, Trunc, TryCast, TryCastFrom, TryExactCast,
        TryExactCastFrom, Two, UnsignedCast, UnsignedCastFrom, WrappingAdd, WrappingDiv,
        WrappingMul, WrappingNeg, WrappingRem, WrappingSub, Zero,
    },
    scalar::{Float, Int, Scalar, Signed, SignedInt, Unsigned, UnsignedInt},
};

#[test]
fn marker_traits_surface() {
    fn assert_scalar<T: Scalar>() {}
    fn assert_signed<T: Signed>() {}
    fn assert_unsigned<T: Unsigned>() {}
    fn assert_float<T: Float>() {}
    fn assert_int<T: Int>() {}
    fn assert_signed_int<T: SignedInt>() {}
    fn assert_unsigned_int<T: UnsignedInt>() {}

    assert_scalar::<i32>();
    assert_scalar::<u32>();
    assert_scalar::<f32>();

    assert_signed::<i32>();
    assert_unsigned::<u32>();
    assert_float::<f32>();
    assert_float::<f64>();

    assert_int::<i32>();
    assert_int::<u32>();
    assert_signed_int::<i32>();
    assert_unsigned_int::<u32>();
}

#[test]
fn constants_surface() {
    assert_eq!(<i32 as Zero>::ZERO, 0);
    assert_eq!(<i32 as One>::ONE, 1);
    assert_eq!(<i32 as Two>::TWO, 2);
    assert_eq!(<i32 as NegOne>::NEG_ONE, -1);

    assert_eq!(<u64 as Zero>::ZERO, 0);
    assert_eq!(<u64 as One>::ONE, 1);
    assert_eq!(<u64 as Two>::TWO, 2);

    assert_eq!(<f32 as Zero>::ZERO, 0.0);
    assert_eq!(<f32 as One>::ONE, 1.0);
    assert_eq!(<f32 as Two>::TWO, 2.0);
    assert_eq!(<f32 as NegOne>::NEG_ONE, -1.0);
    assert_eq!(<f32 as Half>::HALF, 0.5);

    assert_eq!(<f64 as Half>::HALF, 0.5);
}

#[test]
fn bounded_surface() {
    assert_eq!(<i32 as BoundedMin>::MIN, i32::MIN);
    assert_eq!(<i32 as BoundedMax>::MAX, i32::MAX);

    assert_eq!(<u64 as BoundedMin>::MIN, u64::MIN);
    assert_eq!(<u64 as BoundedMax>::MAX, u64::MAX);

    assert_eq!(<f32 as BoundedMin>::MIN, f32::MIN);
    assert_eq!(<f32 as BoundedMax>::MAX, f32::MAX);
}

#[test]
fn elementary_surface() {
    assert_approx_eq_abs!(<f32 as MulAdd>::mul_add(2.0, 3.0, 4.0), 10.0, 1e-6);
    assert_approx_eq_abs!(<f64 as MulAdd>::mul_add(1.5, 2.0, 0.5), 3.5, 1e-12);

    assert_eq!(<i32 as MinMax>::minimum(3, 9), 3);
    assert_eq!(<i32 as MinMax>::maximum(3, 9), 9);

    assert_eq!(<u64 as MinMax>::minimum(10, 4), 4);
    assert_eq!(<u64 as MinMax>::maximum(10, 4), 10);

    assert_eq!(<f32 as MinMax>::minimum(1.25, -2.0), -2.0);
    assert_eq!(<f32 as MinMax>::maximum(1.25, -2.0), 1.25);

    assert_eq!(<f32 as Abs>::abs(-3.5), 3.5);
    assert_eq!(<f64 as Abs>::abs(-7.25), 7.25);

    assert_eq!(<i8 as Abs>::abs(-7), 7);
    assert_eq!(<i16 as Abs>::abs(-123), 123);
    assert_eq!(<i32 as Abs>::abs(-4567), 4567);
    assert_eq!(<i64 as Abs>::abs(-89012), 89012);
    assert_eq!(<i128 as Abs>::abs(-345678), 345678);
    assert_eq!(<isize as Abs>::abs(-42), 42);

    assert_eq!(<f32 as Sqrt>::sqrt(9.0), 3.0);
    assert_eq!(<f64 as Sqrt>::sqrt(2.25), 1.5);

    let x = 0.5_f32;
    let (s, c) = <f32 as Trigonometry>::sin_cos(x);
    assert_approx_eq_abs!(<f32 as Trigonometry>::sin(x), s, 1e-6);
    assert_approx_eq_abs!(<f32 as Trigonometry>::cos(x), c, 1e-6);
    assert_approx_eq_abs!(<f32 as Trigonometry>::tan(x), s / c, 1e-5);
    assert_approx_eq_abs!(<f32 as Trigonometry>::sin(<f32 as Trigonometry>::asin(x)), x, 1e-6);
    assert_approx_eq_abs!(<f32 as Trigonometry>::cos(<f32 as Trigonometry>::acos(x)), x, 1e-6);
    assert_approx_eq_abs!(<f32 as Trigonometry>::tan(<f32 as Trigonometry>::atan(x)), x, 1e-6);
    assert_approx_eq_abs!(
        <f32 as Trigonometry>::atan2(1.0, 1.0),
        core::f32::consts::FRAC_PI_4,
        1e-6
    );

    let e = <f32 as Exponential>::exp(1.0);
    assert_approx_eq_abs!(e, core::f32::consts::E, 1e-6);
    assert_approx_eq_abs!(<f32 as Exponential>::exp2(4.0), 16.0, 1e-6);
    assert_approx_eq_abs!(
        <f32 as Exponential>::exp_m1(1e-4),
        <f32 as Exponential>::exp(1e-4) - 1.0,
        1e-6
    );
    assert_approx_eq_abs!(<f32 as Exponential>::ln(core::f32::consts::E), 1.0, 1e-6);
    assert_approx_eq_abs!(
        <f32 as Exponential>::ln_1p(1e-4),
        <f32 as Exponential>::ln(1.0001),
        1e-6
    );
    assert_approx_eq_abs!(<f32 as Exponential>::log(8.0, 2.0), 3.0, 1e-6);
    assert_approx_eq_abs!(<f32 as Exponential>::log2(8.0), 3.0, 1e-6);
    assert_approx_eq_abs!(<f32 as Exponential>::log10(1000.0), 3.0, 1e-6);

    assert_approx_eq_abs!(<f32 as Power>::powf(9.0, 0.5), 3.0, 1e-6);
    assert_approx_eq_abs!(<f32 as Power>::powi(3.0, 4), 81.0, 1e-6);

    assert_approx_eq_abs!(<f32 as Cbrt>::cbrt(27.0), 3.0, 1e-6);
    assert_approx_eq_abs!(<f32 as Hypot>::hypot(3.0, 4.0), 5.0, 1e-6);

    let hs = <f32 as Hyperbolic>::sinh(x);
    let hc = <f32 as Hyperbolic>::cosh(x);
    assert_approx_eq_abs!(<f32 as Hyperbolic>::tanh(x), hs / hc, 1e-6);
    assert_approx_eq_abs!(<f32 as Hyperbolic>::asinh(hs), x, 1e-5);
    assert_approx_eq_abs!(<f32 as Hyperbolic>::acosh(<f32 as Hyperbolic>::cosh(2.0)), 2.0, 1e-5);
    assert_approx_eq_abs!(<f32 as Hyperbolic>::atanh(<f32 as Hyperbolic>::tanh(0.25)), 0.25, 1e-5);
}

#[test]
fn approx_eq_surface() {
    assert!(approx_ne_abs!(1.0_f32, 1.1_f32, 0.00001_f32));
    assert!(!approx_ne_abs!(1.0_f32, 1.000001_f32, 0.00001_f32));

    assert!(approx_ne_rel!(1.0_f32, 2.0_f32, 0.00001_f32));
    assert!(!approx_ne_rel!(1000.0_f32, 1000.001_f32, 0.00001_f32));

    assert!(approx_ne_abs!(1.0_f32, 1.0_f32 + 2e-6));
    assert!(!approx_ne_abs!(1.0_f32, 1.0_f32 + 5e-7));

    assert!(approx_ne_rel!(100_000.0_f32, 100_002.0_f32));
    assert!(!approx_ne_rel!(100_000.0_f32, 100_000.5_f32));

    assert_approx_eq_abs!(1.0_f32, 1.000001_f32, 0.00001_f32);
    assert_approx_ne_abs!(1.0_f32, 1.1_f32, 0.00001_f32);

    assert_approx_eq_rel!(1000.0_f32, 1000.001_f32, 0.00001_f32);
    assert_approx_ne_rel!(1.0_f32, 2.0_f32, 0.00001_f32);

    // Default abs tolerance (f32: 1e-6)
    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7);
    assert_approx_ne_abs!(1.0_f32, 1.0_f32 + 2e-6);

    // Default rel tolerance (f32: 1e-5)
    assert_approx_eq_rel!(100_000.0_f32, 100_000.5_f32);
    assert_approx_ne_rel!(100_000.0_f32, 100_002.0_f32);

    assert_eq!(<f32 as ApproxEqAbs>::DEFAULT_TOLERANCE_ABS, 1e-6);
    assert_eq!(<f64 as ApproxEqAbs>::DEFAULT_TOLERANCE_ABS, 1e-12);
    assert_eq!(<f32 as ApproxEqRel>::DEFAULT_TOLERANCE_REL, 1e-5);
    assert_eq!(<f64 as ApproxEqRel>::DEFAULT_TOLERANCE_REL, 1e-10);

    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7);
    assert_approx_ne_abs!(1.0_f32, 1.0_f32 + 2e-6);
    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7, 1e-6);
    assert_approx_ne_abs!(1.0_f32, 1.0_f32 + 2e-4, 1e-6);

    assert_approx_eq_rel!(100_000.0_f32, 100_000.5_f32);
    assert_approx_ne_rel!(100_000.0_f32, 100_002.0_f32);
    assert_approx_eq_rel!(1000.0_f32, 1000.001_f32, 1e-5);
    assert_approx_ne_rel!(1.0_f32, 2.0_f32, 1e-5);

    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7);
    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7, 1e-6_f32);
    assert_approx_eq_rel!(100_000.0_f32, 100_000.5_f32);
    assert_approx_eq_rel!(1000.0_f32, 1000.001_f32, 1e-5_f32);
}

#[test]
#[should_panic]
fn approx_eq_assert_abs_panics() {
    assert_approx_eq_abs!(1.0_f32, 2.0_f32);
}

#[test]
#[should_panic(expected = "custom abs message")]
fn approx_eq_assert_abs_message_panics() {
    assert_approx_eq_abs!(1.0_f32, 2.0_f32, message = "custom abs message");
}

#[test]
#[should_panic]
fn approx_eq_assert_abs_tol_panics() {
    assert_approx_eq_abs!(1.0_f32, 2.0_f32, 1e-6_f32);
}

#[test]
#[should_panic]
fn approx_eq_assert_rel_panics() {
    assert_approx_eq_rel!(1.0_f32, 2.0_f32);
}

#[test]
#[should_panic]
fn approx_eq_assert_rel_tol_panics() {
    assert_approx_eq_rel!(1.0_f32, 2.0_f32, 1e-5_f32);
}

#[test]
#[should_panic(expected = "custom rel tol message")]
fn approx_eq_assert_rel_tol_message_panics() {
    assert_approx_eq_rel!(1.0_f32, 2.0_f32, 1e-5_f32, message = "custom rel tol message");
}

#[test]
#[should_panic]
fn approx_ne_assert_abs_panics() {
    assert_approx_ne_abs!(1.0_f32, 1.0_f32 + 5e-7);
}

#[test]
#[should_panic]
fn approx_ne_assert_abs_tol_panics() {
    assert_approx_ne_abs!(1.0_f32, 1.0_f32 + 5e-7, 1e-6_f32);
}

#[test]
#[should_panic]
fn approx_ne_assert_rel_panics() {
    assert_approx_ne_rel!(100_000.0_f32, 100_000.5_f32);
}

#[test]
#[should_panic]
fn approx_ne_assert_rel_tol_panics() {
    assert_approx_ne_rel!(1000.0_f32, 1000.001_f32, 1e-5_f32);
}

#[test]
fn predicates_surface() {
    fn zero<T: IsZero>(x: T) -> bool {
        x.is_zero()
    }

    fn finite<T: IsFinite>(x: T) -> bool {
        x.is_finite()
    }

    fn infinite<T: IsInfinite>(x: T) -> bool {
        x.is_infinite()
    }

    fn nan<T: IsNan>(x: T) -> bool {
        x.is_nan()
    }

    // Zero
    assert!(zero(0_i32));
    assert!(!zero(1_i32));
    assert!(<i32 as IsZero>::is_zero(0));
    assert!(!<i32 as IsZero>::is_zero(42));

    assert!(zero(0_u64));
    assert!(!zero(7_u64));
    assert!(<u64 as IsZero>::is_zero(0));
    assert!(!<u64 as IsZero>::is_zero(7));

    assert!(zero(0.0_f32));
    assert!(zero(-0.0_f32));
    assert!(!zero(1.0_f32));
    assert!(<f32 as IsZero>::is_zero(0.0));
    assert!(!<f32 as IsZero>::is_zero(0.5));

    // Finite
    assert!(finite(1.0_f32));
    assert!(finite(1_u32));

    assert!(!finite(<f32 as Infinite>::INFINITY));
    assert!(!finite(<f32 as Nan>::NAN));

    // Infinite
    assert!(infinite(<f32 as Infinite>::INFINITY));
    assert!(infinite(<f32 as Infinite>::NEG_INFINITY));

    assert!(!infinite(<f32 as Nan>::NAN));
    assert!(!infinite(1_u32));
    assert!(!infinite(1.0_f32));

    // NaN
    assert!(nan(<f32 as Nan>::NAN));

    assert!(!nan(1_u32));
    assert!(!nan(1.0_f32));
    assert!(!nan(<f32 as Infinite>::INFINITY));
}

#[test]
fn rounding_surface() {
    fn assert_rounding<T: Rounding>() {}
    assert_rounding::<f32>();
    assert_rounding::<f64>();

    let x = 1.75_f32;
    let y = -1.75_f32;

    assert_eq!(<f32 as Floor>::floor(x), 1.0);
    assert_eq!(<f32 as Floor>::floor(y), -2.0);

    assert_eq!(<f32 as Ceil>::ceil(x), 2.0);
    assert_eq!(<f32 as Ceil>::ceil(y), -1.0);

    assert_eq!(<f32 as Trunc>::trunc(x), 1.0);
    assert_eq!(<f32 as Trunc>::trunc(y), -1.0);

    assert_eq!(<f32 as Round>::round(1.6), 2.0);
    assert_eq!(<f32 as Round>::round(-1.6), -2.0);

    assert_eq!(<f32 as Fract>::fract(1.25), 0.25);
    assert_eq!(<f32 as Fract>::fract(-1.25), -0.25);
}

#[test]
fn lerp_surface() {
    let a32 = 10.0_f32;
    let b32 = 20.0_f32;
    assert_approx_eq_abs!(a32.lerp(b32, 0.25), 12.5_f32, 1e-6);
    assert_approx_eq_abs!(<f32 as Lerp>::lerp(a32, b32, 0.25), 12.5_f32, 1e-6);

    let a64 = -4.0_f64;
    let b64 = 6.0_f64;
    assert_approx_eq_abs!(a64.lerp(b64, 0.5), 1.0_f64, 1e-12);
    assert_approx_eq_abs!(<f64 as Lerp>::lerp(a64, b64, 0.5), 1.0_f64, 1e-12);
}

#[test]
fn arithmetic_traits_surface() {
    assert_eq!(<u8 as SaturatingAdd>::saturating_add(250, 10), u8::MAX);
    assert_eq!(<i8 as SaturatingSub>::saturating_sub(-120, 20), i8::MIN);
    assert_eq!(<u8 as SaturatingMul>::saturating_mul(40, 10), u8::MAX);
    assert_eq!(<u8 as SaturatingDiv>::saturating_div(10, 2), 5);
    assert_eq!(<i8 as SaturatingNeg>::saturating_neg(1), -1);
    assert_eq!(<i8 as SaturatingNeg>::saturating_neg(i8::MIN), i8::MAX);

    assert_eq!(<u8 as WrappingAdd>::wrapping_add(255, 1), 0);
    assert_eq!(<u8 as WrappingSub>::wrapping_sub(0, 1), u8::MAX);
    assert_eq!(<u8 as WrappingMul>::wrapping_mul(200, 2), 144);
    assert_eq!(<u8 as WrappingDiv>::wrapping_div(10, 2), 5);
    assert_eq!(<u8 as WrappingRem>::wrapping_rem(10, 3), 1);
    assert_eq!(<i8 as WrappingNeg>::wrapping_neg(1), -1);
    assert_eq!(<i8 as WrappingNeg>::wrapping_neg(i8::MIN), i8::MIN);

    assert_eq!(<u8 as CheckedAdd>::checked_add(250, 10), None);
    assert_eq!(<u8 as CheckedAdd>::checked_add(10, 20), Some(30));
    assert_eq!(<u8 as CheckedSub>::checked_sub(0, 1), None);
    assert_eq!(<u8 as CheckedSub>::checked_sub(10, 3), Some(7));
    assert_eq!(<u8 as CheckedMul>::checked_mul(20, 20), None);
    assert_eq!(<u8 as CheckedMul>::checked_mul(10, 20), Some(200));
    assert_eq!(<u8 as CheckedDiv>::checked_div(10, 0), None);
    assert_eq!(<u8 as CheckedDiv>::checked_div(10, 2), Some(5));
    assert_eq!(<u8 as CheckedRem>::checked_rem(10, 0), None);
    assert_eq!(<u8 as CheckedRem>::checked_rem(10, 3), Some(1));
    assert_eq!(<i8 as CheckedNeg>::checked_neg(0), Some(0));
    assert_eq!(<i8 as CheckedNeg>::checked_neg(5), Some(-5));
    assert_eq!(<i8 as CheckedNeg>::checked_neg(i8::MIN), None);
}

#[test]
fn cast_surface() {
    // Identity casts
    assert_eq!(<i32 as Cast<i32>>::cast(42), 42_i32);
    assert_eq!(<f32 as Cast<f32>>::cast(1.5), 1.5_f32);
    assert_eq!(<f32 as Cast<f64>>::cast(0.5_f32), 0.5_f64);
    assert_eq!(<f64 as Cast<f64>>::cast(-7.5), -7.5_f64);

    // Unsigned int widening
    assert_eq!(<u8 as Cast<u16>>::cast(200), 200_u16);
    assert_eq!(<u8 as Cast<u32>>::cast(255), 255_u32);
    assert_eq!(<u8 as Cast<u64>>::cast(1), 1_u64);
    assert_eq!(<u8 as Cast<u128>>::cast(0), 0_u128);
    assert_eq!(<u16 as Cast<u32>>::cast(1000), 1000_u32);
    assert_eq!(<u32 as Cast<u64>>::cast(u32::MAX), u32::MAX as u64);
    assert_eq!(<u64 as Cast<u128>>::cast(u64::MAX), u64::MAX as u128);

    // Signed int widening
    assert_eq!(<i8 as Cast<i16>>::cast(-100), -100_i16);
    assert_eq!(<i8 as Cast<i32>>::cast(-1), -1_i32);
    assert_eq!(<i8 as Cast<i64>>::cast(127), 127_i64);
    assert_eq!(<i8 as Cast<i128>>::cast(-128), -128_i128);
    assert_eq!(<i16 as Cast<i32>>::cast(-32768), -32768_i32);
    assert_eq!(<i32 as Cast<i64>>::cast(i32::MIN), i32::MIN as i64);
    assert_eq!(<i64 as Cast<i128>>::cast(i64::MAX), i64::MAX as i128);
}

#[test]
fn lossy_cast_surface() {
    // Identity casts
    assert_eq!(<i32 as LossyCast<i32>>::lossy_cast(42), 42_i32);
    assert_eq!(<f32 as LossyCast<f32>>::lossy_cast(1.5), 1.5_f32);

    // Widening int casts
    assert_eq!(<u8 as LossyCast<u32>>::lossy_cast(200), 200_u32);
    assert_eq!(<i8 as LossyCast<i32>>::lossy_cast(-100), -100_i32);
    assert_eq!(<i32 as LossyCast<i64>>::lossy_cast(-1_000_000), -1_000_000_i64);
    assert_eq!(<u32 as LossyCast<u64>>::lossy_cast(4_000_000_000), 4_000_000_000_u64);

    // Widening float cast
    assert_eq!(<f32 as LossyCast<f64>>::lossy_cast(1.5_f32), 1.5_f64);

    // Narrowing int casts
    assert_eq!(<u32 as LossyCast<u8>>::lossy_cast(256), 0_u8);
    assert_eq!(<u32 as LossyCast<u8>>::lossy_cast(257), 1_u8);
    assert_eq!(<i32 as LossyCast<i8>>::lossy_cast(128), -128_i8);

    // Signed/unsigned int casts
    assert_eq!(<i32 as LossyCast<u32>>::lossy_cast(-1), u32::MAX);
    assert_eq!(<u32 as LossyCast<i32>>::lossy_cast(u32::MAX), -1_i32);
    assert_eq!(<i8 as LossyCast<u8>>::lossy_cast(-1_i8), 255_u8);

    // Narrowing float cast
    let v: f32 = <f64 as LossyCast<f32>>::lossy_cast(1.5_f64);
    assert_approx_eq_abs!(v, 1.5_f32, 1e-6);

    // Float-to-int casts
    assert_eq!(<f32 as LossyCast<i32>>::lossy_cast(3.9_f32), 3_i32);
    assert_eq!(<f32 as LossyCast<i32>>::lossy_cast(-3.9_f32), -3_i32);
    assert_eq!(<f64 as LossyCast<u64>>::lossy_cast(255.99_f64), 255_u64);

    // Int-to-float casts
    assert_eq!(<i32 as LossyCast<f32>>::lossy_cast(100), 100.0_f32);
    assert_eq!(<i32 as LossyCast<f64>>::lossy_cast(-1000), -1000.0_f64);
    assert_eq!(<u64 as LossyCast<f64>>::lossy_cast(1024), 1024.0_f64);
}

#[test]
fn saturating_cast_surface() {
    // unsigned -> unsigned
    assert_eq!(<u32 as SaturatingCast<u32>>::saturating_cast(42), 42);
    assert_eq!(<u32 as SaturatingCast<u8>>::saturating_cast(0), 0_u8);
    assert_eq!(<u32 as SaturatingCast<u8>>::saturating_cast(255), 255_u8);
    assert_eq!(<u32 as SaturatingCast<u8>>::saturating_cast(256), u8::MAX);
    assert_eq!(<u8 as SaturatingCast<u64>>::saturating_cast(255), 255_u64);
    assert_eq!(<u32 as SaturatingCast<u64>>::saturating_cast(u32::MAX), u32::MAX as u64);

    // signed -> signed
    assert_eq!(<i32 as SaturatingCast<i32>>::saturating_cast(-5), -5);
    assert_eq!(<i32 as SaturatingCast<i8>>::saturating_cast(-200), i8::MIN);
    assert_eq!(<i32 as SaturatingCast<i8>>::saturating_cast(-128), -128_i8);
    assert_eq!(<i32 as SaturatingCast<i8>>::saturating_cast(127), 127_i8);
    assert_eq!(<i32 as SaturatingCast<i8>>::saturating_cast(200), i8::MAX);
    assert_eq!(<i8 as SaturatingCast<i64>>::saturating_cast(-128), -128_i64);
    assert_eq!(<i16 as SaturatingCast<i64>>::saturating_cast(i16::MIN), i16::MIN as i64);

    // unsigned -> signed
    assert_eq!(<u8 as SaturatingCast<i16>>::saturating_cast(255), 255_i16);
    assert_eq!(<u16 as SaturatingCast<i32>>::saturating_cast(u16::MAX), u16::MAX as i32);
    assert_eq!(<u32 as SaturatingCast<i32>>::saturating_cast(i32::MAX as u32 + 1), i32::MAX);
    assert_eq!(<u32 as SaturatingCast<i16>>::saturating_cast(0), 0_i16);
    assert_eq!(<u32 as SaturatingCast<i16>>::saturating_cast(i16::MAX as u32), i16::MAX);
    assert_eq!(<u32 as SaturatingCast<i16>>::saturating_cast((i16::MAX as u32) + 1), i16::MAX);

    // signed -> unsigned
    assert_eq!(<i8 as SaturatingCast<u16>>::saturating_cast(-1), 0_u16);
    assert_eq!(<i8 as SaturatingCast<u16>>::saturating_cast(127), 127_u16);
    assert_eq!(<i32 as SaturatingCast<u32>>::saturating_cast(i32::MAX), i32::MAX as u32);
    assert_eq!(<i32 as SaturatingCast<u32>>::saturating_cast(-10), 0_u32);
    assert_eq!(<i32 as SaturatingCast<u16>>::saturating_cast(-100), u16::MIN);
    assert_eq!(<i32 as SaturatingCast<u16>>::saturating_cast(0), 0_u16);
    assert_eq!(<i32 as SaturatingCast<u16>>::saturating_cast(u16::MAX as i32), u16::MAX);
    assert_eq!(<i32 as SaturatingCast<u16>>::saturating_cast((u16::MAX as i32) + 1), u16::MAX);

    // int -> float (no clamp, same as as cast)
    assert_eq!(<u64 as SaturatingCast<f32>>::saturating_cast(1), 1.0_f32);
    assert_eq!(<u64 as SaturatingCast<f32>>::saturating_cast(16_777_217), 16_777_216.0_f32);

    // float -> int
    assert_eq!(<f32 as SaturatingCast<u8>>::saturating_cast(f32::NAN), 0_u8);
    assert_eq!(<f32 as SaturatingCast<u8>>::saturating_cast(f32::NEG_INFINITY), u8::MIN);
    assert_eq!(<f32 as SaturatingCast<u8>>::saturating_cast(-10.5), u8::MIN);
    assert_eq!(<f32 as SaturatingCast<u8>>::saturating_cast(10.9), 10_u8);
    assert_eq!(<f32 as SaturatingCast<u8>>::saturating_cast(255.9), u8::MAX);
    assert_eq!(<f32 as SaturatingCast<u8>>::saturating_cast(f32::INFINITY), u8::MAX);
    assert_eq!(<f32 as SaturatingCast<i8>>::saturating_cast(200.0), i8::MAX);
    assert_eq!(<f32 as SaturatingCast<i8>>::saturating_cast(-200.0), i8::MIN);

    // float -> float
    assert_eq!(<f64 as SaturatingCast<f32>>::saturating_cast(f64::NAN), 0.0_f32);
    assert_eq!(<f64 as SaturatingCast<f32>>::saturating_cast(f64::NEG_INFINITY), f32::MIN);
    assert_eq!(<f64 as SaturatingCast<f32>>::saturating_cast(f64::INFINITY), f32::MAX);
    assert_approx_eq_abs!(<f64 as SaturatingCast<f32>>::saturating_cast(1.5_f64), 1.5_f32, 1e-6);
    assert_eq!(<f64 as TryExactCast<f32>>::try_exact_cast(16777217.0), Err(CastError::Inexact));
}

#[test]
fn signed_unsigned_cast_surface() {
    assert_eq!(<u8 as SignedCast<i8>>::signed_cast(255_u8), -1_i8);
    assert_eq!(<usize as SignedCast<isize>>::signed_cast(42_usize), 42_isize);
    assert_eq!(<i8 as UnsignedCast<u8>>::unsigned_cast(-1_i8), 255_u8);
    assert_eq!(<isize as UnsignedCast<usize>>::unsigned_cast(42_isize), 42_usize);
}

#[test]
fn try_cast_surface() {
    // int-to-int: success cases
    assert_eq!(<i32 as TryCast<i32>>::try_cast(42), Ok(42_i32));
    assert_eq!(<u8 as TryCast<u32>>::try_cast(200), Ok(200_u32));
    assert_eq!(<i32 as TryCast<i64>>::try_cast(-1_000_000), Ok(-1_000_000_i64));
    assert_eq!(<i32 as TryCast<u32>>::try_cast(0), Ok(0_u32));
    assert_eq!(<i32 as TryCast<u32>>::try_cast(i32::MAX), Ok(i32::MAX as u32));

    // int-to-int: failure cases
    // Same-size casts (e.g. i32 <-> u32) always roundtrip at the bit level, so failures
    // only occur when narrowing to a smaller type and the value is out of range.
    assert_eq!(<i32 as TryCast<u8>>::try_cast(-1), Err(CastError::OutOfRange));
    assert_eq!(<i32 as TryCast<u8>>::try_cast(256), Err(CastError::OutOfRange));
    assert_eq!(<i32 as TryCast<i8>>::try_cast(128), Err(CastError::OutOfRange));
    assert_eq!(<i64 as TryCast<u8>>::try_cast(-1), Err(CastError::OutOfRange));
    assert_eq!(<u64 as TryCast<i32>>::try_cast(2_147_483_648_u64), Err(CastError::OutOfRange));

    // int-to-float: always succeeds
    assert!(<i32 as TryCast<f32>>::try_cast(100).is_ok());
    assert!(<u64 as TryCast<f64>>::try_cast(1024).is_ok());
    assert!(<i128 as TryCast<f32>>::try_cast(i128::MAX).is_ok()); // lossy but Ok

    // float-to-int: success cases
    assert_eq!(<f32 as TryCast<i32>>::try_cast(42.0), Ok(42_i32));
    assert_eq!(<f64 as TryCast<u8>>::try_cast(255.0), Ok(255_u8));
    assert_eq!(<f32 as TryCast<i32>>::try_cast(-10.9), Ok(-10_i32)); // truncates

    // float-to-int: failure cases
    assert_eq!(<f32 as TryCast<i32>>::try_cast(f32::INFINITY), Err(CastError::NonFinite));
    assert_eq!(<f32 as TryCast<i32>>::try_cast(f32::NAN), Err(CastError::NonFinite));
    assert_eq!(<f32 as TryCast<u8>>::try_cast(256.0), Err(CastError::OutOfRange));
    assert_eq!(<f64 as TryCast<i8>>::try_cast(-200.0), Err(CastError::OutOfRange));

    // float-to-float: success cases
    assert!(<f32 as TryCast<f64>>::try_cast(1.5).is_ok());
    assert!(<f64 as TryCast<f32>>::try_cast(1.5).is_ok());
}

#[test]
fn try_exact_cast_surface() {
    // int-to-int: success cases
    assert_eq!(<i32 as TryExactCast<i32>>::try_exact_cast(42), Ok(42_i32));
    assert_eq!(<u8 as TryExactCast<u32>>::try_exact_cast(200), Ok(200_u32));
    assert_eq!(<i32 as TryExactCast<i64>>::try_exact_cast(-1_000_000), Ok(-1_000_000_i64));

    // int-to-int: failure cases
    assert_eq!(<i32 as TryExactCast<u8>>::try_exact_cast(-1), Err(CastError::OutOfRange));
    assert_eq!(<i32 as TryExactCast<u8>>::try_exact_cast(256), Err(CastError::OutOfRange));
    assert_eq!(
        <u64 as TryExactCast<i32>>::try_exact_cast(2_147_483_648_u64),
        Err(CastError::OutOfRange)
    );

    // int-to-float: success when exact
    assert_eq!(<i32 as TryExactCast<f32>>::try_exact_cast(1), Ok(1.0_f32));
    assert_eq!(<i32 as TryExactCast<f64>>::try_exact_cast(i32::MAX), Ok(i32::MAX as f64));
    assert_eq!(<u8 as TryExactCast<f32>>::try_exact_cast(255), Ok(255.0_f32));

    // int-to-float: failure when precision lost
    // f32 has 24 bits of mantissa; 2^24+1 = 16_777_217 is the first integer not exactly representable.
    assert_eq!(<i32 as TryExactCast<f32>>::try_exact_cast(16_777_217), Err(CastError::Inexact));
    assert_eq!(<i64 as TryExactCast<f32>>::try_exact_cast(16_777_217_i64), Err(CastError::Inexact));

    // float-to-int: success when whole number in range
    assert_eq!(<f32 as TryExactCast<i32>>::try_exact_cast(42.0), Ok(42_i32));
    assert_eq!(<f64 as TryExactCast<u8>>::try_exact_cast(255.0), Ok(255_u8));
    assert_eq!(<f32 as TryExactCast<i32>>::try_exact_cast(-100.0), Ok(-100_i32));

    // float-to-int: failure cases
    assert_eq!(<f32 as TryExactCast<i32>>::try_exact_cast(f32::NAN), Err(CastError::NonFinite));
    assert_eq!(
        <f32 as TryExactCast<i32>>::try_exact_cast(f32::INFINITY),
        Err(CastError::NonFinite)
    );
    assert_eq!(<f32 as TryExactCast<i32>>::try_exact_cast(1.5), Err(CastError::Fractional));
    assert_eq!(<f64 as TryExactCast<u8>>::try_exact_cast(-0.5), Err(CastError::Fractional));
    assert_eq!(<f32 as TryExactCast<u8>>::try_exact_cast(256.0), Err(CastError::OutOfRange));
    assert_eq!(<f64 as TryExactCast<i8>>::try_exact_cast(-200.0), Err(CastError::OutOfRange));

    // --- float-to-float: success cases ---
    assert_eq!(<f32 as TryExactCast<f64>>::try_exact_cast(1.5_f32), Ok(1.5_f64));
    assert_eq!(<f64 as TryExactCast<f64>>::try_exact_cast(1.5_f64), Ok(1.5_f64));
    assert_eq!(<f32 as TryExactCast<f32>>::try_exact_cast(0.5_f32), Ok(0.5_f32));

    // float-to-float: failure cases
    // f64 value not exactly representable as f32
    assert_eq!(
        <f64 as TryExactCast<f32>>::try_exact_cast(1.0000000001_f64),
        Err(CastError::Inexact)
    );
}

#[test]
fn from_cast_variants_surface() {
    assert_eq!(<u32 as CastFrom<u8>>::cast_from(255_u8), 255_u32);
    assert_eq!(<u8 as LossyCastFrom<i32>>::lossy_cast_from(300_i32), 44_u8);
    assert_eq!(<u8 as SaturatingCastFrom<u32>>::saturating_cast_from(300_u32), u8::MAX);
    assert_eq!(<i8 as SignedCastFrom<u8>>::signed_cast_from(255_u8), -1_i8);
    assert_eq!(<u8 as UnsignedCastFrom<i8>>::unsigned_cast_from(-1_i8), 255_u8);
    assert_eq!(<u8 as TryCastFrom<i32>>::try_cast_from(255_i32), Ok(255_u8));
    assert_eq!(
        <f32 as TryExactCastFrom<i32>>::try_exact_cast_from(16_777_217_i32),
        Err(CastError::Inexact)
    );
}
