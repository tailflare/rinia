use crate::{
    common,
    scalar::{
        Float, FloatOps, Int, Scalar, ScalarOps, Signed, SignedInt, SignedOps, Unsigned,
        UnsignedInt,
    },
};

// Implement marker traits for all scalar types.
common::impl_marker_trait!(
    Scalar,
    [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,]
);

// Implement marker traits for all signed scalar types.
common::impl_marker_trait!(Signed, [isize, i8, i16, i32, i64, i128, f32, f64,]);

// Implement marker traits for all unsigned scalar types.
common::impl_marker_trait!(Unsigned, [usize, u8, u16, u32, u64, u128,]);

// Implement marker traits for all floating-point scalar types.
common::impl_marker_trait!(Float, [f32, f64,]);

// Implement marker traits for all integer scalar types.
common::impl_marker_trait!(Int, [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,]);

// Implement marker traits for all signed integer scalar types.
common::impl_marker_trait!(SignedInt, [isize, i8, i16, i32, i64, i128,]);

// Implement marker traits for all unsigned integer scalar types.
common::impl_marker_trait!(UnsignedInt, [usize, u8, u16, u32, u64, u128,]);

// Implement marker trait for scalar math for all scalar types.
common::impl_marker_trait!(
    ScalarOps,
    [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,]
);

// Implement marker trait for signed scalar math for all signed scalar types.
common::impl_marker_trait!(SignedOps, [isize, i8, i16, i32, i64, i128, f32, f64,]);

// Implement marker trait for floating-point math for all floating-point scalar types.
common::impl_marker_trait!(FloatOps, [f32, f64]);
