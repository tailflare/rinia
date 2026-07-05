macro_rules! impl_vector_arith_traits {
    ($wrapper:ident<$scalar:ident : $bound:path>) => {
        impl<$scalar> ::core::ops::Add for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for (dst, src) in out.iter_mut().zip(<$wrapper<$scalar>>::as_slice(&rhs).iter()) {
                    *dst += *src;
                }
                <$wrapper<$scalar>>::from_array(out)
            }
        }

        impl<$scalar> ::core::ops::AddAssign for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                for (dst, src) in <$wrapper<$scalar>>::as_mut_slice(self)
                    .iter_mut()
                    .zip(<$wrapper<$scalar>>::as_slice(&rhs).iter())
                {
                    *dst += *src;
                }
            }
        }

        impl<$scalar> ::core::ops::Sub for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for (dst, src) in out.iter_mut().zip(<$wrapper<$scalar>>::as_slice(&rhs).iter()) {
                    *dst -= *src;
                }
                <$wrapper<$scalar>>::from_array(out)
            }
        }

        impl<$scalar> ::core::ops::SubAssign for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                for (dst, src) in <$wrapper<$scalar>>::as_mut_slice(self)
                    .iter_mut()
                    .zip(<$wrapper<$scalar>>::as_slice(&rhs).iter())
                {
                    *dst -= *src;
                }
            }
        }

        impl<$scalar> ::core::ops::Mul for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self::Output {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for (dst, src) in out.iter_mut().zip(<$wrapper<$scalar>>::as_slice(&rhs).iter()) {
                    *dst *= *src;
                }
                <$wrapper<$scalar>>::from_array(out)
            }
        }

        impl<$scalar> ::core::ops::MulAssign for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                for (dst, src) in <$wrapper<$scalar>>::as_mut_slice(self)
                    .iter_mut()
                    .zip(<$wrapper<$scalar>>::as_slice(&rhs).iter())
                {
                    *dst *= *src;
                }
            }
        }

        impl<$scalar> ::core::ops::Div for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for (dst, src) in out.iter_mut().zip(<$wrapper<$scalar>>::as_slice(&rhs).iter()) {
                    *dst /= *src;
                }
                <$wrapper<$scalar>>::from_array(out)
            }
        }

        impl<$scalar> ::core::ops::DivAssign for $wrapper<$scalar>
        where
            $scalar: $bound,
        {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                for (dst, src) in <$wrapper<$scalar>>::as_mut_slice(self)
                    .iter_mut()
                    .zip(<$wrapper<$scalar>>::as_slice(&rhs).iter())
                {
                    *dst /= *src;
                }
            }
        }

        impl<$scalar> ::core::ops::Neg for $wrapper<$scalar>
        where
            $scalar: $bound + ::core::ops::Neg<Output = $scalar>,
        {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self::Output {
                let mut out = <$wrapper<$scalar>>::to_array(self);
                for dst in out.iter_mut() {
                    *dst = -*dst;
                }
                <$wrapper<$scalar>>::from_array(out)
            }
        }
    };
}

pub(crate) use impl_vector_arith_traits;
