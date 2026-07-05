macro_rules! impl_dot_product_via_scalar_tuple {
    ($wrapper:ident<$scalar:ident : $bound:path>) => {
        impl<$scalar> $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            pub fn dot(self, rhs: Self) -> $scalar {
                let mut out = <$scalar as $crate::ops::Zero>::ZERO;
                for (lhs, rhs) in <$wrapper<$scalar>>::as_slice(&self)
                    .iter()
                    .zip(<$wrapper<$scalar>>::as_slice(&rhs).iter())
                {
                    out += *lhs * *rhs;
                }
                out
            }
        }

        impl<$scalar> $crate::ops::DotProduct for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Output = $scalar;

            #[inline]
            fn dot(self, rhs: Self) -> Self::Output {
                <$wrapper<$scalar>>::dot(self, rhs)
            }
        }
    };
}

pub(crate) use impl_dot_product_via_scalar_tuple;
