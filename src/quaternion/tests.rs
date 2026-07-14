#![cfg(test)]

use alloc::vec::Vec;

use crate::{
    algebra::{Dot, Identity, Inverse, Length, LengthSquared, Lerp, Normalize},
    approx_eql_abs, approx_eql_abs_tol, approx_eql_rel, approx_eql_rel_tol,
    numeric::{
        Cast, CastFrom, Infinite, IsFinite, IsZero, LossyCast, LossyCastFrom, Nan, Negate,
        SaturatingCast, SaturatingCastFrom, TryCast, TryCastFrom, TryExactCast, TryExactCastFrom,
    },
    quaternion::Quaternion,
    tuple::{Tuple, TupleLike},
    vector::{Vec4, Vector},
};

#[test]
fn array_roundtrip() {
    let q = Quaternion::<i32>::from_array([1, 2, 3, 4]);
    assert_eq!(q.as_array(), &[1, 2, 3, 4]);
    assert_eq!(q.into_array(), [1, 2, 3, 4]);
}

#[test]
fn tuple_roundtrip() {
    let t = Tuple::from_array([4_i32, 5, 6, 7]);
    let q = Quaternion::<i32>::from_tuple(t);
    assert_eq!(q.as_tuple().as_array(), &[4, 5, 6, 7]);
    assert_eq!(q.into_tuple().as_array(), &[4, 5, 6, 7]);
}

#[test]
fn tuple_vector_bridge_roundtrip() {
    let q = Quaternion::new(8_i32, 9, 10, 11);

    let t = q.into_tuple();
    let v = Vec4::from_tuple(t);
    assert_eq!(v.as_array(), &[8, 9, 10, 11]);

    let q2 = Quaternion::from_tuple(v.into_tuple());
    assert_eq!(q2.into_array(), [8, 9, 10, 11]);
}

#[test]
fn new_and_field_access_surface() {
    let mut q = Quaternion::new(10_i32, 20, 30, 40);
    assert_eq!(q.x, 10);
    assert_eq!(q.y, 20);
    assert_eq!(q.z, 30);
    assert_eq!(q.w, 40);

    q.x = 11;
    q.y = 22;
    q.z = 33;
    q.w = 44;
    assert_eq!(q.into_array(), [11, 22, 33, 44]);
}

#[test]
fn tuple_and_slice_surface() {
    let mut q = Quaternion::<i32>::from_array([1, 2, 3, 4]);
    assert_eq!(q.as_tuple().as_array(), &[1, 2, 3, 4]);
    assert_eq!(q.as_slice(), &[1, 2, 3, 4]);

    q.as_mut_tuple().as_mut_slice()[0] = 9;
    q.as_mut_slice()[3] = 7;
    assert_eq!(q.as_array(), &[9, 2, 3, 7]);
}

#[test]
fn iter_surface() {
    let q = Quaternion::<i32>::from_array([1, 2, 3, 4]);
    let collected = q.iter().copied().collect::<Vec<i32>>();
    assert_eq!(collected, Vec::from([1, 2, 3, 4]));

    let mut q2 = Quaternion::<i32>::from_array([1, 2, 3, 4]);
    for x in q2.iter_mut() {
        *x *= 3;
    }
    assert_eq!(q2.into_array(), [3, 6, 9, 12]);
}

#[test]
fn indexing_surface() {
    let mut q = Quaternion::<i32>::from_array([5, 6, 7, 8]);
    assert_eq!(q[0], 5);
    assert_eq!(q[3], 8);

    q[1] = 60;
    assert_eq!(q.into_array(), [5, 60, 7, 8]);
}

#[test]
fn into_iterator_surface() {
    let owned = Quaternion::<i32>::from_array([1, 2, 3, 4]).into_iter().collect::<Vec<i32>>();
    assert_eq!(owned, Vec::from([1, 2, 3, 4]));

    let by_ref_src = Quaternion::<i32>::from_array([4, 5, 6, 7]);
    let by_ref = (&by_ref_src).into_iter().copied().collect::<Vec<i32>>();
    assert_eq!(by_ref, Vec::from([4, 5, 6, 7]));

    let mut by_mut_src = Quaternion::<i32>::from_array([7, 8, 9, 10]);
    for x in &mut by_mut_src {
        *x += 1;
    }
    assert_eq!(by_mut_src.into_array(), [8, 9, 10, 11]);
}

#[test]
fn from_array_trait_surface() {
    let q: Quaternion<i32> = [1, 3, 5, 7].into();
    assert_eq!(q.as_array(), &[1, 3, 5, 7]);

    let arr: [i32; 4] = q.into();
    assert_eq!(arr, [1, 3, 5, 7]);
}

#[test]
fn tuplelike_surface() {
    let mut q = Quaternion::<i32>::from_array([2, 4, 6, 8]);
    assert_eq!(<Quaternion<i32> as TupleLike>::LEN, 4);
    assert_eq!(TupleLike::as_slice(&q), &[2, 4, 6, 8]);

    TupleLike::as_mut_slice(&mut q)[1] = 40;
    assert_eq!(q.into_array(), [2, 40, 6, 8]);
}

#[test]
fn cast_variants_surface() {
    let src = Quaternion::<i32>::from_array([1, 2, 3, 4]);

    // inherent
    let _: Quaternion<i64> = src.cast();
    let _: Quaternion<u8> = src.lossy_cast();
    let _: Quaternion<u8> = src.saturating_cast();
    let _: Result<Quaternion<u8>, _> = src.try_cast();
    let _: Result<Quaternion<i64>, _> = src.try_exact_cast();

    let _: Quaternion<i64> = Quaternion::cast_from(src);
    let _: Quaternion<u8> = Quaternion::lossy_cast_from(src);
    let _: Quaternion<u8> = Quaternion::saturating_cast_from(src);
    let _: Result<Quaternion<u8>, _> = Quaternion::try_cast_from(src);
    let _: Result<Quaternion<i64>, _> = Quaternion::try_exact_cast_from(src);

    // traits
    let _: Quaternion<i64> = <Quaternion<i32> as Cast<Quaternion<i64>>>::cast(src);
    let _: Quaternion<u8> = <Quaternion<i32> as LossyCast<Quaternion<u8>>>::lossy_cast(src);
    let _: Quaternion<u8> =
        <Quaternion<i32> as SaturatingCast<Quaternion<u8>>>::saturating_cast(src);
    let _: Result<Quaternion<u8>, _> = <Quaternion<i32> as TryCast<Quaternion<u8>>>::try_cast(src);
    let _: Result<Quaternion<i64>, _> =
        <Quaternion<i32> as TryExactCast<Quaternion<i64>>>::try_exact_cast(src);

    let _: Quaternion<i64> = <Quaternion<i64> as CastFrom<Quaternion<i32>>>::cast_from(src);
    let _: Quaternion<u8> =
        <Quaternion<u8> as LossyCastFrom<Quaternion<i32>>>::lossy_cast_from(src);
    let _: Quaternion<u8> =
        <Quaternion<u8> as SaturatingCastFrom<Quaternion<i32>>>::saturating_cast_from(src);
    let _: Result<Quaternion<u8>, _> =
        <Quaternion<u8> as TryCastFrom<Quaternion<i32>>>::try_cast_from(src);
    let _: Result<Quaternion<i64>, _> =
        <Quaternion<i64> as TryExactCastFrom<Quaternion<i32>>>::try_exact_cast_from(src);
}

#[test]
fn identity_surface() {
    let id = Quaternion::<f32>::IDENTITY;
    assert_eq!(id.into_array(), [0.0, 0.0, 0.0, 1.0]);

    let id_trait = <Quaternion<f32> as Identity>::IDENTITY;
    assert_eq!(id_trait.into_array(), [0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn arithmetic_surface() {
    let q = Quaternion::<i32>::from_array([10, 20, 30, 40]);
    assert_eq!((-q).into_array(), [-10, -20, -30, -40]);

    assert_eq!((q * 2).into_array(), [20, 40, 60, 80]);
    assert_eq!((q / 2).into_array(), [5, 10, 15, 20]);

    let mut m = q;
    m *= 2;
    assert_eq!(m.into_array(), [20, 40, 60, 80]);

    let mut d = q;
    d /= 2;
    assert_eq!(d.into_array(), [5, 10, 15, 20]);
}

#[test]
fn numeric_predicates_surface() {
    let zero = Quaternion::<f32>::ZERO;
    let non_zero = Quaternion::<f32>::from_array([0.0, 1.0, 0.0, 0.0]);
    assert!(zero.is_zero());
    assert!(!non_zero.is_zero());
    assert!(<Quaternion<f32> as IsZero>::is_zero(zero));
    assert!(!<Quaternion<f32> as IsZero>::is_zero(non_zero));

    let all_finite = Quaternion::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    let has_infinite = Quaternion::<f32>::from_array([1.0, f32::INFINITY, 3.0, 4.0]);
    let has_nan = Quaternion::<f32>::from_array([1.0, f32::NAN, 3.0, 4.0]);

    assert!(all_finite.is_finite());
    assert!(!has_infinite.is_finite());
    assert!(!has_nan.is_finite());

    assert!(!all_finite.is_infinite());
    assert!(has_infinite.is_infinite());

    assert!(!all_finite.is_nan());
    assert!(has_nan.is_nan());

    let inf = Quaternion::<f32>::INFINITY;
    let neg_inf = Quaternion::<f32>::NEG_INFINITY;
    let nan = Quaternion::<f32>::NAN;
    assert!(inf.is_infinite());
    assert!(neg_inf.is_infinite());
    assert!(nan.is_nan());

    let inf_trait = <Quaternion<f32> as Infinite>::INFINITY;
    let neg_inf_trait = <Quaternion<f32> as Infinite>::NEG_INFINITY;
    let nan_trait = <Quaternion<f32> as Nan>::NAN;
    assert!(inf_trait.is_infinite());
    assert!(neg_inf_trait.is_infinite());
    assert!(nan_trait.is_nan());

    assert!(<Quaternion<f32> as IsFinite>::is_finite(all_finite));
}

#[test]
fn approx_eq_abs_surface() {
    let q = Quaternion::from_array([0.0_f32, 1.0, 2.0, 3.0]);
    let minus_q = Quaternion::from_array([-0.0_f32, -1.0, -2.0, -3.0]);
    let far = Quaternion::from_array([0.0_f32, 1.0, 2.0, 3.2]);

    assert!(approx_eql_abs_tol!(q, minus_q, 1e-6));
    assert!(!approx_eql_abs_tol!(q, far, 1e-6));
    assert!(approx_eql_abs!(q, minus_q));
    assert!(!approx_eql_abs!(q, far));

    assert!(approx_eql_abs_tol!(q, minus_q, 1e-6));
    assert!(!approx_eql_abs_tol!(q, far, 1e-6));
    assert!(approx_eql_abs!(q, minus_q));
    assert!(!approx_eql_abs!(q, far));
}

#[test]
fn approx_eq_rel_surface() {
    let q = Quaternion::from_array([1.0_f32, 2.0, 3.0, 4.0]);
    let minus_q = Quaternion::from_array([-1.0_f32, -2.0, -3.0, -4.0]);
    let far = Quaternion::from_array([1.0_f32, 2.0, 3.0, 4.2]);

    assert!(approx_eql_rel_tol!(q, minus_q, 1e-4));
    assert!(!approx_eql_rel_tol!(q, far, 1e-4));
    assert!(approx_eql_rel!(q, minus_q));
    assert!(!approx_eql_rel!(q, far));

    assert!(approx_eql_rel_tol!(q, minus_q, 1e-4));
    assert!(!approx_eql_rel_tol!(q, far, 1e-4));
    assert!(approx_eql_rel!(q, minus_q));
    assert!(!approx_eql_rel!(q, far));
}

#[test]
fn dot_surface() {
    let a = Quaternion::from_array([1_i32, 2, 3, 4]);
    let b = Quaternion::from_array([5_i32, 6, 7, 8]);

    assert_eq!(a.dot(b), 70);

    assert_eq!(<Quaternion<i32> as Dot>::dot(a, b), 70);
}

#[test]
fn conjugate_surface() {
    let q = Quaternion::from_array([1_i32, 2, 3, 4]);

    assert_eq!(q.conjugate().into_array(), [-1, -2, -3, 4]);
}

#[test]
fn inverse_surface() {
    let q = Quaternion::from_array([1.0_f32, 2.0, 3.0, 4.0]);
    let expected = Quaternion::from_array([-1.0_f32 / 30.0, -2.0 / 30.0, -3.0 / 30.0, 4.0 / 30.0]);

    let inverse = q.inverse();
    let trait_inverse = <Quaternion<f32> as Inverse>::inverse(q);

    assert!(approx_eql_abs!(inverse, trait_inverse));
    assert!(approx_eql_abs!(inverse, expected));
    assert!(approx_eql_abs!(trait_inverse, expected));
}

#[test]
fn compose_surface() {
    let qx_180 = Quaternion::from_array([1.0_f32, 0.0, 0.0, 0.0]);
    let qy_180 = Quaternion::from_array([0.0_f32, 1.0, 0.0, 0.0]);

    let composed_inherent = qx_180.compose(qy_180);
    let composed_op = qx_180 * qy_180;
    let expected = Quaternion::from_array([0.0_f32, 0.0, 1.0, 0.0]);

    assert!(approx_eql_abs!(composed_inherent, composed_op));
    assert!(approx_eql_abs!(composed_inherent, expected));
}

#[test]
fn rotate_vector_surface() {
    let qz_180 = Quaternion::from_array([0.0_f32, 0.0, 1.0, 0.0]);
    let v = Vector::from_array([1.0_f32, 2.0, 0.0]);

    let rotated_inherent = qz_180.rotate_vector(v);
    let rotated_op = qz_180 * v;
    let expected = Vector::from_array([-1.0_f32, -2.0, 0.0]);

    assert!(approx_eql_abs_tol!(rotated_inherent, rotated_op, 1e-6));
    assert!(approx_eql_abs_tol!(rotated_inherent, expected, 1e-6));
}

#[test]
fn axis_angle_surface() {
    let axis_z = Vector::from_array([0.0_f32, 0.0, 1.0]);
    let angle_pi = core::f32::consts::PI;

    let q_from = Quaternion::from_axis_angle(axis_z, angle_pi);
    let qz_180 = Quaternion::from_array([0.0_f32, 0.0, 1.0, 0.0]);
    assert!(approx_eql_abs_tol!(q_from, qz_180, 1e-6));

    let (axis, angle) = qz_180.to_axis_angle();

    assert!(approx_eql_abs_tol!(axis, Vector::from_array([0.0_f32, 0.0, 1.0]), 1e-6));
    assert!(approx_eql_abs_tol!(angle, core::f32::consts::PI, 1e-6));

    let reconstructed = Quaternion::from_axis_angle(axis, angle);
    assert!(approx_eql_abs_tol!(reconstructed, qz_180, 1e-6));

    let identity = Quaternion::from_array([0.0_f32, 0.0, 0.0, 1.0]);
    let (identity_axis, identity_angle) = identity.to_axis_angle();

    assert!(approx_eql_abs_tol!(identity_axis, Vector::from_array([1.0_f32, 0.0, 0.0]), 1e-6));
    assert!(approx_eql_abs_tol!(identity_angle, 0.0_f32, 1e-6));
}

#[test]
fn interpolation_surface() {
    let id = Quaternion::from_array([0.0_f32, 0.0, 0.0, 1.0]);
    let z_180 = Quaternion::from_array([0.0_f32, 0.0, 1.0, 0.0]);
    let half_turn = Quaternion::from_array([
        0.0_f32,
        0.0,
        core::f32::consts::FRAC_1_SQRT_2,
        core::f32::consts::FRAC_1_SQRT_2,
    ]);
    let half_turn_neg_branch = Quaternion::from_array([
        0.0_f32,
        0.0,
        -core::f32::consts::FRAC_1_SQRT_2,
        core::f32::consts::FRAC_1_SQRT_2,
    ]);

    let lerp_inherent = id.lerp(z_180, 0.5);
    let lerp_trait = <Quaternion<f32> as Lerp>::lerp(id, z_180, 0.5);
    let lerp_expected = Quaternion::from_array([0.0_f32, 0.0, 0.5, 0.5]);

    assert!(approx_eql_abs_tol!(lerp_inherent, lerp_trait, 1e-6));
    assert!(approx_eql_abs_tol!(lerp_inherent, lerp_expected, 1e-6));

    let nlerp_half = id.nlerp(z_180, 0.5);
    let nlerp_half_neg_rhs = id.nlerp(-z_180, 0.5);

    assert!(approx_eql_abs_tol!(nlerp_half.length(), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(nlerp_half, half_turn, 1e-6));
    assert!(approx_eql_abs_tol!(nlerp_half_neg_rhs, half_turn_neg_branch, 1e-6));

    let slerp_half = id.slerp(z_180, 0.5);
    let slerp_half_neg_rhs = id.slerp(-z_180, 0.5);

    assert!(approx_eql_abs_tol!(slerp_half.length(), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(slerp_half, half_turn, 1e-6));
    assert!(approx_eql_abs_tol!(slerp_half_neg_rhs, half_turn_neg_branch, 1e-6));
}

#[test]
fn length_and_normalize_surface() {
    let q = Quaternion::<f32>::from_array([0.0, 0.0, 3.0, 4.0]);

    assert!(approx_eql_abs_tol!(q.length_squared(), 25.0, 1e-6));
    assert!(approx_eql_abs_tol!(<Quaternion<f32> as LengthSquared>::length_squared(q), 25.0, 1e-6));

    assert!(approx_eql_abs_tol!(q.length(), 5.0, 1e-6));
    assert!(approx_eql_abs_tol!(<Quaternion<f32> as Length>::length(q), 5.0, 1e-6));

    let n = q.normalize();
    let nt = <Quaternion<f32> as Normalize>::normalize(q);

    assert!(approx_eql_abs_tol!(n.length(), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(nt.length(), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(n, Quaternion::from_array([0.0, 0.0, 0.6, 0.8]), 1e-6));
    assert!(approx_eql_abs_tol!(nt, Quaternion::from_array([0.0, 0.0, 0.6, 0.8]), 1e-6));

    let a = Quaternion::from_array([1_i32, 2, 3, 4]);
    assert_eq!(a.negate().into_array(), [-1, -2, -3, -4]);
    assert_eq!(<Quaternion<i32> as Negate>::negate(a).into_array(), [-1, -2, -3, -4]);
}

#[cfg(feature = "bytemuck")]
#[test]
fn bytemuck_roundtrip() {
    let q = Quaternion::<i32>::from_array([1, 2, 3, 4]);
    let bytes = bytemuck::bytes_of(&q);
    let out = bytemuck::pod_read_unaligned::<Quaternion<i32>>(bytes);
    assert_eq!(out, q);
}

#[cfg(feature = "zerocopy")]
#[test]
fn zerocopy_roundtrip() {
    let q = Quaternion::<i32>::from_array([5, 6, 7, 8]);
    let bytes = <Quaternion<i32> as zerocopy::IntoBytes>::as_bytes(&q);
    let out = <Quaternion<i32> as zerocopy::FromBytes>::read_from_bytes(bytes)
        .expect("valid quaternion bytes");
    assert_eq!(out, q);
}

#[cfg(feature = "sakka")]
#[test]
fn sakka_roundtrip() {
    let q = Quaternion::<i32>::from_array([9, 10, 11, 12]);

    let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
    <Quaternion<i32> as sakka::Encode>::encode(&q, &mut writer).expect("encode quaternion");
    let bytes = writer.finish();

    let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
    let out = <Quaternion<i32> as sakka::Decode>::decode(&mut reader).expect("decode quaternion");

    assert_eq!(out, q);
    assert!(reader.is_eof());
}
