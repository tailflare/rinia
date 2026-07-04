macro_rules! impl_zero_for {
    ($ty:ty, $zero:expr) => {
        impl $crate::ops::Zero for $ty {
            const ZERO: Self = $zero;

            #[inline]
            fn is_zero(self) -> bool {
                self == Self::ZERO
            }
        }
    };
}

pub(crate) use impl_zero_for;

macro_rules! impl_one_for {
    ($ty:ty, $one:expr) => {
        impl $crate::ops::One for $ty {
            const ONE: Self = $one;
        }
    };
}

pub(crate) use impl_one_for;

macro_rules! impl_min_max_ord_via_inherent_for {
    ($ty:ty) => {
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
    };
}

pub(crate) use impl_min_max_ord_via_inherent_for;
