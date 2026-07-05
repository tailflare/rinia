use crate::scalar::{macros_float::impl_float_for_type, type_lists::with_float_scalar_types};

macro_rules! impl_float_scalar_for_all_types {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl $crate::Scalar for $ty {
                const MAX: Self = <$ty>::MAX;
                const MIN: Self = <$ty>::MIN;
            }

            impl $crate::FloatScalar for $ty {}
        )+
    };
}

with_float_scalar_types!(impl_float_scalar_for_all_types);

impl_float_for_type!(f32, 0.0_f32, 1.0_f32, f);
impl_float_for_type!(f64, 0.0_f64, 1.0_f64);
