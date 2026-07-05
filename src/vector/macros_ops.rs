macro_rules! impl_vector_ops_traits {
    ($wrapper:ident<$scalar:ident : $bound:path>) => {
        $crate::ops::impl_dot_product_via_scalar_tuple!($wrapper<$scalar: $bound>);

        impl<$scalar> $wrapper<$scalar>
        where
            $scalar: $bound + $crate::ops::Select,
        {
            /// Returns a vector containing the minimum values of each component of `self` and
            /// `other`.
            #[inline]
            pub fn min(self, other: Self) -> Self {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for (dst, src) in out.iter_mut().zip(<$wrapper<$scalar>>::as_slice(&other).iter()) {
                    *dst = <$scalar as $crate::ops::Select>::min_val(*dst, *src);
                }
                <$wrapper<$scalar>>::from_array(out)
            }

            /// Returns a vector containing the maximum values of each component of `self` and
            /// `other`.
            #[inline]
            pub fn max(self, other: Self) -> Self {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for (dst, src) in out.iter_mut().zip(<$wrapper<$scalar>>::as_slice(&other).iter()) {
                    *dst = <$scalar as $crate::ops::Select>::max_val(*dst, *src);
                }
                <$wrapper<$scalar>>::from_array(out)
            }
        }

        impl<$scalar> $crate::ops::Select for $wrapper<$scalar>
        where
            $scalar: $bound + $crate::ops::Select,
        {
            #[inline]
            fn min_val(self, other: Self) -> Self {
                self.min(other)
            }

            #[inline]
            fn max_val(self, other: Self) -> Self {
                self.max(other)
            }
        }

        impl<$scalar> $wrapper<$scalar>
        where
            $scalar: $bound + ::core::ops::Neg<Output = $scalar>,
        {
            /// Negates each component of the vector and returns a new vector.
            #[inline]
            pub fn negate(self) -> Self {
                -self
            }
        }

        impl<$scalar> $crate::ops::Negate for $wrapper<$scalar>
        where
            $scalar: $bound + ::core::ops::Neg<Output = $scalar>,
        {
            #[inline]
            fn negate(self) -> Self {
                -self
            }
        }

        impl<$scalar> $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            /// Returns a vector with all components set to the additive identity (zero).
            pub const ZERO: Self = Self::splat($scalar::ZERO);

            /// Checks if all components of the vector are equal to the additive identity (zero).
            #[inline]
            pub fn is_zero(self) -> bool {
                self == Self::ZERO
            }
        }

        impl<$scalar> $crate::ops::Zero for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            const ZERO: Self = <$wrapper<$scalar>>::ZERO;

            #[inline]
            fn is_zero(self) -> bool {
                <$wrapper<$scalar>>::is_zero(self)
            }
        }

        impl<$scalar> $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            /// Returns a vector with all components set to the multiplicative identity (one).
            pub const ONE: Self = Self::splat($scalar::ONE);
        }

        impl<$scalar> $crate::ops::One for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            const ONE: Self = <$wrapper<$scalar>>::ONE;
        }

        impl<$scalar> Default for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn default() -> Self {
                <$wrapper<$scalar>>::ZERO
            }
        }
    };
}

pub(crate) use impl_vector_ops_traits;
