macro_rules! impl_scalar_tuple_traits {
    ($wrapper:ident<$scalar:ident : $bound:path>, $len:expr) => {
        $crate::tuple::impl_scalar_tuple_traits!(@with_bound $wrapper<$scalar>, $len, $bound);
    };

    (@with_bound $wrapper:ident<$scalar:ident>, $len:expr, $bound:path) => {
        impl<$scalar> $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            /// Creates a new instance of the tuple type with all elements set to the given scalar
            /// value.
            #[inline]
            pub const fn splat(scalar: $scalar) -> Self {
                Self::from_array([scalar; $len])
            }

            /// Returns a raw pointer to the first element of the tuple.
            #[inline]
            pub const fn as_ptr(&self) -> *const $scalar {
                (self as *const Self) as *const $scalar
            }

            /// Returns a mutable raw pointer to the first element of the tuple.
            #[inline]
            pub const fn as_mut_ptr(&mut self) -> *mut $scalar {
                (self as *mut Self) as *mut $scalar
            }

            /// Returns a slice containing all elements of the tuple.
            #[inline]
            pub const fn as_slice(&self) -> &[$scalar] {
                unsafe { ::core::slice::from_raw_parts(Self::as_ptr(self), $len) }
            }

            /// Returns a mutable slice containing all elements of the tuple.
            #[inline]
            pub const fn as_mut_slice(&mut self) -> &mut [$scalar] {
                unsafe { ::core::slice::from_raw_parts_mut(Self::as_mut_ptr(self), $len) }
            }

            /// Converts the tuple into an array of its scalar elements.
            #[inline]
            pub const fn to_array(self) -> [$scalar; $len] {
                let mut out = ::core::mem::MaybeUninit::<[$scalar; $len]>::uninit();
                unsafe {
                    ::core::ptr::copy_nonoverlapping(
                        Self::as_ptr(&self),
                        out.as_mut_ptr() as *mut $scalar,
                        $len,
                    );
                    out.assume_init()
                }
            }

            /// Creates a new instance of the tuple type from an array of scalar elements.
            #[inline]
            pub const fn from_array(a: [$scalar; $len]) -> Self {
                let mut out = ::core::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::core::ptr::copy_nonoverlapping(
                        a.as_ptr(),
                        out.as_mut_ptr() as *mut $scalar,
                        $len,
                    );
                    out.assume_init()
                }
            }
        }

        impl<$scalar> $crate::ops::HasScalar for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Scalar = $scalar;
        }

        impl<$scalar> $crate::ScalarTuple for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            const LEN: usize = $len;

            type Array = [$scalar; $len];

            #[inline]
            fn splat(scalar: Self::Scalar) -> Self {
                <$wrapper<$scalar>>::splat(scalar)
            }

            #[inline]
            fn as_ptr(&self) -> *const Self::Scalar {
                <$wrapper<$scalar>>::as_ptr(self)
            }

            #[inline]
            fn as_mut_ptr(&mut self) -> *mut Self::Scalar {
                <$wrapper<$scalar>>::as_mut_ptr(self)
            }

            #[inline]
            fn as_slice(&self) -> &[Self::Scalar] {
                <$wrapper<$scalar>>::as_slice(self)
            }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [Self::Scalar] {
                <$wrapper<$scalar>>::as_mut_slice(self)
            }

            #[inline]
            #[allow(clippy::wrong_self_convention)]
            fn to_array(self) -> Self::Array {
                <$wrapper<$scalar>>::to_array(self)
            }

            #[inline]
            fn from_array(a: Self::Array) -> Self {
                <$wrapper<$scalar>>::from_array(a)
            }
        }

        impl<$scalar> ::core::ops::Index<usize> for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Output = $scalar;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                &<$wrapper<$scalar>>::as_slice(self)[index]
            }
        }

        impl<$scalar> ::core::ops::IndexMut<usize> for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut <$wrapper<$scalar>>::as_mut_slice(self)[index]
            }
        }

        impl<$scalar> ::core::convert::AsRef<[$scalar]> for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn as_ref(&self) -> &[$scalar] {
                <$wrapper<$scalar>>::as_slice(self)
            }
        }

        impl<$scalar> ::core::convert::AsMut<[$scalar]> for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [$scalar] {
                <$wrapper<$scalar>>::as_mut_slice(self)
            }
        }

        impl<$scalar> ::core::convert::From<[$scalar; $len]> for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn from(a: [$scalar; $len]) -> Self {
                <$wrapper<$scalar>>::from_array(a)
            }
        }

        impl<$scalar> ::core::convert::From<$wrapper<$scalar>> for [$scalar; $len]
        where
            $scalar: $bound,
        {
            #[inline]
            fn from(t: $wrapper<$scalar>) -> Self {
                <$wrapper<$scalar>>::to_array(t)
            }
        }
    };
}

pub(crate) use impl_scalar_tuple_traits;
