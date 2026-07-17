macro_rules! _impl_scalar_constants {
	($( [$ty:ty $(, $name:ident : $value:expr)* $(,)?] ),+ $(,)?) => {
		$(
			$crate::scalar::_impl_scalar_constants!(@emit_for_type $ty; $($name : $value,)*);
		)+
	};

	(@emit_for_type $ty:ty; $($name:ident : $value:expr,)*) => {
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_zero $ty; $($name : $value,)*);
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_one $ty; $($name : $value,)*);
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_two $ty; $($name : $value,)*);
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_neg_one $ty; $($name : $value,)*);
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_half $ty; $($name : $value,)*);
	};

	(@maybe_impl_zero $ty:ty; zero: $zero:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::Zero for $ty {
			const ZERO: Self = $zero;
		}
	};
	(@maybe_impl_zero $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_zero $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_zero $ty:ty;) => {};

	(@maybe_impl_one $ty:ty; one: $one:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::One for $ty {
			const ONE: Self = $one;
		}
	};
	(@maybe_impl_one $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_one $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_one $ty:ty;) => {};

	(@maybe_impl_two $ty:ty; two: $two:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::Two for $ty {
			const TWO: Self = $two;
		}
	};
	(@maybe_impl_two $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_two $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_two $ty:ty;) => {};

	(@maybe_impl_neg_one $ty:ty; neg_one: $neg_one:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::NegOne for $ty {
			const NEG_ONE: Self = $neg_one;
		}
	};
	(@maybe_impl_neg_one $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_neg_one $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_neg_one $ty:ty;) => {};

	(@maybe_impl_half $ty:ty; half: $half:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::Half for $ty {
			const HALF: Self = $half;
		}
	};
	(@maybe_impl_half $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::_impl_scalar_constants!(@maybe_impl_half $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_half $ty:ty;) => {};
}

macro_rules! _impl_scalar_constants_float {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl $crate::numeric::Infinite for $ty {
                const INFINITY: Self = <$ty>::INFINITY;
                const NEG_INFINITY: Self = <$ty>::NEG_INFINITY;
            }

            impl $crate::numeric::Nan for $ty {
                const NAN: Self = <$ty>::NAN;
            }
        )+
    };
}

macro_rules! _impl_scalar_predicates_float {
	($($ty:ty),+ $(,)?) => {
		$(
            impl $crate::numeric::IsZero for $ty {
                #[inline]
                fn is_zero(self) -> bool {
                    self == <$ty as $crate::numeric::Zero>::ZERO
                }
		    }

			impl $crate::numeric::IsFinite for $ty {
				#[inline]
				fn is_finite(self) -> bool {
					<$ty>::is_finite(self)
				}
			}

            impl $crate::numeric::IsInfinite for $ty {
				#[inline]
				fn is_infinite(self) -> bool {
					<$ty>::is_infinite(self)
				}
			}

            impl $crate::numeric::IsNan for $ty {
				#[inline]
				fn is_nan(self) -> bool {
					<$ty>::is_nan(self)
				}
			}
		)+
	};
}

macro_rules! _impl_scalar_predicates_int {
	($($ty:ty),+ $(,)?) => {
		$(
            impl $crate::numeric::IsZero for $ty {
                #[inline]
                fn is_zero(self) -> bool {
                    self == <$ty as $crate::numeric::Zero>::ZERO
                }
		    }

			impl $crate::numeric::IsFinite for $ty {
				#[inline]
				fn is_finite(self) -> bool {
					true
				}
			}

            impl $crate::numeric::IsInfinite for $ty {
				#[inline]
				fn is_infinite(self) -> bool {
					false
				}
			}

	            impl $crate::numeric::IsNan for $ty {
				#[inline]
				fn is_nan(self) -> bool {
					false
				}
			}
		)+
	};
}

macro_rules! _impl_scalar_bounded {
	($( [$ty:ty, $min:expr, $max:expr] ),+ $(,)?) => {
		$(
			impl $crate::numeric::BoundedMin for $ty {
				const MIN: Self = $min;
			}

            impl $crate::numeric::BoundedMax for $ty {
				const MAX: Self = $max;
			}
		)+
	};
}

macro_rules! _impl_scalar_min_max {
	($($ty:ty),+ $(,)?) => {
		$(
			impl $crate::numeric::MinMax for $ty {
				#[inline]
				fn minimum(self, other: Self) -> Self {
					<$ty>::min(self, other)
				}

				#[inline]
				fn maximum(self, other: Self) -> Self {
					<$ty>::max(self, other)
				}
			}
		)+
	};
}

macro_rules! _impl_scalar_clamp {
	($($ty:ty),+ $(,)?) => {
		$(
			impl $crate::numeric::Clamp for $ty {
				#[inline]
				fn clamp_value(self, min: Self, max: Self) -> Self {
					if self < min {
						min
					} else if self > max {
						max
					} else {
						self
					}
				}
			}
		)+
	};
}

macro_rules! _impl_scalar_signed_equivalent {
	($($src:ty => $dst:ty),+ $(,)?) => {
		$(
			impl $crate::numeric::SignedEquivalent for $src {
				type Signed = $dst;
			}
		)+
	};
}

macro_rules! _impl_scalar_unsigned_equivalent {
	($($src:ty => $dst:ty),+ $(,)?) => {
		$(
			impl $crate::numeric::UnsignedEquivalent for $src {
				type Unsigned = $dst;
			}
		)+
	};
}

macro_rules! _impl_scalar_arithmetic_trait {
	($trait:ident, [$head:tt $(, $tail:tt)* $(,)?], {unary, $method:ident, output: $output:ty}) => {
		impl $crate::numeric::$trait for $head {
			type Output = $output;

			#[inline]
			fn $method(self) -> Self::Output {
				self.$method()
			}
		}

		$crate::scalar::_impl_scalar_arithmetic_trait!($trait, [$($tail),*], {unary, $method, output: $output});
	};

	($trait:ident, [], {unary, $method:ident, output: $output:ty}) => {};

	($trait:ident, [$head:ty $(, $tail:ty)* $(,)?], {binary, $method:ident, output: $output:ty}) => {
		impl $crate::numeric::$trait for $head {
			type Output = $output;

			#[inline]
			fn $method(self, rhs: Self) -> Self::Output {
				self.$method(rhs)
			}
		}

		$crate::scalar::_impl_scalar_arithmetic_trait!($trait, [$($tail),*], {binary, $method, output: $output});
	};

	($trait:ident, [], {binary, $method:ident, output: $output:ty}) => {};

	($trait:ident, [$head:ty $(, $tail:ty)* $(,)?], {ternary, $method:ident, $rhs1:ty, $rhs2:ty, output: $output:ty}) => {
		impl $crate::numeric::$trait for $head {
			type Output = $output;

			#[inline]
			fn $method(self, arg1: $rhs1, arg2: $rhs2) -> Self::Output {
				<$head>::$method(self, arg1, arg2)
			}
		}

		$crate::scalar::_impl_scalar_arithmetic_trait!($trait, [$($tail),*], {ternary, $method, $rhs1, $rhs2, output: $output});
	};

	($trait:ident, [], {ternary, $method:ident, $rhs1:ty, $rhs2:ty, output: $output:ty}) => {};
}

pub(crate) use _impl_scalar_arithmetic_trait;
pub(crate) use _impl_scalar_bounded;
pub(crate) use _impl_scalar_clamp;
pub(crate) use _impl_scalar_constants;
pub(crate) use _impl_scalar_constants_float;
pub(crate) use _impl_scalar_min_max;
pub(crate) use _impl_scalar_predicates_float;
pub(crate) use _impl_scalar_predicates_int;
pub(crate) use _impl_scalar_signed_equivalent;
pub(crate) use _impl_scalar_unsigned_equivalent;
