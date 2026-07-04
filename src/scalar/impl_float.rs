#[allow(unused_imports)]
use core_maths::*;

use crate::scalar::{
    forwarders::{impl_min_max_ord_via_inherent_for, impl_one_for, impl_zero_for},
    type_lists::with_float_scalar_types,
};

macro_rules! impl_scalar_bounds_float {
    ($ty:ty) => {
        const MAX: Self = <$ty>::MAX;

        const MIN: Self = <$ty>::MIN;
    };
}

macro_rules! impl_float_base_for_type {
    ($ty:ty) => {
        impl $crate::Scalar for $ty {
            impl_scalar_bounds_float!($ty);
        }

        impl_zero_for!($ty, 0.0);
        impl_one_for!($ty, 1.0);
        impl_min_max_ord_via_inherent_for!($ty);

        impl $crate::FloatScalar for $ty {
            const INFINITY: Self = <$ty>::INFINITY;

            const NEG_INFINITY: Self = <$ty>::NEG_INFINITY;

            const NAN: Self = <$ty>::NAN;

            #[inline]
            fn is_nan(self) -> bool {
                <$ty>::is_nan(self)
            }

            #[inline]
            fn is_finite(self) -> bool {
                <$ty>::is_finite(self)
            }

            #[inline]
            fn is_infinite(self) -> bool {
                <$ty>::is_infinite(self)
            }
        }
    };
}

macro_rules! impl_float_scalar_for_all_types {
	($($ty:ty),+ $(,)?) => {
		$(
			impl_float_base_for_type!($ty);
		)+
	};
}

with_float_scalar_types!(impl_float_scalar_for_all_types);
