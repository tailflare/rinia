use crate::scalar::{
    forwarders::{impl_min_max_ord_via_inherent_for, impl_one_for, impl_zero_for},
    type_lists::{with_sint_scalar_types, with_uint_scalar_types},
};

macro_rules! impl_scalar_bounds_int {
    ($ty:ty) => {
        const MAX: Self = <$ty>::MAX;

        const MIN: Self = <$ty>::MIN;
    };
}

macro_rules! impl_int_base_for_type {
    ($ty:ty) => {
        impl $crate::Scalar for $ty {
            impl_scalar_bounds_int!($ty);
        }

        impl_zero_for!($ty, 0);
        impl_one_for!($ty, 1);
        impl_min_max_ord_via_inherent_for!($ty);

        impl $crate::IntScalar for $ty {}
    };
}

macro_rules! impl_uint_scalar_for_all_types {
	($($ty:ty),+ $(,)?) => {
		$(
			impl_int_base_for_type!($ty);
			impl $crate::UIntScalar for $ty {}
		)+
	};
}

macro_rules! impl_sint_scalar_for_all_types {
	($($ty:ty),+ $(,)?) => {
		$(
			impl_int_base_for_type!($ty);
			impl $crate::SIntScalar for $ty {}
		)+
	};
}

with_uint_scalar_types!(impl_uint_scalar_for_all_types);
with_sint_scalar_types!(impl_sint_scalar_for_all_types);
