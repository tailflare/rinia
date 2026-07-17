#![cfg(test)]

use alloc::vec::Vec;

use crate::algebra::{ApproxEqAbs, ApproxEqRel};

#[test]
fn approx_eq_abs_array_blanket_impl() {
    let lhs = [1.0_f32, 2.0_f32, 3.0_f32];
    let rhs_close = [1.0_f32, 2.0005_f32, 3.0_f32];
    let rhs_far = [1.0_f32, 2.01_f32, 3.0_f32];

    assert!(lhs.approx_eq_abs_tol(rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(rhs_far, 0.001_f32));
}

#[test]
fn approx_eq_rel_array_blanket_impl() {
    let lhs = [100.0_f32, 200.0_f32, 300.0_f32];
    let rhs_close = [100.05_f32, 200.0_f32, 300.0_f32];
    let rhs_far = [101.0_f32, 200.0_f32, 300.0_f32];

    assert!(lhs.approx_eq_rel_tol(rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(rhs_far, 0.001_f32));
}

#[test]
fn approx_eq_abs_slice_blanket_impl() {
    let lhs = [1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32];
    let rhs_close = [1.0_f32, 2.0_f32, 3.0005_f32, 4.0_f32];
    let rhs_short = [1.0_f32, 2.0_f32, 3.0_f32];

    assert!((&lhs[..]).approx_eq_abs_tol(&rhs_close[..], 0.001_f32));
    assert!(!(&lhs[..]).approx_eq_abs_tol(&rhs_short[..], 0.001_f32));
}

#[test]
fn approx_eq_rel_slice_blanket_impl() {
    let lhs = [10.0_f32, 20.0_f32, 30.0_f32, 40.0_f32];
    let rhs_close = [10.0_f32, 20.01_f32, 30.0_f32, 40.0_f32];
    let rhs_short = [10.0_f32, 20.0_f32, 30.0_f32];

    assert!((&lhs[..]).approx_eq_rel_tol(&rhs_close[..], 0.001_f32));
    assert!(!(&lhs[..]).approx_eq_rel_tol(&rhs_short[..], 0.001_f32));
}

#[test]
fn approx_eq_abs_vec_blanket_impl() {
    let lhs = Vec::from([1.0_f32, 2.0_f32, 3.0_f32]);
    let rhs_close = Vec::from([1.0_f32, 2.0_f32, 3.0005_f32]);
    let rhs_short = Vec::from([1.0_f32, 2.0_f32]);

    assert!(lhs.clone().approx_eq_abs_tol(rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_abs_tol(rhs_short, 0.001_f32));
}

#[test]
fn approx_eq_rel_vec_blanket_impl() {
    let lhs = Vec::from([10.0_f32, 20.0_f32, 30.0_f32]);
    let rhs_close = Vec::from([10.0_f32, 20.01_f32, 30.0_f32]);
    let rhs_short = Vec::from([10.0_f32, 20.0_f32]);

    assert!(lhs.clone().approx_eq_rel_tol(rhs_close, 0.001_f32));
    assert!(!lhs.approx_eq_rel_tol(rhs_short, 0.001_f32));
}
