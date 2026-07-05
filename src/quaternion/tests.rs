use crate::{Quat, Quaternion};

fn assert_is_quaternion<T: Quaternion>() {}

#[test]
fn quaternion_trait_is_implemented_for_supported_scalars() {
    assert_is_quaternion::<Quat<f32>>();
}

#[test]
fn quat_identity_and_new() {
    assert_eq!(Quat::<f32>::IDENTITY.to_array(), [0.0, 0.0, 0.0, 1.0]);

    let q = Quat::<f32>::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(q.to_array(), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn quat_array_roundtrip_and_indexing() {
    let q = Quat::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(q[0], 1.0);
    assert_eq!(q[1], 2.0);
    assert_eq!(q[2], 3.0);
    assert_eq!(q[3], 4.0);
    assert_eq!(q.to_array(), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn quat_dot_product() {
    use crate::ops::DotProduct;

    let a = Quat::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Quat::<f32>::from_array([2.0, -1.0, 0.5, 3.0]);

    assert_eq!(a.dot(b), 13.5);
    assert_eq!(DotProduct::dot(a, b), 13.5);
}

#[cfg(feature = "mint")]
#[test]
fn quat_mint_roundtrip_conversions_work() {
    let q = Quat::<f32>::new(1.0, 2.0, 3.0, 4.0);
    let m: mint::Quaternion<f32> = q.into();
    assert_eq!((m.v.x, m.v.y, m.v.z, m.s), (1.0, 2.0, 3.0, 4.0));

    let r: Quat<f32> = m.into();
    assert_eq!(r.to_array(), [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn quat_approx_equality_accepts_sign_flipped_quaternions() {
    use approx::{AbsDiffEq, RelativeEq, UlpsEq};

    let q = Quat::<f32>::new(0.1, -0.2, 0.3, 0.9);
    let neg_q = Quat::<f32>::new(-0.1, 0.2, -0.3, -0.9);

    assert!(Quat::abs_diff_eq(&q, &neg_q, 1.0e-6));
    assert!(Quat::relative_eq(&q, &neg_q, 1.0e-6, 1.0e-6));
    assert!(Quat::ulps_eq(&q, &neg_q, 1.0e-6, 8));
}

#[test]
fn quat_approx_equality_rejects_different_rotations() {
    use approx::AbsDiffEq;

    let q = Quat::<f32>::new(0.1, -0.2, 0.3, 0.9);
    let r = Quat::<f32>::new(0.1, -0.2, 0.35, 0.85);

    assert!(!Quat::abs_diff_eq(&q, &r, 1.0e-6));
}

#[test]
fn quat_neg_and_negate_trait_work() {
    use crate::ops::Negate;

    let q = Quat::<f32>::new(1.0, -2.0, 3.5, -4.0);

    let via_neg = -q;
    assert_eq!(via_neg.to_array(), [-1.0, 2.0, -3.5, 4.0]);

    let via_trait = q.negate();
    assert_eq!(via_trait.to_array(), [-1.0, 2.0, -3.5, 4.0]);
}
