use crate::scalar;

// Implement safe casts between scalar types.
scalar::impl_scalar_infallible_casts!(
    usize => [usize],
    isize => [isize],
    u8 => [u8, u16, u32, u64, u128],
    u16 => [u16, u32, u64, u128],
    u32 => [u32, u64, u128],
    u64 => [u64, u128],
    u128 => [u128],
    i8 => [i8, i16, i32, i64, i128],
    i16 => [i16, i32, i64, i128],
    i32 => [i32, i64, i128],
    i64 => [i64, i128],
    i128 => [i128],
    f32 => [f32, f64],
    f64 => [f64]
);

// Implement lossy casts between all scalar types.
scalar::impl_scalar_lossy_casts!(
    usize => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    isize => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    u8 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    u16 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    u32 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    u64 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    u128 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    i8 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    i16 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    i32 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    i64 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    i128 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    f32 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ],
    f64 => [
        usize, isize,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        f32, f64
    ]
);

// Impl SaturatingCast for unsigned int to unsigned int
scalar::impl_scalar_saturating_cast_unsigned_to_unsigned!(
    usize => [usize, u8, u16, u32, u64, u128],
    u8 => [usize, u8, u16, u32, u64, u128],
    u16 => [usize, u8, u16, u32, u64, u128],
    u32 => [usize, u8, u16, u32, u64, u128],
    u64 => [usize, u8, u16, u32, u64, u128],
    u128 => [usize, u8, u16, u32, u64, u128]
);

// Impl SaturatingCast for signed int to signed int
scalar::impl_scalar_saturating_cast_signed_to_signed!(
    isize => [isize, i8, i16, i32, i64, i128],
    i8 => [isize, i8, i16, i32, i64, i128],
    i16 => [isize, i8, i16, i32, i64, i128],
    i32 => [isize, i8, i16, i32, i64, i128],
    i64 => [isize, i8, i16, i32, i64, i128],
    i128 => [isize, i8, i16, i32, i64, i128]
);

// Impl SaturatingCast for unsigned int to signed int
scalar::impl_scalar_saturating_cast_unsigned_to_signed!(
    usize => [isize, i8, i16, i32, i64, i128],
    u8 => [isize, i8, i16, i32, i64, i128],
    u16 => [isize, i8, i16, i32, i64, i128],
    u32 => [isize, i8, i16, i32, i64, i128],
    u64 => [isize, i8, i16, i32, i64, i128],
    u128 => [isize, i8, i16, i32, i64, i128]
);

// Impl SaturatingCast for signed int to unsigned int
scalar::impl_scalar_saturating_cast_signed_to_unsigned!(
    isize => [usize, u8, u16, u32, u64, u128],
    i8 => [usize, u8, u16, u32, u64, u128],
    i16 => [usize, u8, u16, u32, u64, u128],
    i32 => [usize, u8, u16, u32, u64, u128],
    i64 => [usize, u8, u16, u32, u64, u128],
    i128 => [usize, u8, u16, u32, u64, u128]
);

// Impl SaturatingCast for int to float
scalar::impl_scalar_saturating_cast_int_to_float!(
    usize => [f32, f64],
    isize => [f32, f64],
    u8 => [f32, f64],
    u16 => [f32, f64],
    u32 => [f32, f64],
    u64 => [f32, f64],
    u128 => [f32, f64],
    i8 => [f32, f64],
    i16 => [f32, f64],
    i32 => [f32, f64],
    i64 => [f32, f64],
    i128 => [f32, f64]
);

// Impl SaturatingCast for float to int
scalar::impl_scalar_saturating_cast_float_to_int!(
    f32 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    f64 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
);

// Impl SaturatingCast for float to float
scalar::impl_scalar_saturating_cast_float_to_float!(
    f32 => [f32, f64],
    f64 => [f32, f64]
);

// Impl TryCast for int to int
scalar::impl_scalar_try_cast_int_to_int!(
    usize => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    isize => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    u8 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    u16 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    u32 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    u64 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    u128 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    i8 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    i16 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    i32 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    i64 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    i128 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
);

// Impl TryCast for int to float
scalar::impl_scalar_try_cast_int_to_float!(
    usize => [f32, f64],
    isize => [f32, f64],
    u8 => [f32, f64],
    u16 => [f32, f64],
    u32 => [f32, f64],
    u64 => [f32, f64],
    u128 => [f32, f64],
    i8 => [f32, f64],
    i16 => [f32, f64],
    i32 => [f32, f64],
    i64 => [f32, f64],
    i128 => [f32, f64]
);

// Impl TryCast for float to int
scalar::impl_scalar_try_cast_float_to_int!(
    f32 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128],
    f64 => [usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128]
);

// Impl TryCast for float to float
scalar::impl_scalar_try_cast_float_to_float!(
    f32 => [f32, f64],
    f64 => [f32, f64]
);
