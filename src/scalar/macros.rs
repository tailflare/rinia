macro_rules! impl_scalar_constants {
	($( [$ty:ty $(, $name:ident : $value:expr)* $(,)?] ),+ $(,)?) => {
		$(
			$crate::scalar::impl_scalar_constants!(@emit_for_type $ty; $($name : $value,)*);
		)+
	};

	(@emit_for_type $ty:ty; $($name:ident : $value:expr,)*) => {
		$crate::scalar::impl_scalar_constants!(@maybe_impl_zero $ty; $($name : $value,)*);
		$crate::scalar::impl_scalar_constants!(@maybe_impl_one $ty; $($name : $value,)*);
		$crate::scalar::impl_scalar_constants!(@maybe_impl_two $ty; $($name : $value,)*);
		$crate::scalar::impl_scalar_constants!(@maybe_impl_neg_one $ty; $($name : $value,)*);
		$crate::scalar::impl_scalar_constants!(@maybe_impl_half $ty; $($name : $value,)*);
	};

	(@maybe_impl_zero $ty:ty; zero: $zero:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::Zero for $ty {
			const ZERO: Self = $zero;
		}
	};
	(@maybe_impl_zero $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::impl_scalar_constants!(@maybe_impl_zero $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_zero $ty:ty;) => {};

	(@maybe_impl_one $ty:ty; one: $one:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::One for $ty {
			const ONE: Self = $one;
		}
	};
	(@maybe_impl_one $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::impl_scalar_constants!(@maybe_impl_one $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_one $ty:ty;) => {};

	(@maybe_impl_two $ty:ty; two: $two:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::Two for $ty {
			const TWO: Self = $two;
		}
	};
	(@maybe_impl_two $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::impl_scalar_constants!(@maybe_impl_two $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_two $ty:ty;) => {};

	(@maybe_impl_neg_one $ty:ty; neg_one: $neg_one:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::NegOne for $ty {
			const NEG_ONE: Self = $neg_one;
		}
	};
	(@maybe_impl_neg_one $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::impl_scalar_constants!(@maybe_impl_neg_one $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_neg_one $ty:ty;) => {};

	(@maybe_impl_half $ty:ty; half: $half:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		impl $crate::numeric::Half for $ty {
			const HALF: Self = $half;
		}
	};
	(@maybe_impl_half $ty:ty; $other_name:ident : $other_value:expr, $($rest_name:ident : $rest_value:expr,)*) => {
		$crate::scalar::impl_scalar_constants!(@maybe_impl_half $ty; $($rest_name : $rest_value,)*);
	};
	(@maybe_impl_half $ty:ty;) => {};
}

macro_rules! impl_scalar_constants_float {
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

macro_rules! impl_scalar_predicates_float {
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

macro_rules! impl_scalar_predicates_int {
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

macro_rules! impl_scalar_bounded {
	($( [$ty:ty, $min:expr, $max:expr] ),+ $(,)?) => {
		$(
			impl $crate::numeric::Bounded for $ty {
				const MIN: Self = $min;
				const MAX: Self = $max;
			}
		)+
	};
}

macro_rules! impl_scalar_min_max {
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

macro_rules! impl_scalar_approx_eq {
	($([$ty:ty, abs: $default_abs_tol:expr, rel: $default_rel_tol:expr]),+ $(,)?) => {
		$(
			impl $crate::algebra::ApproxEqAbs for $ty {
				type Tolerance = $ty;
				const DEFAULT_TOLERANCE_ABS: Self::Tolerance = $default_abs_tol;

				#[inline]
				fn approx_eq_abs_tol(self, other: Self, tol: Self::Tolerance) -> bool {
					(self - other).abs() <= tol
				}
			}

			impl $crate::algebra::ApproxEqRel for $ty {
				type Tolerance = $ty;
				const DEFAULT_TOLERANCE_REL: Self::Tolerance = $default_rel_tol;

				#[inline]
				fn approx_eq_rel_tol(self, other: Self, tol: Self::Tolerance) -> bool {
					if self == other {
						return true;
					}

					(self - other).abs() <= tol * self.abs().max(other.abs())
				}
			}
		)+
	};
}

macro_rules! impl_scalar_lerp {
	($($ty:ty),+ $(,)?) => {
		$(
			impl $crate::algebra::Lerp for $ty {
				type Scalar = $ty;

				#[inline]
				fn lerp(self, rhs: Self, t: Self::Scalar) -> Self {
					self + (rhs - self) * t
				}
			}
		)+
	};
}

macro_rules! impl_scalar_clamp {
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

macro_rules! impl_scalar_trait {
	($trait:ident, [$head:tt $(, $tail:tt)* $(,)?], $($entry:tt),+ $(,)?) => {
		impl $crate::numeric::$trait for $head {
			$(
				$crate::scalar::impl_scalar_trait!(@entry_for_type $head, $entry);
			)+
		}

		$crate::scalar::impl_scalar_trait!($trait, [$($tail),*], $($entry),+);
	};

	($trait:ident, [], $($entry:tt),+ $(,)?) => {};

	(@entry_for_type $ty:tt, {unary, $method:ident}) => {
		$crate::scalar::impl_scalar_trait!(@method_no_fallback $ty, unary, $method);
	};

	(@entry_for_type $ty:tt, {unary_pair, $method:ident}) => {
		$crate::scalar::impl_scalar_trait!(@method_no_fallback $ty, unary_pair, $method);
	};

	(@entry_for_type $ty:tt, {binary, $method:ident, $rhs:ty}) => {
		$crate::scalar::impl_scalar_trait!(@method_no_fallback $ty, binary, $method, $rhs);
	};

	(@entry_for_type $ty:tt, {ternary, $method:ident, $rhs1:ty, $rhs2:ty}) => {
		$crate::scalar::impl_scalar_trait!(@method_no_fallback $ty, ternary, $method, $rhs1, $rhs2);
	};

	(@entry_for_type $ty:tt, {unary, $method:ident, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_trait!(@method $ty, unary, $method, $crate::scalar::impl_scalar_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};

	(@entry_for_type $ty:tt, {unary_pair, $method:ident, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_trait!(@method $ty, unary_pair, $method, $crate::scalar::impl_scalar_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};

	(@entry_for_type $ty:tt, {binary, $method:ident, $rhs:ty, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_trait!(@method $ty, binary, $method, $rhs, $crate::scalar::impl_scalar_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};


	(@method $ty:ty, unary, $method:ident, $fallback:expr) => {
		#[inline]
		fn $method(self) -> Self {
			$crate::scalar::impl_scalar_trait!(@call_unary_with_fallback $ty, $method, self, $fallback)
		}
	};

	(@method_no_fallback $ty:ty, unary, $method:ident) => {
		#[inline]
		fn $method(self) -> Self {
			$crate::scalar::impl_scalar_trait!(@call_unary $ty, $method, self)
		}
	};

	(@method $ty:ty, unary_pair, $method:ident, $fallback:expr) => {
		#[inline]
		fn $method(self) -> (Self, Self) {
			$crate::scalar::impl_scalar_trait!(@call_unary_pair_with_fallback $ty, $method, self, $fallback)
		}
	};

	(@method_no_fallback $ty:ty, unary_pair, $method:ident) => {
		#[inline]
		fn $method(self) -> (Self, Self) {
			$crate::scalar::impl_scalar_trait!(@call_unary_pair $ty, $method, self)
		}
	};

	(@method $ty:ty, binary, $method:ident, $rhs:ty, $fallback:expr) => {
		#[inline]
		fn $method(self, other: $rhs) -> Self {
			$crate::scalar::impl_scalar_trait!(@call_binary_with_fallback $ty, $method, $rhs, self, other, $fallback)
		}
	};

	(@method_no_fallback $ty:ty, binary, $method:ident, $rhs:ty) => {
		#[inline]
		fn $method(self, other: $rhs) -> Self {
			$crate::scalar::impl_scalar_trait!(@call_binary $ty, $method, $rhs, self, other)
		}
	};

	(@method_no_fallback $ty:ty, ternary, $method:ident, $rhs1:ty, $rhs2:ty) => {
		#[inline]
		fn $method(self, arg1: $rhs1, arg2: $rhs2) -> Self {
			$crate::scalar::impl_scalar_trait!(@call_ternary_inherent $ty, $method, $rhs1, $rhs2, self, arg1, arg2)
		}
	};

	(@call_unary $ty:ty, $method:ident, $value:expr) => {{
		let call: fn($ty) -> $ty = |x: $ty| x.$method();
		call($value)
	}};

	(@call_unary_with_fallback $ty:ty, $method:ident, $value:expr, $fallback:expr) => {{
		#[cfg(feature = "std")]
		{
			let call: fn($ty) -> $ty = |x: $ty| x.$method();
			call($value)
		}

		#[cfg(not(feature = "std"))]
		{
			($fallback)($value)
		}
	}};

	(@call_binary $ty:ty, $method:ident, $rhs:ty, $value:expr, $other:expr) => {{
		let call: fn($ty, $rhs) -> $ty = |x: $ty, y: $rhs| x.$method(y);
		call($value, $other)
	}};

	(@call_ternary_inherent $ty:ty, $method:ident, $rhs1:ty, $rhs2:ty, $value:expr, $arg1:expr, $arg2:expr) => {{
		let call: fn($ty, $rhs1, $rhs2) -> $ty = |x: $ty, y: $rhs1, z: $rhs2| <$ty>::$method(x, y, z);
		call($value, $arg1, $arg2)
	}};


	(@call_binary_with_fallback $ty:ty, $method:ident, $rhs:ty, $value:expr, $other:expr, $fallback:expr) => {{
		#[cfg(feature = "std")]
		{
			let call: fn($ty, $rhs) -> $ty = |x: $ty, y: $rhs| x.$method(y);
			call($value, $other)
		}

		#[cfg(not(feature = "std"))]
		{
			($fallback)($value, $other)
		}
	}};

	(@call_unary_pair $ty:ty, $method:ident, $value:expr) => {{
		let call: fn($ty) -> ($ty, $ty) = |x: $ty| x.$method();
		call($value)
	}};

	(@call_unary_pair_with_fallback $ty:ty, $method:ident, $value:expr, $fallback:expr) => {{
		#[cfg(feature = "std")]
		{
			let call: fn($ty) -> ($ty, $ty) = |x: $ty| x.$method();
			call($value)
		}

		#[cfg(not(feature = "std"))]
		{
			($fallback)($value)
		}
	}};

	(@float_fallback_for_type f32, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]) => {
		$f32_fallback
	};

	(@float_fallback_for_type f64, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]) => {
		$f64_fallback
	};

}

pub(crate) use impl_scalar_approx_eq;
pub(crate) use impl_scalar_bounded;
pub(crate) use impl_scalar_clamp;
pub(crate) use impl_scalar_constants;
pub(crate) use impl_scalar_constants_float;
pub(crate) use impl_scalar_lerp;
pub(crate) use impl_scalar_min_max;
pub(crate) use impl_scalar_predicates_float;
pub(crate) use impl_scalar_predicates_int;
pub(crate) use impl_scalar_trait;
