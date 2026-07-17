#![cfg(test)]

use alloc::vec::Vec;

use crate::{
    algebra::{ApproxEqAbs, ApproxEqRel},
    approx_eq_abs, approx_eq_rel, approx_ne_abs, approx_ne_rel, assert_approx_eq_abs,
    assert_approx_eq_rel, assert_approx_ne_abs, assert_approx_ne_rel,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Thin<T>(T);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThickSingle<T> {
    pub value: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThickMulti<T> {
    pub value1: T,
    pub value2: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ThickConst<T, const N: usize> {
    pub value1: T,
    pub value2: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SignedEq<T> {
    value: T,
}

crate::impl_approx_eq_wrapper!([
    T
], impl: Thin<T>, item: T);

crate::impl_approx_eq_wrapper!([
    T
], impl: ThickSingle<T>, item: T, field: value);

crate::impl_approx_eq_wrapper!([
    T
], impl: ThickMulti<T>, item: T, fields: [value1, value2]);

crate::impl_approx_eq_wrapper!([
    T, const N: usize
], impl: ThickConst<T, N>, item: T, fields: [value1, value2]);

crate::impl_approx_eq_wrapper!(
    [T],
    impl: SignedEq<T>,
    item: T,
    extra_bounds: [core::ops::Neg<Output = T>],
    compare_abs: |lhs, rhs, tol| {
        lhs.value.approx_eq_abs_tol(&rhs.value, tol) || lhs.value.approx_eq_abs_tol(&-rhs.value, tol)
    },
    compare_rel: |lhs, rhs, tol| {
        lhs.value.approx_eq_rel_tol(&rhs.value, tol) || lhs.value.approx_eq_rel_tol(&-rhs.value, tol)
    },
);

#[test]
fn approx_eq_wrapper_thin_surface() {
    let lhs = Thin(1.0_f32);
    let rhs_close = Thin(1.0005_f32);
    let rhs_far = Thin(1.01_f32);

    assert!(lhs.approx_eq_abs_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_rel_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_abs(&lhs));
    assert!(lhs.approx_eq_rel(&lhs));

    assert!(<Thin<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_close, 0.001_f32));
    assert!(!<Thin<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<Thin<f32> as ApproxEqAbs>::approx_eq_abs(&lhs, &lhs));

    assert!(<Thin<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_close, 0.001_f32));
    assert!(!<Thin<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<Thin<f32> as ApproxEqRel>::approx_eq_rel(&lhs, &lhs));
}

#[test]
fn approx_eq_wrapper_thick_single_surface() {
    let lhs = ThickSingle { value: 1.0_f32 };
    let rhs_close = ThickSingle { value: 1.0005_f32 };
    let rhs_far = ThickSingle { value: 1.01_f32 };

    assert!(lhs.approx_eq_abs_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_rel_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_abs(&lhs));
    assert!(lhs.approx_eq_rel(&lhs));

    assert!(<ThickSingle<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_close, 0.001_f32));
    assert!(!<ThickSingle<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<ThickSingle<f32> as ApproxEqAbs>::approx_eq_abs(&lhs, &lhs));

    assert!(<ThickSingle<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_close, 0.001_f32));
    assert!(!<ThickSingle<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<ThickSingle<f32> as ApproxEqRel>::approx_eq_rel(&lhs, &lhs));
}

#[test]
fn approx_eq_wrapper_thick_multi_surface() {
    let lhs = ThickMulti { value1: 10.0_f32, value2: 20.0_f32 };
    let rhs_close = ThickMulti { value1: 10.0_f32, value2: 20.01_f32 };
    let rhs_far = ThickMulti { value1: 10.0_f32, value2: 20.5_f32 };

    assert!(lhs.approx_eq_abs_tol(&rhs_close, 0.02_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_far, 0.02_f32));

    assert!(lhs.approx_eq_rel_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_abs(&lhs));
    assert!(lhs.approx_eq_rel(&lhs));

    assert!(<ThickMulti<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_close, 0.02_f32));
    assert!(!<ThickMulti<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_far, 0.02_f32));
    assert!(<ThickMulti<f32> as ApproxEqAbs>::approx_eq_abs(&lhs, &lhs));

    assert!(<ThickMulti<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_close, 0.001_f32));
    assert!(!<ThickMulti<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<ThickMulti<f32> as ApproxEqRel>::approx_eq_rel(&lhs, &lhs));
}

#[test]
fn approx_eq_wrapper_thick_const_surface() {
    let lhs = ThickConst::<f32, 2> { value1: 10.0_f32, value2: 20.0_f32 };
    let rhs_close = ThickConst::<f32, 2> { value1: 10.0_f32, value2: 20.01_f32 };
    let rhs_far = ThickConst::<f32, 2> { value1: 10.0_f32, value2: 20.5_f32 };

    assert!(lhs.approx_eq_abs_tol(&rhs_close, 0.02_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_far, 0.02_f32));

    assert!(lhs.approx_eq_rel_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_abs(&lhs));
    assert!(lhs.approx_eq_rel(&lhs));

    assert!(<ThickConst<f32, 2> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_close, 0.02_f32));
    assert!(!<ThickConst<f32, 2> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_far, 0.02_f32));
    assert!(<ThickConst<f32, 2> as ApproxEqAbs>::approx_eq_abs(&lhs, &lhs));

    assert!(<ThickConst<f32, 2> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_close, 0.001_f32));
    assert!(!<ThickConst<f32, 2> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<ThickConst<f32, 2> as ApproxEqRel>::approx_eq_rel(&lhs, &lhs));
}

#[test]
fn approx_eq_wrapper_signed_callback_surface() {
    let lhs = SignedEq { value: 2.0_f32 };
    let rhs_sign_flipped = SignedEq { value: -2.0_f32 };
    let rhs_far = SignedEq { value: -3.0_f32 };

    assert!(lhs.approx_eq_abs_tol(&rhs_sign_flipped, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_rel_tol(&rhs_sign_flipped, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_far, 0.001_f32));

    assert!(lhs.approx_eq_abs(&lhs));
    assert!(lhs.approx_eq_rel(&lhs));

    assert!(<SignedEq<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_sign_flipped, 0.001_f32));
    assert!(!<SignedEq<f32> as ApproxEqAbs>::approx_eq_abs_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<SignedEq<f32> as ApproxEqAbs>::approx_eq_abs(&lhs, &lhs));

    assert!(<SignedEq<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_sign_flipped, 0.001_f32));
    assert!(!<SignedEq<f32> as ApproxEqRel>::approx_eq_rel_tol(&lhs, &rhs_far, 0.001_f32));
    assert!(<SignedEq<f32> as ApproxEqRel>::approx_eq_rel(&lhs, &lhs));
}

#[test]
fn approx_eq_abs_array_blanket_impl() {
    let lhs = [1.0_f32, 2.0_f32, 3.0_f32];
    let rhs_close = [1.0_f32, 2.0005_f32, 3.0_f32];
    let rhs_far = [1.0_f32, 2.01_f32, 3.0_f32];

    assert!(lhs.approx_eq_abs_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_far, 0.001_f32));
}

#[test]
fn approx_eq_rel_array_blanket_impl() {
    let lhs = [100.0_f32, 200.0_f32, 300.0_f32];
    let rhs_close = [100.05_f32, 200.0_f32, 300.0_f32];
    let rhs_far = [101.0_f32, 200.0_f32, 300.0_f32];

    assert!(lhs.approx_eq_rel_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_far, 0.001_f32));
}

#[test]
fn approx_eq_abs_slice_blanket_impl() {
    let lhs = [1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32];
    let rhs_close = [1.0_f32, 2.0_f32, 3.0005_f32, 4.0_f32];
    let rhs_short = [1.0_f32, 2.0_f32, 3.0_f32];

    assert!((lhs[..]).approx_eq_abs_tol(&rhs_close[..], 0.001_f32));
    assert!(!(lhs[..]).approx_eq_abs_tol(&rhs_short[..], 0.001_f32));
}

#[test]
fn approx_eq_rel_slice_blanket_impl() {
    let lhs = [10.0_f32, 20.0_f32, 30.0_f32, 40.0_f32];
    let rhs_close = [10.0_f32, 20.01_f32, 30.0_f32, 40.0_f32];
    let rhs_short = [10.0_f32, 20.0_f32, 30.0_f32];

    assert!((lhs[..]).approx_eq_rel_tol(&rhs_close[..], 0.001_f32));
    assert!(!(lhs[..]).approx_eq_rel_tol(&rhs_short[..], 0.001_f32));
}

#[test]
fn approx_eq_abs_vec_blanket_impl() {
    let lhs = Vec::from([1.0_f32, 2.0_f32, 3.0_f32]);
    let rhs_close = Vec::from([1.0_f32, 2.0_f32, 3.0005_f32]);
    let rhs_short = Vec::from([1.0_f32, 2.0_f32]);

    assert!(lhs.approx_eq_abs_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(&rhs_short, 0.001_f32));
}

#[test]
fn approx_eq_rel_vec_blanket_impl() {
    let lhs = Vec::from([10.0_f32, 20.0_f32, 30.0_f32]);
    let rhs_close = Vec::from([10.0_f32, 20.01_f32, 30.0_f32]);
    let rhs_short = Vec::from([10.0_f32, 20.0_f32]);

    assert!(lhs.approx_eq_rel_tol(&rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(&rhs_short, 0.001_f32));
}

#[test]
fn approx_eq_macro_variants_surface() {
    assert!(approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7));
    assert!(approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7, 1e-6_f32));
    assert!(approx_ne_abs!(1.0_f32, 1.1_f32));
    assert!(approx_ne_abs!(1.0_f32, 1.1_f32, 1e-6_f32));

    assert!(approx_eq_rel!(1000.0_f32, 1000.001_f32));
    assert!(approx_eq_rel!(1000.0_f32, 1000.001_f32, 1e-5_f32));
    assert!(approx_ne_rel!(1.0_f32, 2.0_f32));
    assert!(approx_ne_rel!(1.0_f32, 2.0_f32, 1e-5_f32));

    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7);
    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7, 1e-6_f32);
    assert_approx_eq_abs!(1.0_f32, 1.0_f32 + 5e-7, message = "abs {} message form", "default");
    assert_approx_eq_abs!(
        1.0_f32,
        1.0_f32 + 5e-7,
        1e-6_f32,
        message = "abs {} message form",
        "tol"
    );

    assert_approx_ne_abs!(1.0_f32, 1.1_f32);
    assert_approx_ne_abs!(1.0_f32, 1.1_f32, 1e-6_f32);
    assert_approx_ne_abs!(1.0_f32, 1.1_f32, message = "ne abs {} message form", "default");
    assert_approx_ne_abs!(1.0_f32, 1.1_f32, 1e-6_f32, message = "ne abs {} message form", "tol");

    assert_approx_eq_rel!(1000.0_f32, 1000.001_f32);
    assert_approx_eq_rel!(1000.0_f32, 1000.001_f32, 1e-5_f32);
    assert_approx_eq_rel!(1000.0_f32, 1000.001_f32, message = "rel {} message form", "default");
    assert_approx_eq_rel!(
        1000.0_f32,
        1000.001_f32,
        1e-5_f32,
        message = "rel {} message form",
        "tol"
    );

    assert_approx_ne_rel!(1.0_f32, 2.0_f32);
    assert_approx_ne_rel!(1.0_f32, 2.0_f32, 1e-5_f32);
    assert_approx_ne_rel!(1.0_f32, 2.0_f32, message = "ne rel {} message form", "default");
    assert_approx_ne_rel!(1.0_f32, 2.0_f32, 1e-5_f32, message = "ne rel {} message form", "tol");
}

#[test]
#[should_panic(expected = "abs message panic: 1 2")]
fn approx_eq_abs_message_variant_panics() {
    assert_approx_eq_abs!(1.0_f32, 2.0_f32, message = "abs message panic: {} {}", 1, 2);
}

#[test]
#[should_panic(expected = "abs tol message panic: 0.000001")]
fn approx_eq_abs_tol_message_variant_panics() {
    assert_approx_eq_abs!(
        1.0_f32,
        2.0_f32,
        1e-6_f32,
        message = "abs tol message panic: {}",
        1e-6_f32
    );
}

#[test]
#[should_panic(expected = "ne abs message panic: x")]
fn approx_ne_abs_message_variant_panics() {
    assert_approx_ne_abs!(1.0_f32, 1.0_f32 + 5e-7, message = "ne abs message panic: {}", "x");
}

#[test]
#[should_panic(expected = "ne abs tol message panic: 7")]
fn approx_ne_abs_tol_message_variant_panics() {
    assert_approx_ne_abs!(
        1.0_f32,
        1.0_f32 + 5e-7,
        1e-6_f32,
        message = "ne abs tol message panic: {}",
        7
    );
}

#[test]
#[should_panic(expected = "rel message panic: lhs rhs")]
fn approx_eq_rel_message_variant_panics() {
    assert_approx_eq_rel!(1.0_f32, 2.0_f32, message = "rel message panic: {} {}", "lhs", "rhs");
}

#[test]
#[should_panic(expected = "rel tol message panic: 0.00001")]
fn approx_eq_rel_tol_message_variant_panics() {
    assert_approx_eq_rel!(
        1.0_f32,
        2.0_f32,
        1e-5_f32,
        message = "rel tol message panic: {}",
        1e-5_f32
    );
}

#[test]
#[should_panic(expected = "ne rel message panic: z")]
fn approx_ne_rel_message_variant_panics() {
    assert_approx_ne_rel!(100_000.0_f32, 100_000.5_f32, message = "ne rel message panic: {}", "z");
}

#[test]
#[should_panic(expected = "ne rel tol message panic: ok")]
fn approx_ne_rel_tol_message_variant_panics() {
    assert_approx_ne_rel!(
        1000.0_f32,
        1000.001_f32,
        1e-5_f32,
        message = "ne rel tol message panic: {}",
        "ok"
    );
}
