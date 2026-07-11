use crate::scalar;

// Implement scalar constants for all scalars
scalar::impl_scalar_constants!(
    [usize, zero: 0, one: 1, two: 2],
    [isize, zero: 0, one: 1, two: 2, neg_one: -1],
    [u8, zero: 0, one: 1, two: 2],
    [u16, zero: 0, one: 1, two: 2],
    [u32, zero: 0, one: 1, two: 2],
    [u64, zero: 0, one: 1, two: 2],
    [u128, zero: 0, one: 1, two: 2],
    [i8, zero: 0, one: 1, two: 2, neg_one: -1],
    [i16, zero: 0, one: 1, two: 2, neg_one: -1],
    [i32, zero: 0, one: 1, two: 2, neg_one: -1],
    [i64, zero: 0, one: 1, two: 2, neg_one: -1],
    [i128, zero: 0, one: 1, two: 2, neg_one: -1],
    [f32, zero: 0.0, one: 1.0, two: 2.0, neg_one: -1.0, half: 0.5],
    [f64, zero: 0.0, one: 1.0, two: 2.0, neg_one: -1.0, half: 0.5],
);

// Implement ApproxEqAbs/ApproxEqRel for floating scalar types.
scalar::impl_scalar_approx_eq!(
    [f32, abs: 1e-6, rel: 1e-5],
    [f64, abs: 1e-12, rel: 1e-10]
);

// Implement Lerp for floating scalar types.
scalar::impl_scalar_lerp!(f32, f64);
