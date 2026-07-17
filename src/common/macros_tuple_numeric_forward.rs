macro_rules! _impl_tuple_wrapper_finite {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		// IsFinite inherent
        impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::IsFinite + Copy,
		{
            /// Returns `true` if all elements are finite, `false` otherwise.
			#[inline]
			pub fn is_finite(self) -> bool {
				self.into_tuple().is_finite()
			}
		}

        // IsFinite trait
		impl<$($impl_generics)*> $crate::numeric::IsFinite for $outer
		where
			$item: $crate::numeric::IsFinite + Copy,
		{
			#[inline]
			fn is_finite(self) -> bool {
				<$outer>::is_finite(self)
			}
		}
	};
}

macro_rules! _impl_tuple_wrapper_infinite {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		// Infinite inherent
        impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::Infinite + Copy,
		{
            /// Returns a new wrapper with all elements set to positive infinity.
			pub const INFINITY: Self =
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::INFINITY);

            /// Returns a new wrapper with all elements set to negative infinity.
			pub const NEG_INFINITY: Self =
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::NEG_INFINITY);
		}

        // Infinite trait
		impl<$($impl_generics)*> $crate::numeric::Infinite for $outer
		where
			$item: $crate::numeric::Infinite + Copy,
		{
			const INFINITY: Self = <$outer>::INFINITY;
			const NEG_INFINITY: Self = <$outer>::NEG_INFINITY;
		}

        // IsInfinite inherent
        impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::IsInfinite + Copy,
		{
            /// Returns `true` if any element is infinite, `false` otherwise.
			#[inline]
			pub fn is_infinite(self) -> bool {
				self.into_tuple().is_infinite()
			}
		}

        // IsInfinite trait
        impl<$($impl_generics)*> $crate::numeric::IsInfinite for $outer
		where
			$item: $crate::numeric::IsInfinite + Copy,
		{
			#[inline]
			fn is_infinite(self) -> bool {
				<$outer>::is_infinite(self)
			}
		}
	};
}

macro_rules! _impl_tuple_wrapper_nan {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		// Nan inherent
        impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::Nan + Copy,
		{
            /// Returns a new wrapper with all elements set to NaN.
			pub const NAN: Self = Self::from_tuple($crate::tuple::Tuple::<$item, $len>::NAN);

		}

        // Nan trait
		impl<$($impl_generics)*> $crate::numeric::Nan for $outer
		where
			$item: $crate::numeric::Nan + Copy,
		{
			const NAN: Self = <$outer>::NAN;
		}

        // IsNan inherent
        impl<$($impl_generics)*> $outer
		where
			$item: $crate::numeric::IsNan + Copy,
		{
            /// Returns `true` if any element is NaN, `false` otherwise.
			#[inline]
			pub fn is_nan(self) -> bool {
				self.into_tuple().is_nan()
			}
		}

        // IsNan trait
        impl<$($impl_generics)*> $crate::numeric::IsNan for $outer
		where
			$item: $crate::numeric::IsNan + Copy,
		{
			#[inline]
			fn is_nan(self) -> bool {
				<$outer>::is_nan(self)
			}
		}
	};
}

macro_rules! _impl_tuple_wrapper_numeric_predicates {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		$crate::common::_impl_tuple_wrapper_finite!([$($impl_generics)*], $outer, item: $item, len: $len);
		$crate::common::_impl_tuple_wrapper_infinite!([$($impl_generics)*], $outer, item: $item, len: $len);
		$crate::common::_impl_tuple_wrapper_nan!([$($impl_generics)*], $outer, item: $item, len: $len);
	};
}

pub(crate) use _impl_tuple_wrapper_finite;
pub(crate) use _impl_tuple_wrapper_infinite;
pub(crate) use _impl_tuple_wrapper_nan;
pub(crate) use _impl_tuple_wrapper_numeric_predicates;
