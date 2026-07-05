macro_rules! impl_unsigned_int_ops_for {
    ($ty:ty, $zero:expr, $one:expr) => {
        impl $crate::ops::Zero for $ty {
            const ZERO: Self = $zero;

            #[inline]
            fn is_zero(self) -> bool {
                self == Self::ZERO
            }
        }

        impl $crate::ops::One for $ty {
            const ONE: Self = $one;
        }

        impl $crate::ops::Select for $ty {
            #[inline]
            fn min_val(self, other: Self) -> Self {
                <$ty>::min(self, other)
            }

            #[inline]
            fn max_val(self, other: Self) -> Self {
                <$ty>::max(self, other)
            }
        }

        impl $crate::ops::Abs for $ty {
            type Output = Self;

            #[inline]
            fn abs(self) -> Self::Output {
                self
            }
        }
    };
}

pub(crate) use impl_unsigned_int_ops_for;

macro_rules! impl_signed_int_ops_for {
    ($ty:ty, $zero:expr, $one:expr) => {
        impl $crate::ops::Zero for $ty {
            const ZERO: Self = $zero;

            #[inline]
            fn is_zero(self) -> bool {
                self == Self::ZERO
            }
        }

        impl $crate::ops::One for $ty {
            const ONE: Self = $one;
        }

        impl $crate::ops::Select for $ty {
            #[inline]
            fn min_val(self, other: Self) -> Self {
                <$ty>::min(self, other)
            }

            #[inline]
            fn max_val(self, other: Self) -> Self {
                <$ty>::max(self, other)
            }
        }

        impl $crate::ops::Negate for $ty {
            #[inline]
            fn negate(self) -> Self {
                -self
            }
        }

        impl $crate::ops::Abs for $ty {
            type Output = Self;

            #[inline]
            fn abs(self) -> Self::Output {
                <$ty>::abs(self)
            }
        }
    };
}

pub(crate) use impl_signed_int_ops_for;
