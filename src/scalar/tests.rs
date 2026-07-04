use approx::assert_relative_eq;

use super::{FloatScalar, Scalar};

fn scalar_identities<T: Scalar>() -> (T, T) {
    (T::ZERO, T::ONE)
}

fn float_constants<T: FloatScalar>() -> (T, T, T) {
    (T::INFINITY, T::NEG_INFINITY, T::NAN)
}

fn sin_via_floatscalar<T: FloatScalar>(x: T) -> T {
    x.sin()
}

fn round_trip<T: FloatScalar>(value: T) -> T {
    (value + T::ONE) - T::ONE
}

#[test]
fn scalar_constants_match_for_f32_and_f64() {
    assert_eq!(scalar_identities::<f32>(), (0.0, 1.0));
    assert_eq!(scalar_identities::<f64>(), (0.0, 1.0));
}

#[test]
fn float_constants_match_for_f32_and_f64() {
    let (f32_inf, f32_neg_inf, f32_nan) = float_constants::<f32>();
    assert!(f32_inf.is_infinite() && f32_inf.is_sign_positive());
    assert!(f32_neg_inf.is_infinite() && f32_neg_inf.is_sign_negative());
    assert!(f32_nan.is_nan());

    let (f64_inf, f64_neg_inf, f64_nan) = float_constants::<f64>();
    assert!(f64_inf.is_infinite() && f64_inf.is_sign_positive());
    assert!(f64_neg_inf.is_infinite() && f64_neg_inf.is_sign_negative());
    assert!(f64_nan.is_nan());
}

#[test]
fn corefloat_and_approx_work_through_floatscalar_for_f32_and_f64() {
    let sin_zero_f32 = sin_via_floatscalar::<f32>(0.0);
    assert_relative_eq!(sin_zero_f32, 0.0, epsilon = 1.0e-6);

    let sin_zero_f64 = sin_via_floatscalar::<f64>(0.0);
    assert_relative_eq!(sin_zero_f64, 0.0, epsilon = 1.0e-12);
}

#[test]
fn generic_round_trip_works_for_f32_and_f64() {
    let out_f32 = round_trip::<f32>(3.5);
    assert_relative_eq!(out_f32, 3.5, epsilon = 1.0e-6);

    let out_f64 = round_trip::<f64>(3.5);
    assert_relative_eq!(out_f64, 3.5, epsilon = 1.0e-12);
}

#[cfg(feature = "std")]
#[test]
fn std_inherent_sin_matches_floatscalar_path_for_f32_and_f64() {
    let x32 = 0.5_f32;
    let std_sin32 = x32.sin();
    let generic_sin32 = sin_via_floatscalar::<f32>(x32);
    assert_relative_eq!(std_sin32, generic_sin32, epsilon = 1.0e-6);

    let x64 = 0.5_f64;
    let std_sin64 = x64.sin();
    let generic_sin64 = sin_via_floatscalar::<f64>(x64);
    assert_relative_eq!(std_sin64, generic_sin64, epsilon = 1.0e-12);
}

#[cfg(not(feature = "std"))]
#[test]
fn non_std_method_sin_still_works_for_f32_and_f64() {
    let x32 = 0.5_f32;
    let sin32 = x32.sin();
    assert_relative_eq!(sin32, sin_via_floatscalar::<f32>(x32), epsilon = 1.0e-6);

    let x64 = 0.5_f64;
    let sin64 = x64.sin();
    assert_relative_eq!(sin64, sin_via_floatscalar::<f64>(x64), epsilon = 1.0e-12);
}
