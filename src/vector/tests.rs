#![cfg(test)]

use alloc::vec::Vec;

use crate::{
    algebra::{
        Cast, CastError, Dot, Length, LengthSquared, Lerp, LossyCast, Normalize, TryCast,
        TryExactCast,
    },
    approx_eql_abs, approx_eql_abs_tol, approx_eql_rel, approx_eql_rel_tol,
    numeric::{Abs, Infinite, IsFinite, IsZero, MinMax, Nan, Negate, One, Zero},
    tuple::{Tuple, TupleLike},
    vector::{Vec2, Vec3, Vec4, Vector},
};

#[test]
fn array_roundtrip() {
    let v = Vector::<i32, 3>::from_array([1, 2, 3]);
    assert_eq!(v.as_array(), &[1, 2, 3]);
    assert_eq!(v.into_array(), [1, 2, 3]);
}

#[test]
fn tuple_roundtrip() {
    let t = Tuple::from_array([4_i32, 5, 6]);
    let v = Vector::<i32, 3>::from_tuple(t);
    assert_eq!(v.as_tuple().as_array(), &[4, 5, 6]);
    assert_eq!(v.into_tuple().as_array(), &[4, 5, 6]);
}

#[test]
fn new_constructors_surface() {
    let v2 = Vec2::new(1_i32, 2);
    let v3 = Vec3::new(3_i32, 4, 5);
    let v4 = Vec4::new(6_i32, 7, 8, 9);

    assert_eq!(v2.into_array(), [1, 2]);
    assert_eq!(v3.into_array(), [3, 4, 5]);
    assert_eq!(v4.into_array(), [6, 7, 8, 9]);
}

#[test]
fn field_access_surface() {
    let mut v2 = Vec2::new(10_i32, 20);
    assert_eq!(v2.x, 10);
    assert_eq!(v2.y, 20);
    v2.x = 11;
    v2.y = 22;
    assert_eq!(v2.into_array(), [11, 22]);

    let mut v3 = Vec3::new(1_i32, 2, 3);
    assert_eq!(v3.x, 1);
    assert_eq!(v3.y, 2);
    assert_eq!(v3.z, 3);
    v3.z = 30;
    assert_eq!(v3.into_array(), [1, 2, 30]);

    let mut v4 = Vec4::new(4_i32, 5, 6, 7);
    assert_eq!(v4.w, 7);
    v4.w = 70;
    assert_eq!(v4.into_array(), [4, 5, 6, 70]);
}

#[test]
fn tuple_and_slice_surface() {
    let mut v = Vector::<i32, 3>::from_array([1, 2, 3]);
    assert_eq!(v.as_tuple().as_array(), &[1, 2, 3]);
    assert_eq!(v.as_slice(), &[1, 2, 3]);

    v.as_mut_tuple().as_mut_slice()[0] = 9;
    v.as_mut_slice()[2] = 7;
    assert_eq!(v.as_array(), &[9, 2, 7]);
}

#[test]
fn iter_surface() {
    let v = Vector::<i32, 3>::from_array([1, 2, 3]);
    let collected = v.iter().copied().collect::<Vec<i32>>();
    assert_eq!(collected, Vec::from([1, 2, 3]));

    let mut v2 = Vector::<i32, 3>::from_array([1, 2, 3]);
    for x in v2.iter_mut() {
        *x *= 3;
    }
    assert_eq!(v2.into_array(), [3, 6, 9]);
}

#[test]
fn indexing_surface() {
    let mut v = Vector::<i32, 3>::from_array([5, 6, 7]);
    assert_eq!(v[0], 5);
    assert_eq!(v[2], 7);

    v[1] = 60;
    assert_eq!(v.into_array(), [5, 60, 7]);
}

#[test]
fn into_iterator_surface() {
    let owned = Vector::<i32, 3>::from_array([1, 2, 3]).into_iter().collect::<Vec<i32>>();
    assert_eq!(owned, Vec::from([1, 2, 3]));

    let by_ref_src = Vector::<i32, 3>::from_array([4, 5, 6]);
    let by_ref = (&by_ref_src).into_iter().copied().collect::<Vec<i32>>();
    assert_eq!(by_ref, Vec::from([4, 5, 6]));

    let mut by_mut_src = Vector::<i32, 3>::from_array([7, 8, 9]);
    for x in &mut by_mut_src {
        *x += 1;
    }
    assert_eq!(by_mut_src.into_array(), [8, 9, 10]);
}

#[test]
fn from_array_trait_surface() {
    let v: Vector<i32, 3> = [1, 3, 5].into();
    assert_eq!(v.as_array(), &[1, 3, 5]);

    let arr: [i32; 3] = v.into();
    assert_eq!(arr, [1, 3, 5]);
}

#[test]
fn tuplelike_surface() {
    let mut v = Vector::<i32, 3>::from_array([2, 4, 6]);
    assert_eq!(<Vector<i32, 3> as TupleLike>::LEN, 3);
    assert_eq!(TupleLike::as_slice(&v), &[2, 4, 6]);

    TupleLike::as_mut_slice(&mut v)[1] = 40;
    assert_eq!(v.into_array(), [2, 40, 6]);
}

#[test]
fn cast_variants_surface() {
    let cast_src = Vector::<i8, 3>::from_array([1, 2, 3]);
    assert_eq!(cast_src.cast::<i32>().into_array(), [1_i32, 2, 3]);
    assert_eq!(Vector::<i32, 3>::cast_from(cast_src).into_array(), [1_i32, 2, 3]);
    assert_eq!(<Vector<i8, 3> as Cast<Vector<i32, 3>>>::cast(cast_src).into_array(), [1_i32, 2, 3]);

    let lossy_src = Vector::<i32, 3>::from_array([300, -1, 127]);
    assert_eq!(lossy_src.lossy_cast::<u8>().into_array(), [44_u8, 255, 127]);
    assert_eq!(Vector::<u8, 3>::lossy_cast_from(lossy_src).into_array(), [44_u8, 255, 127]);
    assert_eq!(
        <Vector<i32, 3> as LossyCast<Vector<u8, 3>>>::lossy_cast(lossy_src).into_array(),
        [44_u8, 255, 127]
    );

    let try_src = Vector::<i32, 3>::from_array([1, 2, 255]);
    assert_eq!(try_src.try_cast::<u8>().map(Vector::into_array), Ok([1_u8, 2, 255]));
    assert_eq!(Vector::<u8, 3>::try_cast_from(try_src).map(Vector::into_array), Ok([1_u8, 2, 255]));
    assert_eq!(
        <Vector<i32, 3> as TryCast<Vector<u8, 3>>>::try_cast(try_src).map(Vector::into_array),
        Ok([1_u8, 2, 255])
    );
    assert_eq!(
        Vector::<i32, 3>::from_array([256, 2, 3]).try_cast::<u8>(),
        Err(CastError::OutOfRange)
    );

    let exact_src = Vector::<i32, 3>::from_array([1, 2, 100]);
    assert_eq!(exact_src.try_exact_cast::<i64>().map(Vector::into_array), Ok([1_i64, 2, 100]));
    assert_eq!(
        Vector::<i64, 3>::try_exact_cast_from(exact_src).map(Vector::into_array),
        Ok([1_i64, 2, 100])
    );
    assert_eq!(
        <Vector<i32, 3> as TryExactCast<Vector<i64, 3>>>::try_exact_cast(exact_src)
            .map(Vector::into_array),
        Ok([1_i64, 2, 100])
    );
    assert_eq!(
        Vector::<i32, 3>::from_array([300, 2, 3]).try_exact_cast::<u8>(),
        Err(CastError::OutOfRange)
    );
}

#[test]
fn zero_one_surface() {
    let z = Vector::<f32, 4>::ZERO;
    let o = Vector::<f32, 4>::ONE;

    assert_eq!(z.into_array(), [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(o.into_array(), [1.0, 1.0, 1.0, 1.0]);

    let z_trait = <Vector<f32, 4> as Zero>::ZERO;
    let o_trait = <Vector<f32, 4> as One>::ONE;
    assert_eq!(z_trait.into_array(), [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(o_trait.into_array(), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn arithmetic_surface() {
    let a = Vector::<i32, 3>::from_array([10, 20, 30]);
    let b = Vector::<i32, 3>::from_array([1, 2, 3]);

    assert_eq!((-a).into_array(), [-10, -20, -30]);
    assert_eq!((a + b).into_array(), [11, 22, 33]);
    assert_eq!((a - b).into_array(), [9, 18, 27]);
    assert_eq!((a * 2).into_array(), [20, 40, 60]);
    assert_eq!((a / 2).into_array(), [5, 10, 15]);

    let mut c = Vector::<i32, 3>::from_array([1, 2, 3]);
    c += Vector::<i32, 3>::from_array([2, 3, 4]);
    assert_eq!(c.into_array(), [3, 5, 7]);

    c -= Vector::<i32, 3>::from_array([1, 1, 1]);
    assert_eq!(c.into_array(), [2, 4, 6]);

    c *= 3;
    assert_eq!(c.into_array(), [6, 12, 18]);

    c /= 3;
    assert_eq!(c.into_array(), [2, 4, 6]);
}

#[test]
fn numeric_predicates_surface() {
    let zero = Vector::<f32, 3>::ZERO;
    let non_zero = Vector::<f32, 3>::from_array([0.0, 1.0, 0.0]);
    assert!(zero.is_zero());
    assert!(!non_zero.is_zero());
    assert!(<Vector<f32, 3> as IsZero>::is_zero(zero));
    assert!(!<Vector<f32, 3> as IsZero>::is_zero(non_zero));

    let all_finite = Vector::<f32, 3>::from_array([1.0, 2.0, 3.0]);
    let has_infinite = Vector::<f32, 3>::from_array([1.0, f32::INFINITY, 3.0]);
    let has_nan = Vector::<f32, 3>::from_array([1.0, f32::NAN, 3.0]);

    assert!(all_finite.is_finite());
    assert!(!has_infinite.is_finite());
    assert!(!has_nan.is_finite());

    assert!(!all_finite.is_infinite());
    assert!(has_infinite.is_infinite());

    assert!(!all_finite.is_nan());
    assert!(has_nan.is_nan());

    let inf = Vector::<f32, 3>::INFINITY;
    let neg_inf = Vector::<f32, 3>::NEG_INFINITY;
    let nan = Vector::<f32, 3>::NAN;
    assert!(inf.is_infinite());
    assert!(neg_inf.is_infinite());
    assert!(nan.is_nan());

    let inf_trait = <Vector<f32, 3> as Infinite>::INFINITY;
    let neg_inf_trait = <Vector<f32, 3> as Infinite>::NEG_INFINITY;
    let nan_trait = <Vector<f32, 3> as Nan>::NAN;
    assert!(inf_trait.is_infinite());
    assert!(neg_inf_trait.is_infinite());
    assert!(nan_trait.is_nan());

    assert!(<Vector<f32, 3> as IsFinite>::is_finite(all_finite));
}

#[test]
fn approx_eq_abs_surface() {
    let a = Vector::<f32, 3>::from_array([1.0, 2.0, 3.0]);
    let b = Vector::<f32, 3>::from_array([1.0 + 5e-7, 2.0 - 5e-7, 3.0 + 5e-7]);
    let c = Vector::<f32, 3>::from_array([1.0 + 2e-4, 2.0, 3.0]);

    assert!(approx_eql_abs!(a, b));
    assert!(!approx_eql_abs!(a, c));

    assert!(approx_eql_abs_tol!(a, b, 1e-6));
    assert!(!approx_eql_abs_tol!(a, c, 1e-6));

    assert!(approx_eql_abs!(a, b));
    assert!(!approx_eql_abs!(a, c));
}

#[test]
fn approx_eq_rel_surface() {
    let a = Vector::<f32, 3>::from_array([10.0, 20.0, 30.0]);
    let b = Vector::<f32, 3>::from_array([10.0 + 5e-5, 20.0 - 5e-5, 30.0 + 5e-5]);
    let c = Vector::<f32, 3>::from_array([10.0 + 5e-2, 20.0, 30.0]);

    assert!(approx_eql_rel!(a, b));
    assert!(!approx_eql_rel!(a, c));

    assert!(approx_eql_rel_tol!(a, b, 1e-4));
    assert!(!approx_eql_rel_tol!(a, c, 1e-4));

    assert!(approx_eql_rel!(a, b));
    assert!(!approx_eql_rel!(a, c));
}

#[test]
fn dot_surface() {
    let a = Vector::<i32, 3>::from_array([1, 2, 3]);
    let b = Vector::<i32, 3>::from_array([4, 5, 6]);

    assert_eq!(a.dot(b), 32);
    assert_eq!(<Vector<i32, 3> as Dot>::dot(a, b), 32);
}

#[test]
fn length_and_normalize_surface() {
    let v = Vector::<f32, 2>::from_array([3.0, 4.0]);

    assert!(approx_eql_abs_tol!(v.length_squared(), 25.0, 1e-6));
    assert!(approx_eql_abs_tol!(<Vector<f32, 2> as LengthSquared>::length_squared(v), 25.0, 1e-6));

    assert!(approx_eql_abs_tol!(v.length(), 5.0, 1e-6));
    assert!(approx_eql_abs_tol!(<Vector<f32, 2> as Length>::length(v), 5.0, 1e-6));

    let n = v.normalize();
    let nt = <Vector<f32, 2> as Normalize>::normalize(v);

    assert!(approx_eql_abs_tol!(n.length(), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(nt.length(), 1.0, 1e-6));
    assert!(approx_eql_abs_tol!(n, Vector::from_array([0.6, 0.8]), 1e-6));
    assert!(approx_eql_abs_tol!(nt, Vector::from_array([0.6, 0.8]), 1e-6));

    let a = Vector::<i32, 3>::from_array([1, 2, 3]);
    assert_eq!(a.negate().into_array(), [-1, -2, -3]);
    assert_eq!(<Vector<i32, 3> as Negate>::negate(a).into_array(), [-1, -2, -3]);
}

#[test]
fn lerp_surface() {
    let a = Vector::<f32, 3>::from_array([0.0, 10.0, 20.0]);
    let b = Vector::<f32, 3>::from_array([10.0, 20.0, 30.0]);

    let inherent = a.lerp(b, 0.25);
    let trait_path = <Vector<f32, 3> as Lerp>::lerp(a, b, 0.25);
    let expected = Vector::<f32, 3>::from_array([2.5, 12.5, 22.5]);

    assert!(approx_eql_abs_tol!(inherent, expected, 1e-6));
    assert!(approx_eql_abs_tol!(trait_path, expected, 1e-6));
    assert!(approx_eql_abs_tol!(inherent, trait_path, 1e-6));
}

#[cfg(feature = "bytemuck")]
#[test]
fn bytemuck_roundtrip() {
    let v = Vector::<i32, 4>::from_array([1, 2, 3, 4]);
    let bytes = bytemuck::bytes_of(&v);
    let out = bytemuck::pod_read_unaligned::<Vector<i32, 4>>(bytes);
    assert_eq!(out, v);
}

#[cfg(feature = "zerocopy")]
#[test]
fn zerocopy_roundtrip() {
    let v = Vector::<i32, 4>::from_array([5, 6, 7, 8]);
    let bytes = <Vector<i32, 4> as zerocopy::IntoBytes>::as_bytes(&v);
    let out = <Vector<i32, 4> as zerocopy::FromBytes>::read_from_bytes(bytes)
        .expect("valid vector bytes");
    assert_eq!(out, v);
}

#[cfg(feature = "sakka")]
#[test]
fn sakka_roundtrip() {
    let v = Vector::<i32, 4>::from_array([9, 10, 11, 12]);

    let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
    <Vector<i32, 4> as sakka::Encode>::encode(&v, &mut writer).expect("encode vector");
    let bytes = writer.finish();

    let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
    let out = <Vector<i32, 4> as sakka::Decode>::decode(&mut reader).expect("decode vector");

    assert_eq!(out, v);
    assert!(reader.is_eof());
}

#[test]
fn minmax_surface() {
    let a = Vector::<i32, 3>::from_array([5, 2, 8]);
    let b = Vector::<i32, 3>::from_array([3, 7, 1]);

    // Inherent method
    let min_result = a.min(b);
    let max_result = a.max(b);

    assert_eq!(min_result.into_array(), [3, 2, 1]);
    assert_eq!(max_result.into_array(), [5, 7, 8]);

    // Trait method
    let min_trait = <Vector<i32, 3> as MinMax>::minimum(a, b);
    let max_trait = <Vector<i32, 3> as MinMax>::maximum(a, b);

    assert_eq!(min_trait.into_array(), [3, 2, 1]);
    assert_eq!(max_trait.into_array(), [5, 7, 8]);

    // With floats
    let af = Vector::<f32, 2>::from_array([5.5, 2.2]);
    let bf = Vector::<f32, 2>::from_array([3.3, 7.7]);

    let min_f = af.min(bf);
    let max_f = af.max(bf);

    assert!(approx_eql_abs_tol!(min_f, Vector::<f32, 2>::from_array([3.3, 2.2]), 1e-6));
    assert!(approx_eql_abs_tol!(max_f, Vector::<f32, 2>::from_array([5.5, 7.7]), 1e-6));
}

#[test]
fn bounded_surface() {
    let min_int = Vector::<i32, 2>::MIN;
    let max_int = Vector::<i32, 2>::MAX;

    assert_eq!(min_int.into_array(), [i32::MIN, i32::MIN]);
    assert_eq!(max_int.into_array(), [i32::MAX, i32::MAX]);

    let min_float = Vector::<f32, 3>::MIN;
    let max_float = Vector::<f32, 3>::MAX;

    assert_eq!(min_float.into_array(), [f32::MIN, f32::MIN, f32::MIN]);
    assert_eq!(max_float.into_array(), [f32::MAX, f32::MAX, f32::MAX]);

    // Trait access
    use crate::numeric::{BoundedMax, BoundedMin};
    let min_trait = <Vector<i32, 2> as BoundedMin>::MIN;
    let max_trait = <Vector<i32, 2> as BoundedMax>::MAX;

    assert_eq!(min_trait.into_array(), [i32::MIN, i32::MIN]);
    assert_eq!(max_trait.into_array(), [i32::MAX, i32::MAX]);
}

#[test]
fn abs_surface() {
    let v = Vector::<i32, 3>::from_array([-1, 2, -3]);
    assert_eq!(v.abs().into_array(), [1, 2, 3]);
    assert_eq!(<Vector<i32, 3> as Abs>::abs(v).into_array(), [1, 2, 3]);

    let a = Vector::<i32, 3>::from_array([1, 2, 3]);
    assert_eq!(a.negate().into_array(), [-1, -2, -3]);
    assert_eq!(<Vector<i32, 3> as Negate>::negate(a).into_array(), [-1, -2, -3]);

    let nf = Vector::<f32, 2>::from_array([-1.5, 2.5]);
    let abs_f = nf.abs();
    assert!(approx_eql_abs_tol!(abs_f, Vector::<f32, 2>::from_array([1.5, 2.5]), 1e-6));
}

#[test]
fn unit_vector_constants_surface() {
    // Vec2 unit vectors
    assert_eq!(Vec2::<i32>::X.into_array(), [1, 0]);
    assert_eq!(Vec2::<i32>::Y.into_array(), [0, 1]);
    assert_eq!(Vec2::<i32>::NEG_X.into_array(), [-1, 0]);
    assert_eq!(Vec2::<i32>::NEG_Y.into_array(), [0, -1]);

    // Vec3 unit vectors
    assert_eq!(Vec3::<i32>::X.into_array(), [1, 0, 0]);
    assert_eq!(Vec3::<i32>::Y.into_array(), [0, 1, 0]);
    assert_eq!(Vec3::<i32>::Z.into_array(), [0, 0, 1]);
    assert_eq!(Vec3::<i32>::NEG_X.into_array(), [-1, 0, 0]);
    assert_eq!(Vec3::<i32>::NEG_Y.into_array(), [0, -1, 0]);
    assert_eq!(Vec3::<i32>::NEG_Z.into_array(), [0, 0, -1]);

    // Floats
    assert_eq!(Vec2::<f32>::X.into_array(), [1.0, 0.0]);
    assert_eq!(Vec3::<f32>::Y.into_array(), [0.0, 1.0, 0.0]);
    assert_eq!(Vec3::<f32>::Z.into_array(), [0.0, 0.0, 1.0]);

    // AXES array
    let v2_axes = Vec2::<i32>::AXES;
    assert_eq!(v2_axes.len(), 2);
    assert_eq!(v2_axes[0].into_array(), [1, 0]);
    assert_eq!(v2_axes[1].into_array(), [0, 1]);

    let v3_axes = Vec3::<i32>::AXES;
    assert_eq!(v3_axes.len(), 3);
    assert_eq!(v3_axes[0].into_array(), [1, 0, 0]);
    assert_eq!(v3_axes[1].into_array(), [0, 1, 0]);
    assert_eq!(v3_axes[2].into_array(), [0, 0, 1]);
}

#[test]
fn cross_product_surface() {
    // Vec2 cross product returns a scalar (determinant)
    let a2 = Vec2::new(1_i32, 2);
    let b2 = Vec2::new(3_i32, 4);
    assert_eq!(a2.cross(b2), -2); // 1*4 - 2*3 = -2

    let c2 = Vec2::new(2_i32, 0);
    let d2 = Vec2::new(0_i32, 3);
    assert_eq!(c2.cross(d2), 6); // 2*3 - 0*0 = 6

    // Vec3 cross product returns a vector
    let a3 = Vec3::new(1_i32, 0, 0);
    let b3 = Vec3::new(0_i32, 1, 0);
    let cross_result = a3.cross(b3);
    assert_eq!(cross_result.into_array(), [0, 0, 1]);

    let a = Vec3::new(1_f32, 2.0, 3.0);
    let b = Vec3::new(4_f32, 5.0, 6.0);
    let cross = a.cross(b);
    assert!(approx_eql_abs_tol!(cross, Vec3::new(-3.0, 6.0, -3.0), 1e-6));

    // Standard basis vectors cross products
    let x = Vec3::new(1_f32, 0.0, 0.0);
    let y = Vec3::new(0_f32, 1.0, 0.0);
    let z = Vec3::new(0_f32, 0.0, 1.0);

    assert!(approx_eql_abs_tol!(x.cross(y), z, 1e-6));
    assert!(approx_eql_abs_tol!(y.cross(z), x, 1e-6));
    assert!(approx_eql_abs_tol!(z.cross(x), y, 1e-6));

    // Anti-commutativity: a × b = -(b × a)
    let result1 = a.cross(b);
    let result2 = b.cross(a);
    assert!(approx_eql_abs_tol!(result1, -result2, 1e-6));
}
