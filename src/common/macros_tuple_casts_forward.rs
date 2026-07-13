macro_rules! impl_tuple_wrapper_casts {
	([$($impl_generics:tt)*], $outer:ty => $cast_outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		// Cast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements cast to type `U`.
			#[inline]
			pub fn cast<U>(self) -> $cast_outer
			where
				$item: $crate::algebra::Cast<U>,
			{
				<$cast_outer>::from_tuple(self.into_tuple().cast::<U>())
			}
		}

		// Cast trait
		impl<$($impl_generics)*, U> $crate::algebra::Cast<$cast_outer> for $outer
		where
			$item: $crate::algebra::Cast<U>,
		{
			#[inline]
			fn cast(self) -> $cast_outer {
				<$outer>::cast::<U>(self)
			}
		}

		// LossyCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements lossy cast to type `U`.
			#[inline]
			pub fn lossy_cast<U>(self) -> $cast_outer
			where
				$item: $crate::algebra::LossyCast<U>,
			{
				<$cast_outer>::from_tuple(self.into_tuple().lossy_cast::<U>())
			}
		}

		// LossyCast trait
		impl<$($impl_generics)*, U> $crate::algebra::LossyCast<$cast_outer> for $outer
		where
			$item: $crate::algebra::LossyCast<U>,
		{
			#[inline]
			fn lossy_cast(self) -> $cast_outer {
				<$outer>::lossy_cast::<U>(self)
			}
		}

		// TryCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements try cast to type `U`.
			#[inline]
			pub fn try_cast<U>(self) -> Result<$cast_outer, $crate::algebra::CastError>
			where
				$item: $crate::algebra::TryCast<U>,
			{
				self.into_tuple().try_cast::<U>().map(<$cast_outer>::from_tuple)
			}
		}

		// TryCast trait
		impl<$($impl_generics)*, U> $crate::algebra::TryCast<$cast_outer> for $outer
		where
			$item: $crate::algebra::TryCast<U>,
		{
			#[inline]
			fn try_cast(self) -> Result<$cast_outer, $crate::algebra::CastError> {
				<$outer>::try_cast::<U>(self)
			}
		}

		// TryExactCast inherent
		impl<$($impl_generics)*> $outer {
			/// Returns a new wrapper with all elements try exact cast to type `U`.
			#[inline]
			pub fn try_exact_cast<U>(self) -> Result<$cast_outer, $crate::algebra::CastError>
			where
				$item: $crate::algebra::TryExactCast<U>,
			{
				self.into_tuple().try_exact_cast::<U>().map(<$cast_outer>::from_tuple)
			}
		}

		// TryExactCast trait
		impl<$($impl_generics)*, U> $crate::algebra::TryExactCast<$cast_outer> for $outer
		where
			$item: $crate::algebra::TryExactCast<U>,
		{
			#[inline]
			fn try_exact_cast(self) -> Result<$cast_outer, $crate::algebra::CastError> {
				<$outer>::try_exact_cast::<U>(self)
			}
		}
	};
}

pub(crate) use impl_tuple_wrapper_casts;
