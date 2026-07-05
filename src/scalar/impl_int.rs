use crate::scalar::{
    macros_int::{impl_signed_int_ops_for, impl_unsigned_int_ops_for},
    type_lists::{with_sint_scalar_types, with_uint_scalar_types},
};

macro_rules! impl_uint_scalar_for_all_types {
	($($ty:ty),+ $(,)?) => {
		$(
			impl $crate::Scalar for $ty {
				const MAX: Self = <$ty>::MAX;
				const MIN: Self = <$ty>::MIN;
			}

			impl $crate::IntScalar for $ty {}
			impl $crate::UIntScalar for $ty {}
			impl_unsigned_int_ops_for!($ty, 0, 1);
		)+
	};
}

macro_rules! impl_sint_scalar_for_all_types {
	($($ty:ty),+ $(,)?) => {
		$(
			impl $crate::Scalar for $ty {
				const MAX: Self = <$ty>::MAX;
				const MIN: Self = <$ty>::MIN;
			}

			impl $crate::IntScalar for $ty {}
			impl $crate::SIntScalar for $ty {}
			impl_signed_int_ops_for!($ty, 0, 1);
		)+
	};
}

with_uint_scalar_types!(impl_uint_scalar_for_all_types);
with_sint_scalar_types!(impl_sint_scalar_for_all_types);
