macro_rules! impl_int_scalar {
    ($($int:ty),+ $(,)?) => {
        $(
            impl $crate::Scalar for $int {
                const ZERO: Self = 0 as Self;

                const ONE: Self = 1 as Self;
            }

            impl $crate::scalar::MinMax for $int {
                #[inline]
                fn min(self, other: Self) -> Self {
                    if self < other { self } else { other }
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    if self > other { self } else { other }
                }
            }

            impl $crate::IntScalar for $int {}
        )+
    };
}

pub(crate) use impl_int_scalar;

macro_rules! impl_float_scalar {
    ($($float:ty),+ $(,)?) => {
        $(
            impl $crate::Scalar for $float {
                const ZERO: Self = 0.0;

                const ONE: Self = 1.0;
            }

            impl $crate::FloatScalar for $float {
                const INFINITY: Self = <$float>::INFINITY;

                const NEG_INFINITY: Self = <$float>::NEG_INFINITY;

                const NAN: Self = <$float>::NAN;

                #[inline]
                fn is_nan(self) -> bool {
                    <$float>::is_nan(self)
                }

                #[inline]
                fn is_finite(self) -> bool {
                    <$float>::is_finite(self)
                }

                #[inline]
                fn is_infinite(self) -> bool {
                    <$float>::is_infinite(self)
                }

                #[inline]
                fn is_zero(self) -> bool {
                    self == 0.0
                }
            }

            impl $crate::scalar::MinMax for $float {
                #[inline]
                fn min(self, other: Self) -> Self {
                    if self.is_nan() {
                        other
                    } else if other.is_nan() {
                        self
                    } else if self < other {
                        self
                    } else {
                        other
                    }
                }

                #[inline]
                fn max(self, other: Self) -> Self {
                    if self.is_nan() {
                        other
                    } else if other.is_nan() {
                        self
                    } else if self > other {
                        self
                    } else {
                        other
                    }
                }
            }
        )+
    };
}

pub(crate) use impl_float_scalar;

macro_rules! impl_from_scalar_for_all {
    ([$($all:ty),+ $(,)?]) => {
        impl_from_scalar_for_all!(@targets [$($all),+] ; [$($all),+]);
    };

    (@targets [$($all:ty),+] ; []) => {};
    (@targets [$($all:ty),+] ; [$head:ty $(, $tail:ty)* $(,)?]) => {
        impl_from_scalar_for_all!(@sources $head ; [$($all),+]);
        impl_from_scalar_for_all!(@targets [$($all),+] ; [$($tail),*]);
    };

    (@sources $target:ty ; [$($source:ty),+ $(,)?]) => {
        $(
            impl $crate::scalar::FromScalar<$source> for $target {
                #[inline]
                fn from_scalar(value: $source) -> Self {
                    value as Self
                }

                #[inline]
                fn as_scalar(&self) -> $source {
                    *self as $source
                }
            }
        )+
    };
}

pub(crate) use impl_from_scalar_for_all;
