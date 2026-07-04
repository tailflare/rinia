macro_rules! with_float_scalar_types {
    ($m:ident) => {
        $m!(f32, f64);
    };
}

macro_rules! with_uint_scalar_types {
    ($m:ident) => {
        $m!(u8, u16, u32, u64, usize);
    };
}

macro_rules! with_sint_scalar_types {
    ($m:ident) => {
        $m!(i8, i16, i32, i64, isize);
    };
}

macro_rules! with_all_scalar_types {
    ($m:ident) => {
        $m!(f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
    };
}

pub(crate) use with_all_scalar_types;
pub(crate) use with_float_scalar_types;
pub(crate) use with_sint_scalar_types;
pub(crate) use with_uint_scalar_types;
