macro_rules! impl_scalar_infallible_casts {
    (
        $(
            $src:ty => [$($dst:ty),* $(,)?]
        ),* $(,)?
    ) => {
        $(
            $(
                impl $crate::numeric::Cast<$dst> for $src {
                    #[inline]
                    fn cast(self) -> $dst {
                        self as $dst
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_lossy_casts {
    (
        $(
            $src:ty => [$($dst:ty),* $(,)?]
        ),* $(,)?
    ) => {
        $(
            $(
                impl $crate::numeric::LossyCast<$dst> for $src {
                    #[inline]
                    fn lossy_cast(self) -> $dst {
                        self as $dst
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_unsigned_to_unsigned_wide {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        self as $dst
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_unsigned_to_unsigned_narrow {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self > <$dst>::MAX as $src {
                            <$dst>::MAX
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_signed_to_signed_narrow {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self < <$dst>::MIN as $src {
                            <$dst>::MIN
                        } else if self > <$dst>::MAX as $src {
                            <$dst>::MAX
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_signed_to_signed_wide {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        self as $dst
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_unsigned_to_signed {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self > <$dst>::MAX as $src {
                            <$dst>::MAX
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_signed_to_unsigned_wide {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self < 0 {
                            0
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_signed_to_unsigned_narrow {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self < 0 {
                            0
                        } else if self > <$dst>::MAX as $src {
                            <$dst>::MAX
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_int_to_float {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        self as $dst
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_float_to_int {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self.is_nan() {
                            0 as $dst
                        } else if self < <$dst>::MIN as $src {
                            <$dst>::MIN
                        } else if self > <$dst>::MAX as $src {
                            <$dst>::MAX
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_saturating_cast_float_to_float {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::SaturatingCast<$dst> for $src {
                    #[inline]
                    fn saturating_cast(self) -> $dst {
                        if self.is_nan() {
                            0.0
                        } else if self > <$dst>::MAX as $src {
                            <$dst>::MAX
                        } else if self < <$dst>::MIN as $src {
                            <$dst>::MIN
                        } else {
                            self as $dst
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_unsigned_cast {
    ($($src:ty => $dst:ty),* $(,)?) => {
        $(
            impl $crate::numeric::UnsignedCast<$dst> for $src {
                #[inline]
                fn unsigned_cast(self) -> $dst {
                    self as $dst
                }
            }
        )*
    };
}

macro_rules! impl_scalar_signed_cast {
    ($($src:ty => $dst:ty),* $(,)?) => {
        $(
            impl $crate::numeric::SignedCast<$dst> for $src {
                #[inline]
                fn signed_cast(self) -> $dst {
                    self as $dst
                }
            }
        )*
    };
}

macro_rules! impl_scalar_try_cast_int_to_int {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::TryExactCast<$dst> for $src {
                    #[inline]
                    fn try_exact_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        let value = self as $dst;

                        if value as $src == self {
                            Ok(value)
                        } else {
                            Err($crate::numeric::CastError::OutOfRange)
                        }
                    }
                }

                impl $crate::numeric::TryCast<$dst> for $src {
                    #[inline]
                    fn try_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        let value = self as $dst;

                        if value as $src == self {
                            Ok(value)
                        } else {
                            Err($crate::numeric::CastError::OutOfRange)
                        }
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_try_cast_int_to_float {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::TryExactCast<$dst> for $src {
                    #[inline]
                    fn try_exact_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        let value = self as $dst;

                        if value as $src == self {
                            Ok(value)
                        } else {
                            Err($crate::numeric::CastError::Inexact)
                        }
                    }
                }

                impl $crate::numeric::TryCast<$dst> for $src {
                    #[inline]
                    fn try_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                         Ok(self as $dst)
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_try_cast_float_to_int {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::TryExactCast<$dst> for $src where Self: $crate::numeric::Fract {
                    #[inline]
                    fn try_exact_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        if !self.is_finite() {
                            return Err($crate::numeric::CastError::NonFinite);
                        }

                        if <Self as $crate::numeric::Fract>::fract(self) != 0.0 {
                            return Err($crate::numeric::CastError::Fractional);
                        }

                        let value = self as $dst;

                        if value as $src == self {
                            Ok(value)
                        } else {
                            Err($crate::numeric::CastError::OutOfRange)
                        }
                    }
                }

                impl $crate::numeric::TryCast<$dst> for $src {
                    #[inline]
                    fn try_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        if !self.is_finite() {
                            return Err($crate::numeric::CastError::NonFinite);
                        }

                        if self < <$dst>::MIN as $src || self > <$dst>::MAX as $src {
                            return Err($crate::numeric::CastError::OutOfRange);
                        }

                        Ok(self as $dst)
                    }
                }
            )*
        )*
    };
}

macro_rules! impl_scalar_try_cast_float_to_float {
    ($($src:ty => [$($dst:ty),* $(,)?]),* $(,)?) => {
        $(
            $(
                impl $crate::numeric::TryExactCast<$dst> for $src {
                    #[inline]
                    fn try_exact_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        let value = self as $dst;

                        if value as $src == self {
                            Ok(value)
                        } else {
                            Err($crate::numeric::CastError::Inexact)
                        }
                    }
                }

                impl $crate::numeric::TryCast<$dst> for $src {
                    #[inline]
                    fn try_cast(self) -> Result<$dst, $crate::numeric::CastError> {
                        Ok(self as $dst)
                    }
                }
            )*
        )*
    };
}

pub(crate) use impl_scalar_infallible_casts;
pub(crate) use impl_scalar_lossy_casts;
pub(crate) use impl_scalar_saturating_cast_float_to_float;
pub(crate) use impl_scalar_saturating_cast_float_to_int;
pub(crate) use impl_scalar_saturating_cast_int_to_float;
pub(crate) use impl_scalar_saturating_cast_signed_to_signed_narrow;
pub(crate) use impl_scalar_saturating_cast_signed_to_signed_wide;
pub(crate) use impl_scalar_saturating_cast_signed_to_unsigned_narrow;
pub(crate) use impl_scalar_saturating_cast_signed_to_unsigned_wide;
pub(crate) use impl_scalar_saturating_cast_unsigned_to_signed;
pub(crate) use impl_scalar_saturating_cast_unsigned_to_unsigned_narrow;
pub(crate) use impl_scalar_saturating_cast_unsigned_to_unsigned_wide;
pub(crate) use impl_scalar_signed_cast;
pub(crate) use impl_scalar_try_cast_float_to_float;
pub(crate) use impl_scalar_try_cast_float_to_int;
pub(crate) use impl_scalar_try_cast_int_to_float;
pub(crate) use impl_scalar_try_cast_int_to_int;
pub(crate) use impl_scalar_unsigned_cast;
