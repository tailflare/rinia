use crate::ScalarTuple;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct TestTuple3<T: crate::Scalar> {
    x: T,
    y: T,
    z: T,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct TestTuple4<T: crate::Scalar> {
    x: T,
    y: T,
    z: T,
    w: T,
}

crate::tuple::impl_scalar_tuple_traits!(TestTuple3<T: crate::Scalar>, 3);
crate::tuple::impl_scalar_tuple_traits!(TestTuple4<T: crate::Scalar>, 4);

#[test]
fn test_tuple3_len_and_splat() {
    assert_eq!(<TestTuple3<f32> as ScalarTuple>::LEN, 3);
    let v = TestTuple3::<f32>::splat(2.5);
    assert_eq!(v.to_array(), [2.5, 2.5, 2.5]);
}

#[test]
fn test_tuple4_slice_and_mut_slice_roundtrip() {
    let mut v = TestTuple4::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(v.as_slice(), &[1.0, 2.0, 3.0, 4.0]);

    let s = v.as_mut_slice();
    s[0] = 9.0;
    s[3] = -1.0;

    assert_eq!(v.to_array(), [9.0, 2.0, 3.0, -1.0]);
}

#[test]
fn test_tuple3_ptrs_match_storage() {
    let mut v = TestTuple3::<f32>::from_array([3.0, 4.0, 5.0]);
    let p = v.as_ptr();
    assert_eq!(p, v.as_slice().as_ptr());

    let mp = v.as_mut_ptr();
    assert_eq!(mp, v.as_mut_slice().as_mut_ptr());
}

#[test]
fn test_tuple3_index_and_index_mut() {
    let mut v = TestTuple3::<f32>::from_array([1.0, 2.0, 3.0]);
    assert_eq!(v[1], 2.0);
    v[1] = 7.0;
    assert_eq!(v.to_array(), [1.0, 7.0, 3.0]);
}

#[test]
fn test_tuple3_asref_asmut_and_from_into_array() {
    let mut v = TestTuple3::<f32>::from([1.0, 2.0, 3.0]);
    let r: &[f32] = v.as_ref();
    assert_eq!(r, &[1.0, 2.0, 3.0]);

    let m: &mut [f32] = v.as_mut();
    m[2] = 9.0;

    let arr: [f32; 3] = v.into();
    assert_eq!(arr, [1.0, 2.0, 9.0]);
}

#[test]
fn test_tuple4_len_and_conversion() {
    assert_eq!(<TestTuple4<f32> as ScalarTuple>::LEN, 4);
    let q = TestTuple4::<f32>::splat(0.5);
    assert_eq!(q.to_array(), [0.5, 0.5, 0.5, 0.5]);

    let q2 = TestTuple4::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    assert_eq!(q2.to_array(), [1.0, 2.0, 3.0, 4.0]);
}
