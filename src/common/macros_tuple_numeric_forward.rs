macro_rules! impl_tuple_wrapper_finite {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::Finite + Copy,
		{
			#[inline]
			pub fn is_finite(self) -> bool {
				self.into_tuple().is_finite()
			}
		}

		impl<$($impl_generics)*> $crate::numeric::Finite for $outer
		where
			$item: $crate::numeric::Finite + Copy,
		{
			#[inline]
			fn is_finite(self) -> bool {
				<$outer>::is_finite(self)
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_infinite {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::Infinite + Copy,
		{
			pub const INFINITY: Self =
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::INFINITY);

			pub const NEG_INFINITY: Self =
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::NEG_INFINITY);

			#[inline]
			pub fn is_infinite(self) -> bool {
				self.into_tuple().is_infinite()
			}
		}

		impl<$($impl_generics)*> $crate::numeric::Infinite for $outer
		where
			$item: $crate::numeric::Infinite + Copy,
		{
			const INFINITY: Self = <$outer>::INFINITY;
			const NEG_INFINITY: Self = <$outer>::NEG_INFINITY;

			#[inline]
			fn is_infinite(self) -> bool {
				<$outer>::is_infinite(self)
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_nan {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::Nan + Copy,
		{
			pub const NAN: Self = Self::from_tuple($crate::tuple::Tuple::<$item, $len>::NAN);

			#[inline]
			pub fn is_nan(self) -> bool {
				self.into_tuple().is_nan()
			}
		}

		impl<$($impl_generics)*> $crate::numeric::Nan for $outer
		where
			$item: $crate::numeric::Nan + Copy,
		{
			const NAN: Self = <$outer>::NAN;

			#[inline]
			fn is_nan(self) -> bool {
				<$outer>::is_nan(self)
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_numeric_predicates {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		$crate::common::impl_tuple_wrapper_finite!([$($impl_generics)*], $outer, item: $item, len: $len);
		$crate::common::impl_tuple_wrapper_infinite!([$($impl_generics)*], $outer, item: $item, len: $len);
		$crate::common::impl_tuple_wrapper_nan!([$($impl_generics)*], $outer, item: $item, len: $len);
	};
}

pub(crate) use impl_tuple_wrapper_finite;
pub(crate) use impl_tuple_wrapper_infinite;
pub(crate) use impl_tuple_wrapper_nan;
pub(crate) use impl_tuple_wrapper_numeric_predicates;
