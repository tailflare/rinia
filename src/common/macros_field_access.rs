macro_rules! impl_layout_field_access {
	([$($impl_generics:tt)*], $outer:ty => $fields:ty $(,)?) => {
        // Field access deref impl
		impl<$($impl_generics)*> core::ops::Deref for $outer {
			type Target = $fields;

			#[inline]
			fn deref(&self) -> &Self::Target {
				// SAFETY:
				// The outer type is #[repr(transparent)] over the inner layout-compatible field struct.
				// The field struct is #[repr(C)] with the same fields in the same order.
				// Therefore both types have identical layout.
				unsafe { &*(self as *const Self as *const Self::Target) }
			}
		}

        // Field access deref_mut impl
		impl<$($impl_generics)*> core::ops::DerefMut for $outer {
			#[inline]
			fn deref_mut(&mut self) -> &mut Self::Target {
				// SAFETY:
				// The outer type is #[repr(transparent)] over the inner layout-compatible field struct.
				// The field struct is #[repr(C)] with the same fields in the same order.
				// Therefore both types have identical layout.
				unsafe { &mut *(self as *mut Self as *mut Self::Target) }
			}
		}
	};
}

pub(crate) use impl_layout_field_access;
