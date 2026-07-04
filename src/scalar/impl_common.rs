use crate::{Scalar, ops::HasScalar, scalar::type_lists::with_all_scalar_types};

macro_rules! impl_scalar_cast_for_all {
	([$($all:ty),+ $(,)?]) => {
		impl_scalar_cast_for_all!(@targets [$($all),+] ; [$($all),+]);
	};

	(@targets [$($all:ty),+] ; []) => {};
	(@targets [$($all:ty),+] ; [$head:ty $(, $tail:ty)* $(,)?]) => {
		impl_scalar_cast_for_all!(@sources $head ; [$($all),+]);
		impl_scalar_cast_for_all!(@targets [$($all),+] ; [$($tail),*]);
	};

	(@sources $target:ty ; [$($source:ty),+ $(,)?]) => {
		$(
			impl $crate::scalar::ScalarCast<$source> for $target {
				#[inline]
				fn from_scalar_impl(value: $source) -> Self {
					value as Self
				}

				#[inline]
				fn as_scalar_impl(&self) -> $source {
					*self as $source
				}
			}
		)+
	};
}

macro_rules! impl_scalar_cast_matrix_for_all {
	($($ty:ty),+ $(,)?) => {
		impl_scalar_cast_for_all!([$($ty),+]);
	};
}

with_all_scalar_types!(impl_scalar_cast_matrix_for_all);

impl<T: Scalar> HasScalar for T {
    type Scalar = T;
}
