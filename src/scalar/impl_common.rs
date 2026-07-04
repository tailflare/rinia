use crate::scalar::macros::{impl_from_scalar_for_all, with_all_scalar_types};

macro_rules! impl_from_scalar_matrix_for_all_types {
	($($ty:ty),+ $(,)?) => {
		impl_from_scalar_for_all!([$($ty),+]);
	};
}

with_all_scalar_types!(impl_from_scalar_matrix_for_all_types);
