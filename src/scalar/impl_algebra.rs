use crate::scalar;

// Implement ApproxEqAbs/ApproxEqRel for floating scalar types.
scalar::_impl_scalar_approx_eq!(
    [f32, abs: 1e-6, rel: 1e-5],
    [f64, abs: 1e-12, rel: 1e-10]
);

// Implement Lerp for floating scalar types.
scalar::_impl_scalar_lerp!(f32, f64);
