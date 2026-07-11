#![cfg(test)]

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel, Lerp},
    approx_eql_abs, approx_eql_abs_tol, approx_eql_rel, approx_eql_rel_tol,
    numeric::{
        Abs, Bounded, Cbrt, Ceil, Exponential, Finite, Floor, Fract, Half, Hyperbolic, Hypot,
        Infinite, MinMax, Nan, NegOne, One, Power, Round, Rounding, Sqrt, Trigonometry, Trunc, Two,
        Zero,
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
    assert_eq!(<i32 as Bounded>::MIN, i32::MIN);
    assert_eq!(<i32 as Bounded>::MAX, i32::MAX);

    assert_eq!(<u64 as Bounded>::MIN, u64::MIN);
    assert_eq!(<u64 as Bounded>::MAX, u64::MAX);

    assert_eq!(<f32 as Bounded>::MIN, f32::MIN);
    assert_eq!(<f32 as Bounded>::MAX, f32::MAX);
}

#[test]
fn elementary_surface() {
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
    assert!(approx_eql_abs_tol!(<f32 as Trigonometry>::sin(x), s, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Trigonometry>::cos(x), c, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Trigonometry>::tan(x), s / c, 1e-5));
    assert!(approx_eql_abs_tol!(
        <f32 as Trigonometry>::sin(<f32 as Trigonometry>::asin(x)),
        x,
        1e-6
    ));
    assert!(approx_eql_abs_tol!(
        <f32 as Trigonometry>::cos(<f32 as Trigonometry>::acos(x)),
        x,
        1e-6
    ));
    assert!(approx_eql_abs_tol!(
        <f32 as Trigonometry>::tan(<f32 as Trigonometry>::atan(x)),
        x,
        1e-6
    ));
    assert!(approx_eql_abs_tol!(
        <f32 as Trigonometry>::atan2(1.0, 1.0),
        core::f32::consts::FRAC_PI_4,
        1e-6
    ));

    let e = <f32 as Exponential>::exp(1.0);
    assert!(approx_eql_abs_tol!(e, core::f32::consts::E, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Exponential>::exp2(4.0), 16.0, 1e-6));
    assert!(approx_eql_abs_tol!(
        <f32 as Exponential>::exp_m1(1e-4),
        <f32 as Exponential>::exp(1e-4) - 1.0,
        1e-6
    ));
    assert!(approx_eql_abs_tol!(<f32 as Exponential>::ln(core::f32::consts::E), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(
        <f32 as Exponential>::ln_1p(1e-4),
        <f32 as Exponential>::ln(1.0001),
        1e-6
    ));
    assert!(approx_eql_abs_tol!(<f32 as Exponential>::log(8.0, 2.0), 3.0, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Exponential>::log2(8.0), 3.0, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Exponential>::log10(1000.0), 3.0, 1e-6));

    assert!(approx_eql_abs_tol!(<f32 as Power>::powf(9.0, 0.5), 3.0, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Power>::powi(3.0, 4), 81.0, 1e-6));

    assert!(approx_eql_abs_tol!(<f32 as Cbrt>::cbrt(27.0), 3.0, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Hypot>::hypot(3.0, 4.0), 5.0, 1e-6));

    let hs = <f32 as Hyperbolic>::sinh(x);
    let hc = <f32 as Hyperbolic>::cosh(x);
    assert!(approx_eql_abs_tol!(<f32 as Hyperbolic>::tanh(x), hs / hc, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Hyperbolic>::asinh(hs), x, 1e-5));
    assert!(approx_eql_abs_tol!(
        <f32 as Hyperbolic>::acosh(<f32 as Hyperbolic>::cosh(2.0)),
        2.0,
        1e-5
    ));
    assert!(approx_eql_abs_tol!(
        <f32 as Hyperbolic>::atanh(<f32 as Hyperbolic>::tanh(0.25)),
        0.25,
        1e-5
    ));
}

#[test]
fn approx_eq_surface() {
    assert!(approx_eql_abs_tol!(1.0_f32, 1.000001_f32, 0.00001_f32));
    assert!(!approx_eql_abs_tol!(1.0_f32, 1.1_f32, 0.00001_f32));

    assert!(approx_eql_rel_tol!(1000.0_f32, 1000.001_f32, 0.00001_f32));
    assert!(!approx_eql_rel_tol!(1.0_f32, 2.0_f32, 0.00001_f32));

    // Default abs tolerance (f32: 1e-6)
    assert!(approx_eql_abs!(1.0_f32, 1.0_f32 + 5e-7));
    assert!(!approx_eql_abs!(1.0_f32, 1.0_f32 + 2e-6));

    // Default rel tolerance (f32: 1e-5)
    assert!(approx_eql_rel!(100_000.0_f32, 100_000.5_f32));
    assert!(!approx_eql_rel!(100_000.0_f32, 100_002.0_f32));

    assert_eq!(<f32 as ApproxEqAbs>::DEFAULT_TOLERANCE_ABS, 1e-6);
    assert_eq!(<f64 as ApproxEqAbs>::DEFAULT_TOLERANCE_ABS, 1e-12);
    assert_eq!(<f32 as ApproxEqRel>::DEFAULT_TOLERANCE_REL, 1e-5);
    assert_eq!(<f64 as ApproxEqRel>::DEFAULT_TOLERANCE_REL, 1e-10);

    assert!(approx_eql_abs!(1.0_f32, 1.0_f32 + 5e-7));
    assert!(!approx_eql_abs!(1.0_f32, 1.0_f32 + 2e-6));
    assert!(approx_eql_abs_tol!(1.0_f32, 1.0_f32 + 5e-7, 1e-6));
    assert!(!approx_eql_abs_tol!(1.0_f32, 1.0_f32 + 2e-4, 1e-6));

    assert!(approx_eql_rel!(100_000.0_f32, 100_000.5_f32));
    assert!(!approx_eql_rel!(100_000.0_f32, 100_002.0_f32));
    assert!(approx_eql_rel_tol!(1000.0_f32, 1000.001_f32, 1e-5));
    assert!(!approx_eql_rel_tol!(1.0_f32, 2.0_f32, 1e-5));
}

#[test]
fn float_predicates_surface() {
    fn finite<T: Finite>(x: T) -> bool {
        x.is_finite()
    }

    fn infinite<T: Infinite>(x: T) -> bool {
        x.is_infinite()
    }

    fn nan<T: Nan>(x: T) -> bool {
        x.is_nan()
    }

    assert!(finite(1.0_f32));

    assert!(!finite(<f32 as Infinite>::INFINITY));
    assert!(!finite(<f32 as Nan>::NAN));

    assert!(infinite(<f32 as Infinite>::INFINITY));
    assert!(infinite(<f32 as Infinite>::NEG_INFINITY));

    assert!(!infinite(<f32 as Nan>::NAN));
    assert!(!infinite(1.0_f32));

    assert!(nan(<f32 as Nan>::NAN));

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
    assert!(approx_eql_abs_tol!(a32.lerp(b32, 0.25), 12.5_f32, 1e-6));
    assert!(approx_eql_abs_tol!(<f32 as Lerp>::lerp(a32, b32, 0.25), 12.5_f32, 1e-6));

    let a64 = -4.0_f64;
    let b64 = 6.0_f64;
    assert!(approx_eql_abs_tol!(a64.lerp(b64, 0.5), 1.0_f64, 1e-12));
    assert!(approx_eql_abs_tol!(<f64 as Lerp>::lerp(a64, b64, 0.5), 1.0_f64, 1e-12));
}
