macro_rules! route_float_fn {
    (unary, $ty:ident, $value:expr, $std_method:ident, $libm_base:ident $(, $suffix:ident)?) => {{
        #[cfg(feature = "std")]
        {
            ::std::primitive::$ty::$std_method($value)
        }

        #[cfg(not(feature = "std"))]
        {
            ::pastey::paste! {
                $crate::external::libm::[<$libm_base $($suffix)?>]($value)
            }
        }
    }};

    (binary, $ty:ident, $value:expr, $arg:expr, $std_method:ident, $libm_base:ident $(, $suffix:ident)?) => {{
        #[cfg(feature = "std")]
        {
            ::std::primitive::$ty::$std_method($value, $arg)
        }

        #[cfg(not(feature = "std"))]
        {
            ::pastey::paste! {
                $crate::external::libm::[<$libm_base $($suffix)?>]($value, $arg)
            }
        }
    }};

    (ternary, $ty:ident, $value:expr, $arg1:expr, $arg2:expr, $std_method:ident, $libm_base:ident $(, $suffix:ident)?) => {{
        #[cfg(feature = "std")]
        {
            ::std::primitive::$ty::$std_method($value, $arg1, $arg2)
        }

        #[cfg(not(feature = "std"))]
        {
            ::pastey::paste! {
                $crate::external::libm::[<$libm_base $($suffix)?>]($value, $arg1, $arg2)
            }
        }
    }};
}

macro_rules! impl_float_for_type {
    (
        $ty:ident,
        $zero:expr,
        $one:expr
        $(,
            $suffix:ident
        )?
    ) => {
        ::pastey::paste! {
            impl $crate::ops::Abs for $ty {
                type Output = Self;

                #[inline]
                fn abs(self) -> Self::Output {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, abs, fabs $(, $suffix)?)
                }
            }

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

            impl $crate::ops::Rounding for $ty {
                #[inline]
                fn floor(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, floor, floor $(, $suffix)?)
                }

                #[inline]
                fn ceil(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, ceil, ceil $(, $suffix)?)
                }

                #[inline]
                fn trunc(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, trunc, trunc $(, $suffix)?)
                }

                #[inline]
                fn round(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, round, round $(, $suffix)?)
                }

                #[inline]
                fn fract(self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        ::std::primitive::$ty::fract(self)
                    }

                    #[cfg(not(feature = "std"))]
                    {
                        ::pastey::paste! {
                            $crate::external::libm::[<modf $($suffix)?>](self).0
                        }
                    }
                }
            }

            impl $crate::ops::NaN for $ty {
                const NAN: Self = <$ty>::NAN;

                #[inline]
                fn is_nan(self) -> bool {
                    <$ty>::is_nan(self)
                }
            }

            impl $crate::ops::Infinity for $ty {
                const INFINITY: Self = <$ty>::INFINITY;
                const NEG_INFINITY: Self = <$ty>::NEG_INFINITY;

                #[inline]
                fn is_finite(self) -> bool {
                    <$ty>::is_finite(self)
                }

                #[inline]
                fn is_infinite(self) -> bool {
                    <$ty>::is_infinite(self)
                }
            }

            impl $crate::scalar::Float for $ty {
                #[inline]
                fn sin(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, sin, sin $(, $suffix)?)
                }

                #[inline]
                fn cos(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, cos, cos $(, $suffix)?)
                }

                #[inline]
                fn tan(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, tan, tan $(, $suffix)?)
                }

                #[inline]
                fn sin_cos(self) -> (Self, Self) {
                    (self.sin(), self.cos())
                }

                #[inline]
                fn sqrt(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, sqrt, sqrt $(, $suffix)?)
                }

                #[inline]
                fn exp(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, exp, exp $(, $suffix)?)
                }

                #[inline]
                fn ln(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, ln, log $(, $suffix)?)
                }

                #[inline]
                fn powf(self, exp: Self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(binary, $ty, self, exp, powf, pow $(, $suffix)?)
                }

                #[inline]
                fn signum(self) -> Self {
                    <$ty>::signum(self)
                }

                #[inline]
                fn copysign(self, sign: Self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(binary, $ty, self, sign, copysign, copysign $(, $suffix)?)
                }

                #[inline]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(
                        ternary,
                        $ty,
                        self,
                        a,
                        b,
                        mul_add,
                        fma
                        $(, $suffix)?
                    )
                }

                #[inline]
                fn div_euclid(self, rhs: Self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        ::std::primitive::$ty::div_euclid(self, rhs)
                    }

                    #[cfg(not(feature = "std"))]
                    {
                        let q = self / rhs;
                        if rhs > $zero {
                            $crate::scalar::macros_float::route_float_fn!(unary, $ty, q, floor, floor $(, $suffix)?)
                        } else {
                            $crate::scalar::macros_float::route_float_fn!(unary, $ty, q, ceil, ceil $(, $suffix)?)
                        }
                    }
                }

                #[inline]
                fn rem_euclid(self, rhs: Self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        ::std::primitive::$ty::rem_euclid(self, rhs)
                    }

                    #[cfg(not(feature = "std"))]
                    {
                        let r = ::pastey::paste! {
                            $crate::external::libm::[<fmod $($suffix)?>](self, rhs)
                        };
                        if r < $zero {
                            r + <$ty as $crate::ops::Abs>::abs(rhs)
                        } else {
                            r
                        }
                    }
                }

                #[inline]
                fn powi(self, n: i32) -> Self {
                    #[cfg(feature = "std")]
                    {
                        ::std::primitive::$ty::powi(self, n)
                    }

                    #[cfg(not(feature = "std"))]
                    {
                        $crate::scalar::macros_float::route_float_fn!(binary, $ty, self, n as $ty, powf, pow $(, $suffix)?)
                    }
                }

                #[inline]
                fn exp2(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, exp2, exp2 $(, $suffix)?)
                }

                #[inline]
                fn log(self, base: Self) -> Self {
                    #[cfg(feature = "std")]
                    {
                        ::std::primitive::$ty::log(self, base)
                    }

                    #[cfg(not(feature = "std"))]
                    {
                        $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, ln, log $(, $suffix)?)
                            / $crate::scalar::macros_float::route_float_fn!(unary, $ty, base, ln, log $(, $suffix)?)
                    }
                }

                #[inline]
                fn log2(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, log2, log2 $(, $suffix)?)
                }

                #[inline]
                fn log10(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, log10, log10 $(, $suffix)?)
                }

                #[inline]
                fn cbrt(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, cbrt, cbrt $(, $suffix)?)
                }

                #[inline]
                fn hypot(self, other: Self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(binary, $ty, self, other, hypot, hypot $(, $suffix)?)
                }

                #[inline]
                fn asin(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, asin, asin $(, $suffix)?)
                }

                #[inline]
                fn acos(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, acos, acos $(, $suffix)?)
                }

                #[inline]
                fn atan(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, atan, atan $(, $suffix)?)
                }

                #[inline]
                fn atan2(self, other: Self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(binary, $ty, self, other, atan2, atan2 $(, $suffix)?)
                }

                #[inline]
                fn exp_m1(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, exp_m1, expm1 $(, $suffix)?)
                }

                #[inline]
                fn ln_1p(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, ln_1p, log1p $(, $suffix)?)
                }

                #[inline]
                fn sinh(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, sinh, sinh $(, $suffix)?)
                }

                #[inline]
                fn cosh(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, cosh, cosh $(, $suffix)?)
                }

                #[inline]
                fn tanh(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, tanh, tanh $(, $suffix)?)
                }

                #[inline]
                fn asinh(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, asinh, asinh $(, $suffix)?)
                }

                #[inline]
                fn acosh(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, acosh, acosh $(, $suffix)?)
                }

                #[inline]
                fn atanh(self) -> Self {
                    $crate::scalar::macros_float::route_float_fn!(unary, $ty, self, atanh, atanh $(, $suffix)?)
                }
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

        }
    };
}

pub(crate) use impl_float_for_type;
pub(crate) use route_float_fn;
