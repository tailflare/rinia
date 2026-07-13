#![cfg(test)]

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    algebra::{
        Cast, CastError, CastFrom, LossyCast, LossyCastFrom, TryCast, TryCastFrom, TryExactCast,
        TryExactCastFrom,
    },
    numeric::{Abs, BoundedMax, BoundedMin, Infinite, IsFinite, MinMax, Nan},
    tuple::{Tuple, TupleLike},
};

#[test]
fn array_roundtrip() {
    let t = Tuple::from_array([1_i32, 2, 3]);
    assert_eq!(t.as_array(), &[1, 2, 3]);
    assert_eq!(t.into_array(), [1, 2, 3]);
}

#[test]
fn splat_surface() {
    let t = Tuple::<i32, 4>::splat(7);
    assert_eq!(t.as_array(), &[7, 7, 7, 7]);
}

#[test]
fn iota_surface() {
    let t = Tuple::<f32, 5>::iota();
    assert_eq!(t.as_array(), &[0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn slice_access_surface() {
    let mut t = Tuple::from_array([1_i32, 2, 3]);
    assert_eq!(t.as_slice(), &[1, 2, 3]);

    t.as_mut_slice()[1] = 20;
    assert_eq!(t.as_slice(), &[1, 20, 3]);
}

#[test]
fn iter_surface() {
    let t = Tuple::from_array([1_i32, 2, 3]);
    let collected = t.iter().copied().collect::<Vec<i32>>();
    assert_eq!(collected, Vec::from([1, 2, 3]));

    let mut t2 = Tuple::from_array([1_i32, 2, 3]);
    for v in t2.iter_mut() {
        *v *= 2;
    }
    assert_eq!(t2.as_array(), &[2, 4, 6]);
}

#[test]
fn map_and_zip_surface() {
    let a = Tuple::from_array([1_i32, 2, 3]);
    let b = Tuple::from_array([4_i32, 5, 6]);

    let doubled = a.map(|x| x * 2);
    assert_eq!(doubled.as_array(), &[2, 4, 6]);

    let plus_one = a.map_ref(|x| x + 1);
    assert_eq!(plus_one.as_array(), &[2, 3, 4]);

    let summed = a.zip_with(&b, |x, y| x + y);
    assert_eq!(summed.as_array(), &[5, 7, 9]);

    let zipped = a.zip(&b);
    assert_eq!(zipped.as_array(), &[(&1, &4), (&2, &5), (&3, &6)]);
}

#[test]
fn indexing_surface() {
    let mut t = Tuple::from_array([10_i32, 20, 30]);
    assert_eq!(t[0], 10);
    assert_eq!(t[2], 30);

    t[1] = 99;
    assert_eq!(t.as_array(), &[10, 99, 30]);
}

#[test]
fn into_iterator_surface() {
    let t = Tuple::from_array([1_i32, 2, 3]);
    let owned = t.into_iter().collect::<Vec<i32>>();
    assert_eq!(owned, Vec::from([1, 2, 3]));

    let t2 = Tuple::from_array([4_i32, 5, 6]);
    let by_ref = (&t2).into_iter().copied().collect::<Vec<i32>>();
    assert_eq!(by_ref, Vec::from([4, 5, 6]));

    let mut t3 = Tuple::from_array([7_i32, 8, 9]);
    for v in &mut t3 {
        *v += 1;
    }
    assert_eq!(t3.as_array(), &[8, 9, 10]);
}

#[test]
fn conversion_and_default_surface() {
    let t: Tuple<i32, 3> = [1, 2, 3].into();
    assert_eq!(t.as_array(), &[1, 2, 3]);

    let arr: [i32; 3] = t.into();
    assert_eq!(arr, [1, 2, 3]);

    let d = Tuple::<i32, 3>::default();
    assert_eq!(d.as_array(), &[0, 0, 0]);
}

#[test]
fn tuplelike_surface() {
    let mut t = Tuple::from_array([1_i32, 2, 3]);
    assert_eq!(<Tuple<i32, 3> as TupleLike>::LEN, 3);
    assert_eq!(TupleLike::as_slice(&t), &[1, 2, 3]);

    TupleLike::as_mut_slice(&mut t)[0] = 11;
    assert_eq!(t.as_array(), &[11, 2, 3]);
}

#[test]
fn arithmetic_surface() {
    let a = Tuple::from_array([10_i32, 20, 30]);
    let b = Tuple::from_array([1_i32, 2, 3]);

    assert_eq!((-a).as_array(), &[-10, -20, -30]);
    assert_eq!((a + b).as_array(), &[11, 22, 33]);
    assert_eq!((a - b).as_array(), &[9, 18, 27]);
    assert_eq!((a * b).as_array(), &[10, 40, 90]);
    assert_eq!((a / b).as_array(), &[10, 10, 10]);
    assert_eq!((a % b).as_array(), &[0, 0, 0]);

    assert_eq!((a + 2).as_array(), &[12, 22, 32]);
    assert_eq!((a - 2).as_array(), &[8, 18, 28]);
    assert_eq!((a * 2).as_array(), &[20, 40, 60]);
    assert_eq!((a / 2).as_array(), &[5, 10, 15]);
    assert_eq!((a % 4).as_array(), &[2, 0, 2]);

    let mut c = Tuple::from_array([1_i32, 2, 3]);
    c += Tuple::from_array([2, 3, 4]);
    assert_eq!(c.as_array(), &[3, 5, 7]);
    c -= Tuple::from_array([1, 1, 1]);
    assert_eq!(c.as_array(), &[2, 4, 6]);
    c *= Tuple::from_array([2, 2, 2]);
    assert_eq!(c.as_array(), &[4, 8, 12]);
    c /= Tuple::from_array([2, 2, 3]);
    assert_eq!(c.as_array(), &[2, 4, 4]);
    c %= Tuple::from_array([2, 3, 3]);
    assert_eq!(c.as_array(), &[0, 1, 1]);

    c += 2;
    assert_eq!(c.as_array(), &[2, 3, 3]);
    c -= 1;
    assert_eq!(c.as_array(), &[1, 2, 2]);
    c *= 3;
    assert_eq!(c.as_array(), &[3, 6, 6]);
    c /= 3;
    assert_eq!(c.as_array(), &[1, 2, 2]);
    c %= 2;
    assert_eq!(c.as_array(), &[1, 0, 0]);
}

#[test]
fn numeric_predicates_surface() {
    let all_finite = Tuple::from_array([1.0_f32, 2.0, 3.0]);
    let has_infinite = Tuple::from_array([1.0_f32, f32::INFINITY, 3.0]);
    let has_nan = Tuple::from_array([1.0_f32, f32::NAN, 3.0]);

    assert!(all_finite.is_finite());
    assert!(!has_infinite.is_finite());
    assert!(!has_nan.is_finite());

    assert!(!all_finite.is_infinite());
    assert!(has_infinite.is_infinite());

    assert!(!all_finite.is_nan());
    assert!(has_nan.is_nan());

    let inf = Tuple::<f32, 3>::INFINITY;
    let neg_inf = Tuple::<f32, 3>::NEG_INFINITY;
    let nan = Tuple::<f32, 3>::NAN;
    assert!(inf.is_infinite());
    assert!(neg_inf.is_infinite());
    assert!(nan.is_nan());

    let inf_trait = <Tuple<f32, 3> as Infinite>::INFINITY;
    let neg_inf_trait = <Tuple<f32, 3> as Infinite>::NEG_INFINITY;
    let nan_trait = <Tuple<f32, 3> as Nan>::NAN;
    assert!(inf_trait.is_infinite());
    assert!(neg_inf_trait.is_infinite());
    assert!(nan_trait.is_nan());

    assert!(<Tuple<f32, 3> as IsFinite>::is_finite(all_finite));
}

#[test]
fn bounded_and_min_max_surface() {
    assert_eq!(<Tuple<i32, 3> as BoundedMin>::MIN.as_array(), &[i32::MIN; 3]);
    assert_eq!(<Tuple<i32, 3> as BoundedMax>::MAX.as_array(), &[i32::MAX; 3]);

    let a = Tuple::from_array([1_i32, 5, 3]);
    let b = Tuple::from_array([2_i32, 4, 7]);

    assert_eq!(a.min(b).as_array(), &[1, 4, 3]);
    assert_eq!(a.max(b).as_array(), &[2, 5, 7]);
    assert_eq!(<Tuple<i32, 3> as MinMax>::minimum(a, b).as_array(), &[1, 4, 3]);
    assert_eq!(<Tuple<i32, 3> as MinMax>::maximum(a, b).as_array(), &[2, 5, 7]);
}

#[test]
fn abs_surface() {
    let t = Tuple::from_array([-1_i32, 2, -3]);
    assert_eq!(t.abs().as_array(), &[1, 2, 3]);
    assert_eq!(<Tuple<i32, 3> as Abs>::abs(t).as_array(), &[1, 2, 3]);

    let a = Tuple::from_array([1_i32, 2, 3]);
    assert_eq!(a.negate().as_array(), &[-1, -2, -3]);
}

#[test]
fn approx_eq_abs_surface() {
    let a = Tuple::from_array([1.0_f32, 2.0, 3.0]);
    let b = Tuple::from_array([1.0_f32 + 5e-7, 2.0 - 5e-7, 3.0 + 5e-7]);
    let c = Tuple::from_array([1.0_f32 + 2e-4, 2.0, 3.0]);

    assert!(a.approx_eq_abs_tol(b, 1e-6));
    assert!(!a.approx_eq_abs_tol(c, 1e-6));
}

#[test]
fn approx_eq_rel_surface() {
    let a = Tuple::from_array([10.0_f32, 20.0, 30.0]);
    let b = Tuple::from_array([10.0_f32 + 5e-5, 20.0 - 5e-5, 30.0 + 5e-5]);
    let c = Tuple::from_array([10.0_f32 + 5e-2, 20.0, 30.0]);

    assert!(a.approx_eq_rel_tol(b, 1e-4));
    assert!(!a.approx_eq_rel_tol(c, 1e-4));
}

#[test]
fn length_squared_surface() {
    let t = Tuple::from_array([2_i32, 3, 6]);
    assert_eq!(t.length_squared(), 49);
}

#[cfg(feature = "bytemuck")]
#[test]
fn bytemuck_roundtrip() {
    let t = Tuple::from_array([1_i32, 2, 3, 4]);
    let bytes = bytemuck::bytes_of(&t);
    let out = bytemuck::pod_read_unaligned::<Tuple<i32, 4>>(bytes);
    assert_eq!(out, t);
}

#[cfg(feature = "zerocopy")]
#[test]
fn zerocopy_roundtrip() {
    let t = Tuple::from_array([5_i32, 6, 7, 8]);
    let bytes = <Tuple<i32, 4> as zerocopy::IntoBytes>::as_bytes(&t);
    let out =
        <Tuple<i32, 4> as zerocopy::FromBytes>::read_from_bytes(bytes).expect("valid tuple bytes");
    assert_eq!(out, t);
}

#[cfg(feature = "sakka")]
#[test]
fn sakka_roundtrip() {
    let t = Tuple::from_array([9_i32, 10, 11, 12]);

    let mut writer = sakka::Writer::new(sakka::Endian::Little, ());
    <Tuple<i32, 4> as sakka::Encode>::encode(&t, &mut writer).expect("encode tuple");
    let bytes = writer.finish();

    let mut reader = sakka::Reader::new(&bytes, sakka::Endian::Little, ());
    let out = <Tuple<i32, 4> as sakka::Decode>::decode(&mut reader).expect("decode tuple");

    assert_eq!(out, t);
    assert!(reader.is_eof());
}

#[test]
fn try_map_surface() {
    let t = Tuple::from_array([1_i32, 2, 3]);

    // Successful mapping
    let result = t.try_map(|x| -> Result<i32, String> { Ok(x * 2) });
    assert_eq!(result, Ok(Tuple::from_array([2_i32, 4, 6])));

    // Failed mapping
    let result = t.try_map(|x| -> Result<i32, String> {
        if x == 2 { Err("failed at 2".to_string()) } else { Ok(x * 10) }
    });
    assert_eq!(result, Err("failed at 2".to_string()));

    // Using CastError as the error type
    let result = t.try_map(|x| -> Result<u8, CastError> {
        if x > 255 { Err(CastError::OutOfRange) } else { Ok(x as u8) }
    });
    assert_eq!(result, Ok(Tuple::from_array([1_u8, 2, 3])));
}

#[test]
fn cast_variants_surface() {
    // Cast: infallible, lossless cast (widening)
    let t_i8 = Tuple::from_array([1_i8, 2, 3]);
    let cast_result = t_i8.cast::<i32>();
    assert_eq!(cast_result.as_array(), &[1_i32, 2, 3]);
    let cast_trait = <Tuple<i8, 3> as Cast<Tuple<i32, 3>>>::cast(t_i8);
    assert_eq!(cast_trait.as_array(), &[1_i32, 2, 3]);

    // LossyCast: infallible, potentially lossy cast
    let t_i32 = Tuple::from_array([300_i32, -1, 127]);
    let lossy_result = t_i32.lossy_cast::<u8>();
    assert_eq!(lossy_result.as_array(), &[44_u8, 255, 127]); // wraps
    let lossy_trait = <Tuple<i32, 3> as LossyCast<Tuple<u8, 3>>>::lossy_cast(t_i32);
    assert_eq!(lossy_trait.as_array(), &[44_u8, 255, 127]);

    // TryCast: fallible, accepts loss of precision for int-to-float
    let t_i32_narrow = Tuple::from_array([1_i32, 2, 255]);
    let try_result = t_i32_narrow.try_cast::<u8>();
    assert_eq!(try_result, Ok(Tuple::from_array([1_u8, 2, 255])));

    let try_fail = Tuple::from_array([256_i32, 2, 3]);
    assert!(try_fail.try_cast::<u8>().is_err());
    let try_trait = <Tuple<i32, 3> as TryCast<Tuple<u8, 3>>>::try_cast(t_i32_narrow);
    assert_eq!(try_trait, Ok(Tuple::from_array([1_u8, 2, 255])));

    // TryExactCast: fallible, must preserve value exactly
    let t_exact = Tuple::from_array([1_i32, 2, 100]);
    let exact_result = t_exact.try_exact_cast::<i64>();
    assert_eq!(exact_result, Ok(Tuple::from_array([1_i64, 2, 100])));

    let exact_fail = Tuple::from_array([300_i32, 2, 3]);
    assert_eq!(exact_fail.try_exact_cast::<u8>(), Err(CastError::OutOfRange));
    let exact_trait = <Tuple<i32, 3> as TryExactCast<Tuple<i64, 3>>>::try_exact_cast(t_exact);
    assert_eq!(exact_trait, Ok(Tuple::from_array([1_i64, 2, 100])));
}

#[test]
fn from_cast_variants_surface() {
    let cast_src = Tuple::from_array([1_i8, 2, 3]);
    assert_eq!(Tuple::<i32, 3>::cast_from(cast_src), Tuple::from_array([1_i32, 2, 3]));
    assert_eq!(
        <Tuple<i32, 3> as CastFrom<Tuple<i8, 3>>>::cast_from(cast_src),
        Tuple::from_array([1_i32, 2, 3])
    );

    let lossy_src = Tuple::from_array([300_i32, -1, 127]);
    assert_eq!(Tuple::<u8, 3>::lossy_cast_from(lossy_src), Tuple::from_array([44_u8, 255, 127]));
    assert_eq!(
        <Tuple<u8, 3> as LossyCastFrom<Tuple<i32, 3>>>::lossy_cast_from(lossy_src),
        Tuple::from_array([44_u8, 255, 127])
    );

    let try_src = Tuple::from_array([1_i32, 2, 255]);
    assert_eq!(Tuple::<u8, 3>::try_cast_from(try_src), Ok(Tuple::from_array([1_u8, 2, 255])));
    assert_eq!(
        <Tuple<u8, 3> as TryCastFrom<Tuple<i32, 3>>>::try_cast_from(try_src),
        Ok(Tuple::from_array([1_u8, 2, 255]))
    );

    let exact_src = Tuple::from_array([1_i32, 2, 100]);
    assert_eq!(
        Tuple::<i64, 3>::try_exact_cast_from(exact_src),
        Ok(Tuple::from_array([1_i64, 2, 100]))
    );
    assert_eq!(
        <Tuple<i64, 3> as TryExactCastFrom<Tuple<i32, 3>>>::try_exact_cast_from(exact_src),
        Ok(Tuple::from_array([1_i64, 2, 100]))
    );
}
