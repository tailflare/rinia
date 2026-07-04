use approx::assert_relative_eq;

use super::{FloatScalar, Scalar};
use crate::ops::{HasScalar, Select, Zero};

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

macro_rules! assert_as_scalar_targets {
    ($value:expr, [$($target:ty),+ $(,)?]) => {
        $(
            let _: $target = ($value).as_scalar::<$target>();
        )+
    };
}

#[test]
fn scalar_constants_match_for_f32_and_f64() {
    assert_eq!(scalar_identities::<f32>(), (0.0, 1.0));
    assert_eq!(scalar_identities::<f64>(), (0.0, 1.0));
}

#[test]
fn scalar_bounds_match_for_representative_types() {
    assert_eq!(<f32 as Scalar>::MAX, f32::MAX);
    assert_eq!(<f32 as Scalar>::MIN, f32::MIN);

    assert_eq!(<f64 as Scalar>::MAX, f64::MAX);
    assert_eq!(<f64 as Scalar>::MIN, f64::MIN);

    assert_eq!(<u32 as Scalar>::MAX, u32::MAX);
    assert_eq!(<u32 as Scalar>::MIN, u32::MIN);

    assert_eq!(<i32 as Scalar>::MAX, i32::MAX);
    assert_eq!(<i32 as Scalar>::MIN, i32::MIN);
}

fn assert_has_scalar_self<T>()
where
    T: Scalar + HasScalar<Scalar = T>,
{
}

#[test]
fn has_scalar_associated_type_is_self_for_scalars() {
    assert_has_scalar_self::<f32>();
    assert_has_scalar_self::<f64>();
    assert_has_scalar_self::<u32>();
    assert_has_scalar_self::<i32>();
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

fn min_via_scalar<T: Scalar>(a: T, b: T) -> T {
    a.min_val(b)
}

fn max_via_scalar<T: Scalar>(a: T, b: T) -> T {
    a.max_val(b)
}

fn min_via_select<T: Select>(a: T, b: T) -> T {
    a.min_val(b)
}

fn max_via_select<T: Select>(a: T, b: T) -> T {
    a.max_val(b)
}

#[test]
fn min_max_works_for_integer_scalars() {
    assert_eq!(min_via_scalar(3_i32, 7_i32), 3_i32);
    assert_eq!(max_via_scalar(3_i32, 7_i32), 7_i32);

    assert_eq!(min_via_scalar(10_u16, 2_u16), 2_u16);
    assert_eq!(max_via_scalar(10_u16, 2_u16), 10_u16);

    assert_eq!(min_via_select(3_i32, 7_i32), 3_i32);
    assert_eq!(max_via_select(3_i32, 7_i32), 7_i32);
}

#[test]
fn min_max_works_for_float_scalars() {
    assert_relative_eq!(min_via_scalar(3.0_f32, 7.0_f32), 3.0_f32, epsilon = 1.0e-6);
    assert_relative_eq!(max_via_scalar(3.0_f32, 7.0_f32), 7.0_f32, epsilon = 1.0e-6);

    assert_relative_eq!(min_via_scalar(10.0_f64, 2.0_f64), 2.0_f64, epsilon = 1.0e-12);
    assert_relative_eq!(max_via_scalar(10.0_f64, 2.0_f64), 10.0_f64, epsilon = 1.0e-12);
}

#[test]
fn min_max_nan_behavior_for_float_scalars() {
    let nan32 = <f32 as FloatScalar>::NAN;
    assert_eq!(min_via_scalar(nan32, 4.0_f32), 4.0_f32);
    assert_eq!(max_via_scalar(nan32, 4.0_f32), 4.0_f32);
    assert!(min_via_scalar(nan32, nan32).is_nan());
    assert!(max_via_scalar(nan32, nan32).is_nan());

    let nan64 = <f64 as FloatScalar>::NAN;
    assert_eq!(min_via_scalar(nan64, 4.0_f64), 4.0_f64);
    assert_eq!(max_via_scalar(nan64, 4.0_f64), 4.0_f64);
    assert!(min_via_scalar(nan64, nan64).is_nan());
    assert!(max_via_scalar(nan64, nan64).is_nan());
}

#[test]
fn float_scalar_predicates_work_for_f32_and_f64() {
    let finite32 = 1.0_f32;
    assert!(finite32.is_finite());
    assert!(!finite32.is_infinite());
    assert!(!finite32.is_nan());

    let inf32 = <f32 as FloatScalar>::INFINITY;
    assert!(!inf32.is_finite());
    assert!(inf32.is_infinite());
    assert!(!inf32.is_nan());

    let nan32 = <f32 as FloatScalar>::NAN;
    assert!(!nan32.is_finite());
    assert!(!nan32.is_infinite());
    assert!(nan32.is_nan());

    let finite64 = 1.0_f64;
    assert!(finite64.is_finite());
    assert!(!finite64.is_infinite());
    assert!(!finite64.is_nan());

    let inf64 = <f64 as FloatScalar>::INFINITY;
    assert!(!inf64.is_finite());
    assert!(inf64.is_infinite());
    assert!(!inf64.is_nan());

    let nan64 = <f64 as FloatScalar>::NAN;
    assert!(!nan64.is_finite());
    assert!(!nan64.is_infinite());
    assert!(nan64.is_nan());
}

#[test]
fn float_scalar_is_zero_works_for_f32_and_f64() {
    assert!(0.0_f32.is_zero());
    assert!(f32::ZERO.is_zero());
    assert!(!1.0_f32.is_zero());

    assert!(0.0_f64.is_zero());
    assert!(f64::ZERO.is_zero());
    assert!(!1.0_f64.is_zero());
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

#[test]
fn scalar_as_scalar_converts_between_f32_and_f64() {
    let as_f64 = 1.25_f32.as_scalar::<f64>();
    assert_relative_eq!(as_f64, 1.25_f64, epsilon = 1.0e-12);

    let as_f32 = 2.5_f64.as_scalar::<f32>();
    assert_relative_eq!(as_f32, 2.5_f32, epsilon = 1.0e-6);
}

#[test]
fn scalar_from_scalar_matches_as_scalar_for_f32_and_f64() {
    let from_f64 = <f64 as Scalar>::from_scalar(1.25_f32);
    let as_f64 = 1.25_f32.as_scalar::<f64>();
    assert_relative_eq!(from_f64, as_f64, epsilon = 1.0e-12);

    let from_f32 = <f32 as Scalar>::from_scalar(2.5_f64);
    let as_f32 = 2.5_f64.as_scalar::<f32>();
    assert_relative_eq!(from_f32, as_f32, epsilon = 1.0e-6);
}

#[test]
fn scalar_as_scalar_matrix_compiles_for_all_supported_types() {
    assert_as_scalar_targets!(
        1.5_f32,
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        2.5_f64,
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(7_u8, [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]);
    assert_as_scalar_targets!(
        8_u16,
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        9_u32,
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        10_u64,
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        (-7_i8),
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        (-8_i16),
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        (-9_i32),
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        (-10_i64),
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        11_usize,
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
    assert_as_scalar_targets!(
        (-11_isize),
        [f32, f64, u8, u16, u32, u64, i8, i16, i32, i64, usize, isize]
    );
}

#[test]
fn scalar_as_scalar_cross_family_edges_are_correct_for_representable_values() {
    let f32_from_i32 = (-7_i32).as_scalar::<f32>();
    assert_relative_eq!(f32_from_i32, -7.0_f32, epsilon = 1.0e-6);

    let f64_from_u16 = 42_u16.as_scalar::<f64>();
    assert_relative_eq!(f64_from_u16, 42.0_f64, epsilon = 1.0e-12);

    let i64_from_f32 = 9.9_f32.as_scalar::<i64>();
    assert_eq!(i64_from_f32, 9_i64);

    let u8_from_f64 = 255.0_f64.as_scalar::<u8>();
    assert_eq!(u8_from_f64, 255_u8);

    let usize_from_i16 = 12_i16.as_scalar::<usize>();
    assert_eq!(usize_from_i16, 12_usize);
}

#[test]
fn scalar_as_scalar_works_for_int_and_float_targets() {
    let value_f32 = 1.25_f32;
    let to_f64 = value_f32.as_scalar::<f64>();
    assert_relative_eq!(to_f64, 1.25_f64, epsilon = 1.0e-12);

    let value_f64 = 2.5_f64;
    let to_f32 = value_f64.as_scalar::<f32>();
    assert_relative_eq!(to_f32, 2.5_f32, epsilon = 1.0e-6);
    let x_u32 = 123_u32;
    let as_f64 = x_u32.as_scalar::<f64>();
    assert_relative_eq!(as_f64, 123.0_f64, epsilon = 1.0e-12);

    let x_f64 = 200.5_f64;
    let as_i16 = x_f64.as_scalar::<i16>();
    assert_eq!(as_i16, 200_i16);
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
