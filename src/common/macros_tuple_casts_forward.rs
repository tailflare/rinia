macro_rules! impl_tuple_wrapper_casts {
	([$($impl_generics:tt)*], $outer:ty => $cast_outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		// Cast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements cast to type `U`.
			#[inline]
			pub fn cast<U>(self) -> $cast_outer
			where
				$item: $crate::numeric::Cast<U>,
			{
				<$cast_outer>::from_tuple(self.into_tuple().cast::<U>())
			}
		}

		// Cast trait
		impl<$($impl_generics)*, U> $crate::numeric::Cast<$cast_outer> for $outer
		where
			$item: $crate::numeric::Cast<U>,
		{
			#[inline]
			fn cast(self) -> $cast_outer {
				<$outer>::cast::<U>(self)
			}
		}

        // CastFrom inherent
		impl<$($impl_generics)*> $outer {
			/// Casts `value` into `Self`.
			#[inline]
			pub fn cast_from<U>(value: $cast_outer) -> Self
			where
				U: $crate::numeric::Cast<$item>,
			{
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::cast_from(value.into_tuple()))
			}
		}

		// LossyCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements lossy cast to type `U`.
			#[inline]
			pub fn lossy_cast<U>(self) -> $cast_outer
			where
				$item: $crate::numeric::LossyCast<U>,
			{
				<$cast_outer>::from_tuple(self.into_tuple().lossy_cast::<U>())
			}
		}

		// LossyCast trait
		impl<$($impl_generics)*, U> $crate::numeric::LossyCast<$cast_outer> for $outer
		where
			$item: $crate::numeric::LossyCast<U>,
		{
			#[inline]
			fn lossy_cast(self) -> $cast_outer {
				<$outer>::lossy_cast::<U>(self)
			}
		}

        // LossyCastFrom inherent
		impl<$($impl_generics)*> $outer {
			/// Lossy casts `value` into `Self`.
			#[inline]
			pub fn lossy_cast_from<U>(value: $cast_outer) -> Self
			where
				U: $crate::numeric::LossyCast<$item>,
			{
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::lossy_cast_from(value.into_tuple()))
			}
		}

        // SaturatingCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements saturating cast to type `U`.
			#[inline]
			pub fn saturating_cast<U>(self) -> $cast_outer
			where
				$item: $crate::numeric::SaturatingCast<U>,
			{
				<$cast_outer>::from_tuple(self.into_tuple().saturating_cast::<U>())
			}
		}

		// SaturatingCast trait
		impl<$($impl_generics)*, U> $crate::numeric::SaturatingCast<$cast_outer> for $outer
		where
			$item: $crate::numeric::SaturatingCast<U>,
		{
			#[inline]
			fn saturating_cast(self) -> $cast_outer {
				<$outer>::saturating_cast::<U>(self)
			}
		}

        // SaturatingCastFrom inherent
		impl<$($impl_generics)*> $outer {
			/// Saturating casts `value` into `Self`.
			#[inline]
			pub fn saturating_cast_from<U>(value: $cast_outer) -> Self
			where
				U: $crate::numeric::SaturatingCast<$item>,
			{
				Self::from_tuple($crate::tuple::Tuple::<$item, $len>::saturating_cast_from(value.into_tuple()))
			}
		}

		// TryCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements try cast to type `U`.
			#[inline]
			pub fn try_cast<U>(self) -> Result<$cast_outer, $crate::numeric::CastError>
			where
				$item: $crate::numeric::TryCast<U>,
			{
				self.into_tuple().try_cast::<U>().map(<$cast_outer>::from_tuple)
			}
		}

		// TryCast trait
		impl<$($impl_generics)*, U> $crate::numeric::TryCast<$cast_outer> for $outer
		where
			$item: $crate::numeric::TryCast<U>,
		{
			#[inline]
			fn try_cast(self) -> Result<$cast_outer, $crate::numeric::CastError> {
				<$outer>::try_cast::<U>(self)
			}
		}

        // TryCastFrom inherent
		impl<$($impl_generics)*> $outer {
			/// Try casts `value` into `Self`.
			#[inline]
			pub fn try_cast_from<U>(value: $cast_outer) -> Result<Self, $crate::numeric::CastError>
			where
				U: $crate::numeric::TryCast<$item>,
			{
				$crate::tuple::Tuple::<$item, $len>::try_cast_from(value.into_tuple())
					.map(Self::from_tuple)
			}
		}

		// TryExactCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements try exact cast to type `U`.
			#[inline]
			pub fn try_exact_cast<U>(self) -> Result<$cast_outer, $crate::numeric::CastError>
			where
				$item: $crate::numeric::TryExactCast<U>,
			{
				self.into_tuple().try_exact_cast::<U>().map(<$cast_outer>::from_tuple)
			}
		}

		// TryExactCast trait
		impl<$($impl_generics)*, U> $crate::numeric::TryExactCast<$cast_outer> for $outer
		where
			$item: $crate::numeric::TryExactCast<U>,
		{
			#[inline]
			fn try_exact_cast(self) -> Result<$cast_outer, $crate::numeric::CastError> {
				<$outer>::try_exact_cast::<U>(self)
			}
		}

        // TryExactCastFrom inherent
		impl<$($impl_generics)*> $outer {
			/// Try exact casts `value` into `Self`.
			#[inline]
			pub fn try_exact_cast_from<U>(
				value: $cast_outer,
			) -> Result<Self, $crate::numeric::CastError>
			where
				U: $crate::numeric::TryExactCast<$item>,
			{
				$crate::tuple::Tuple::<$item, $len>::try_exact_cast_from(value.into_tuple())
					.map(Self::from_tuple)
			}
		}
	};
}

pub(crate) use impl_tuple_wrapper_casts;
