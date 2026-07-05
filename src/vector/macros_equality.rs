macro_rules! impl_vector_equality_traits {
    ($wrapper:ident<$scalar:ident : $bound:path>) => {
        impl<$scalar> ::approx::AbsDiffEq for $wrapper<$scalar>
        where
            $scalar: $bound + ::approx::AbsDiffEq<Epsilon = $scalar>,
        {
            type Epsilon = $scalar;

            #[inline]
            fn default_epsilon() -> Self::Epsilon {
                $scalar::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                <$wrapper<$scalar>>::as_slice(self)
                    .iter()
                    .zip(<$wrapper<$scalar>>::as_slice(other).iter())
                    .all(|(a, b)| $scalar::abs_diff_eq(a, b, epsilon))
            }
        }

        impl<$scalar> ::approx::RelativeEq for $wrapper<$scalar>
        where
            $scalar: $bound + ::approx::RelativeEq<Epsilon = $scalar>,
        {
            #[inline]
            fn default_max_relative() -> Self::Epsilon {
                $scalar::default_max_relative()
            }

            #[inline]
            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                <$wrapper<$scalar>>::as_slice(self)
                    .iter()
                    .zip(<$wrapper<$scalar>>::as_slice(other).iter())
                    .all(|(a, b)| $scalar::relative_eq(a, b, epsilon, max_relative))
            }
        }

        impl<$scalar> ::approx::UlpsEq for $wrapper<$scalar>
        where
            $scalar: $bound + ::approx::UlpsEq<Epsilon = $scalar>,
        {
            #[inline]
            fn default_max_ulps() -> u32 {
                $scalar::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                <$wrapper<$scalar>>::as_slice(self)
                    .iter()
                    .zip(<$wrapper<$scalar>>::as_slice(other).iter())
                    .all(|(a, b)| $scalar::ulps_eq(a, b, epsilon, max_ulps))
            }
        }
    };
}

pub(crate) use impl_vector_equality_traits;
