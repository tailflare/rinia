use crate::{
    common,
    scalar::{
        CheckedIntNegOps, CheckedIntOps, Float, FloatOps, Int, IntOps, SaturatingIntNegOps,
        SaturatingIntOps, Scalar, ScalarOps, Signed, SignedInt, SignedIntOps, SignedOps, Unsigned,
        UnsignedInt, WrappingIntNegOps, WrappingIntOps,
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

// Implement marker trait for integer scalar math for all integer scalar types.
common::impl_marker_trait!(
    IntOps,
    [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,]
);

// Implement marker trait for signed integer scalar math for all signed integer scalar types.
common::impl_marker_trait!(SignedIntOps, [isize, i8, i16, i32, i64, i128]);

// Implement marker trait for saturating integer scalar math for all integer scalar types.
common::impl_marker_trait!(
    SaturatingIntOps,
    [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,]
);

// Implement marker trait for wrapping integer scalar math for all integer scalar types.
common::impl_marker_trait!(
    WrappingIntOps,
    [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,]
);

// Implement marker trait for checked integer scalar math for all integer scalar types.
common::impl_marker_trait!(
    CheckedIntOps,
    [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,]
);

// Implement marker trait for saturating negation for all signed integer scalar types.
common::impl_marker_trait!(SaturatingIntNegOps, [isize, i8, i16, i32, i64, i128]);

// Implement marker trait for wrapping negation for all signed integer scalar types.
common::impl_marker_trait!(WrappingIntNegOps, [isize, i8, i16, i32, i64, i128]);

// Implement marker trait for checked negation for all signed integer scalar types.
common::impl_marker_trait!(CheckedIntNegOps, [isize, i8, i16, i32, i64, i128]);
