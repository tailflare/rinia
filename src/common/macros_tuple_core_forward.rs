macro_rules! impl_tuple_wrapper_from_array {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, field: $field:ident $(,)?) => {
		impl<$($impl_generics)*> $outer {
			/// Creates a new wrapper from an array.
			#[inline]
			pub const fn from_array(inner: [$item; $len]) -> Self {
				Self { $field: $crate::tuple::Tuple::from_array(inner) }
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_from_tuple {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, field: $field:ident $(,)?) => {
		impl<$($impl_generics)*> $outer {
			/// Creates a new wrapper from a [Tuple].
			#[inline]
			pub const fn from_tuple(inner: $crate::tuple::Tuple<$item, $len>) -> Self {
				Self { $field: inner }
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_array_accessors {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, field: $field:ident $(,)?) => {
		impl<$($impl_generics)*> $outer {
			/// Returns a reference to the inner array.
			#[inline]
			pub const fn as_array(&self) -> &[$item; $len] {
				self.$field.as_array()
			}

			/// Consumes the wrapper and returns the inner array.
			#[inline]
			pub fn into_array(self) -> [$item; $len] {
				self.$field.into_array()
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_tuple_accessors {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr, field: $field:ident $(,)?) => {
		impl<$($impl_generics)*> $outer {
			/// Returns a reference to the inner [Tuple] field.
			#[inline]
			pub fn as_tuple(&self) -> &$crate::tuple::Tuple<$item, $len> {
				&self.$field
			}

			/// Returns a mutable reference to the inner [Tuple] field.
			#[inline]
			pub fn as_mut_tuple(&mut self) -> &mut $crate::tuple::Tuple<$item, $len> {
				&mut self.$field
			}

			/// Consumes the wrapper and returns the inner [Tuple] field.
			#[inline]
			pub fn into_tuple(self) -> $crate::tuple::Tuple<$item, $len> {
				self.$field
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_slice_accessors {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, field: $field:ident $(,)?) => {
		impl<$($impl_generics)*> $outer {
			/// Returns a slice of the elements.
			#[inline]
			pub fn as_slice(&self) -> &[$item] {
				self.$field.as_slice()
			}

			/// Returns a mutable slice of the elements.
			#[inline]
			pub fn as_mut_slice(&mut self) -> &mut [$item] {
				self.$field.as_mut_slice()
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_iter_accessors {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, field: $field:ident $(,)?) => {
		impl<$($impl_generics)*> $outer {
			/// Returns an iterator over the elements.
			#[inline]
			pub fn iter(&self) -> core::slice::Iter<'_, $item> {
				self.$field.iter()
			}

			/// Returns a mutable iterator over the elements.
			#[inline]
			pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, $item> {
				self.$field.iter_mut()
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_index_traits {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty $(,)?) => {
		impl<$($impl_generics)*> core::ops::Index<usize> for $outer {
			type Output = $item;

			#[inline]
			fn index(&self, index: usize) -> &Self::Output {
				&self.as_array()[index]
			}
		}

		impl<$($impl_generics)*> core::ops::IndexMut<usize> for $outer {
			#[inline]
			fn index_mut(&mut self, index: usize) -> &mut Self::Output {
				&mut self.as_mut_slice()[index]
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_into_iter_traits {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		impl<'a, $($impl_generics)*> core::iter::IntoIterator for &'a $outer {
			type Item = &'a $item;
			type IntoIter = core::slice::Iter<'a, $item>;

			#[inline]
			fn into_iter(self) -> Self::IntoIter {
				self.iter()
			}
		}

		impl<'a, $($impl_generics)*> core::iter::IntoIterator for &'a mut $outer {
			type Item = &'a mut $item;
			type IntoIter = core::slice::IterMut<'a, $item>;

			#[inline]
			fn into_iter(self) -> Self::IntoIter {
				self.iter_mut()
			}
		}

		impl<$($impl_generics)*> core::iter::IntoIterator for $outer {
			type Item = $item;
			type IntoIter = core::array::IntoIter<$item, $len>;

			#[inline]
			fn into_iter(self) -> Self::IntoIter {
				self.into_array().into_iter()
			}
		}
	};
}

macro_rules! impl_tuple_wrapper_from_array_trait {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		impl<$($impl_generics)*> core::convert::From<[$item; $len]> for $outer {
			#[inline]
			fn from(inner: [$item; $len]) -> Self {
				Self::from_array(inner)
			}
		}

		impl<$($impl_generics)*> core::convert::From<$outer> for [$item; $len] {
			#[inline]
			fn from(value: $outer) -> Self {
				value.into_array()
			}
		}
	};
}

macro_rules! impl_tuplelike_for_wrapper {
	([$($impl_generics:tt)*], $outer:ty, item: $item:ty, len: $len:expr $(,)?) => {
		impl<$($impl_generics)*> $crate::tuple::TupleLike for $outer {
			type Item = $item;
			const LEN: usize = $len;

			#[inline]
			fn as_slice(&self) -> &[Self::Item] {
				self.as_slice()
			}

			#[inline]
			fn as_mut_slice(&mut self) -> &mut [Self::Item] {
				self.as_mut_slice()
			}
		}
	};
}

pub(crate) use impl_tuple_wrapper_array_accessors;
pub(crate) use impl_tuple_wrapper_from_array;
pub(crate) use impl_tuple_wrapper_from_array_trait;
pub(crate) use impl_tuple_wrapper_from_tuple;
pub(crate) use impl_tuple_wrapper_index_traits;
pub(crate) use impl_tuple_wrapper_into_iter_traits;
pub(crate) use impl_tuple_wrapper_iter_accessors;
pub(crate) use impl_tuple_wrapper_slice_accessors;
pub(crate) use impl_tuple_wrapper_tuple_accessors;
pub(crate) use impl_tuplelike_for_wrapper;
