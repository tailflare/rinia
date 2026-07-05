use crate::{Vec2, Vec3, Vec4, Vector};

fn assert_is_vector<T: Vector>() {}

#[test]
fn vector_trait_is_implemented_for_supported_scalars() {
    assert_is_vector::<Vec2<f32>>();
    assert_is_vector::<Vec3<i32>>();
}

#[test]
fn vec3_identity_and_default() {
    assert_eq!(Vec3::<f32>::ZERO.to_array(), [0.0, 0.0, 0.0]);
    assert_eq!(Vec3::<f32>::ONE.to_array(), [1.0, 1.0, 1.0]);
    assert_eq!(Vec3::<f32>::default(), Vec3::<f32>::ZERO);
    assert!(crate::ops::Zero::is_zero(Vec3::<f32>::ZERO));
}

#[test]
fn vec3_add_and_add_assign() {
    let a = Vec3::<f32>::from_array([1.0, 2.0, 3.0]);
    let b = Vec3::<f32>::from_array([4.0, 5.0, 6.0]);
    assert_eq!((a + b).to_array(), [5.0, 7.0, 9.0]);

    let mut c = Vec3::<f32>::from_array([1.0, 1.0, 1.0]);
    c += Vec3::<f32>::from_array([2.0, 3.0, 4.0]);
    assert_eq!(c.to_array(), [3.0, 4.0, 5.0]);
}

#[test]
fn vec3_sub_mul_div_and_assign_variants() {
    let a = Vec3::<f32>::from_array([8.0, 6.0, 4.0]);
    let b = Vec3::<f32>::from_array([2.0, 3.0, 4.0]);

    assert_eq!((a - b).to_array(), [6.0, 3.0, 0.0]);
    assert_eq!((a * b).to_array(), [16.0, 18.0, 16.0]);
    assert_eq!((a / b).to_array(), [4.0, 2.0, 1.0]);

    let mut c = Vec3::<f32>::from_array([8.0, 6.0, 4.0]);
    c -= Vec3::<f32>::from_array([1.0, 2.0, 3.0]);
    assert_eq!(c.to_array(), [7.0, 4.0, 1.0]);

    c *= Vec3::<f32>::from_array([2.0, 2.0, 2.0]);
    assert_eq!(c.to_array(), [14.0, 8.0, 2.0]);

    c /= Vec3::<f32>::from_array([2.0, 4.0, 2.0]);
    assert_eq!(c.to_array(), [7.0, 2.0, 1.0]);
}

#[test]
fn vec4_neg() {
    let a = Vec4::<f32>::from_array([1.0, -2.0, 3.5, 0.0]);
    assert_eq!((-a).to_array(), [-1.0, 2.0, -3.5, -0.0]);
}

#[test]
fn vec2_integer_arithmetic() {
    let a = Vec2::<i32>::from_array([10, -3]);
    let b = Vec2::<i32>::from_array([2, 5]);
    assert_eq!((a + b).to_array(), [12, 2]);
    assert_eq!((a - b).to_array(), [8, -8]);
    assert_eq!((-b).to_array(), [-2, -5]);
}

#[test]
fn vec3_select_min_max() {
    use crate::ops::Select;

    let a = Vec3::<f32>::from_array([1.0, 8.0, -4.0]);
    let b = Vec3::<f32>::from_array([2.0, 3.0, -5.0]);

    assert_eq!(a.min(b).to_array(), [1.0, 3.0, -5.0]);
    assert_eq!(a.max(b).to_array(), [2.0, 8.0, -4.0]);

    assert_eq!(Select::min_val(a, b).to_array(), [1.0, 3.0, -5.0]);
    assert_eq!(Select::max_val(a, b).to_array(), [2.0, 8.0, -4.0]);
}

#[test]
fn vec3_dot_product() {
    use crate::ops::DotProduct;

    let a = Vec3::<f32>::from_array([1.0, 2.0, 3.0]);
    let b = Vec3::<f32>::from_array([4.0, -5.0, 6.0]);

    assert_eq!(a.dot(b), 12.0);
    assert_eq!(DotProduct::dot(a, b), 12.0);
}

#[test]
fn vec2_dot_product_integer() {
    use crate::ops::DotProduct;

    let a = Vec2::<i32>::from_array([3, -2]);
    let b = Vec2::<i32>::from_array([4, 5]);

    assert_eq!(a.dot(b), 2);
    assert_eq!(DotProduct::dot(a, b), 2);
}

#[test]
fn vec3_approx_equality_traits() {
    use approx::{AbsDiffEq, RelativeEq, UlpsEq};

    let a = Vec3::<f32>::from_array([1.0, 2.0, 3.0]);
    let b = Vec3::<f32>::from_array([1.0 + 1.0e-6, 2.0 - 1.0e-6, 3.0]);

    assert!(Vec3::abs_diff_eq(&a, &b, 1.0e-5));
    assert!(Vec3::relative_eq(&a, &b, 1.0e-5, 1.0e-5));
    assert!(Vec3::ulps_eq(&a, &b, 1.0e-5, 16));
}

#[cfg(feature = "mint")]
#[test]
fn vec_mint_roundtrip_conversions_work() {
    let v2 = Vec2::<f32>::new(1.0, -2.0);
    let m2: mint::Vector2<f32> = v2.into();
    assert_eq!((m2.x, m2.y), (1.0, -2.0));
    let r2: Vec2<f32> = m2.into();
    assert_eq!(r2.to_array(), [1.0, -2.0]);

    let v3 = Vec3::<f32>::new(1.0, -2.0, 3.5);
    let m3: mint::Vector3<f32> = v3.into();
    assert_eq!((m3.x, m3.y, m3.z), (1.0, -2.0, 3.5));
    let r3: Vec3<f32> = m3.into();
    assert_eq!(r3.to_array(), [1.0, -2.0, 3.5]);

    let v4 = Vec4::<f32>::new(1.0, -2.0, 3.5, 4.25);
    let m4: mint::Vector4<f32> = v4.into();
    assert_eq!((m4.x, m4.y, m4.z, m4.w), (1.0, -2.0, 3.5, 4.25));
    let r4: Vec4<f32> = m4.into();
    assert_eq!(r4.to_array(), [1.0, -2.0, 3.5, 4.25]);
}
