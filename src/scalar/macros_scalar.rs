macro_rules! impl_scalar_elementary_trait {
	($trait:ident, [$head:tt $(, $tail:tt)* $(,)?], $($entry:tt),+ $(,)?) => {
		impl $crate::numeric::$trait for $head {
			$(
				$crate::scalar::impl_scalar_elementary_trait!(@entry_for_type $head, $entry);
			)+
		}

		$crate::scalar::impl_scalar_elementary_trait!($trait, [$($tail),*], $($entry),+);
	};

	($trait:ident, [], $($entry:tt),+ $(,)?) => {};

	(@entry_for_type $ty:tt, {unary, $method:ident}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method_no_fallback $ty, unary, $method);
	};

	(@entry_for_type $ty:tt, {unary_pair, $method:ident}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method_no_fallback $ty, unary_pair, $method);
	};

	(@entry_for_type $ty:tt, {binary, $method:ident, $rhs:ty}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method_no_fallback $ty, binary, $method, $rhs);
	};

	(@entry_for_type $ty:tt, {ternary, $method:ident, $rhs1:ty, $rhs2:ty}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method_no_fallback $ty, ternary, $method, $rhs1, $rhs2);
	};

	(@entry_for_type $ty:tt, {unary, $method:ident, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method $ty, unary, $method, $crate::scalar::impl_scalar_elementary_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};

	(@entry_for_type $ty:tt, {unary_pair, $method:ident, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method $ty, unary_pair, $method, $crate::scalar::impl_scalar_elementary_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};

	(@entry_for_type $ty:tt, {binary, $method:ident, $rhs:ty, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method $ty, binary, $method, $rhs, $crate::scalar::impl_scalar_elementary_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};

	(@entry_for_type $ty:tt, {ternary, $method:ident, $rhs1:ty, $rhs2:ty, [f32: $f32_fallback:expr, f64: $f64_fallback:expr]}) => {
		$crate::scalar::impl_scalar_elementary_trait!(@method $ty, ternary, $method, $rhs1, $rhs2, $crate::scalar::impl_scalar_elementary_trait!(@float_fallback_for_type $ty, [f32: $f32_fallback, f64: $f64_fallback]));
	};


	(@method $ty:ty, unary, $method:ident, $fallback:expr) => {
		#[inline]
		fn $method(self) -> Self {
			$crate::scalar::impl_scalar_elementary_trait!(@call_unary_with_fallback $ty, $method, self, $fallback)
		}
	};

	(@method_no_fallback $ty:ty, unary, $method:ident) => {
		#[inline]
		fn $method(self) -> Self {
			$crate::scalar::impl_scalar_elementary_trait!(@call_unary $ty, $method, self)
		}
	};

	(@method $ty:ty, unary_pair, $method:ident, $fallback:expr) => {
		#[inline]
		fn $method(self) -> (Self, Self) {
			$crate::scalar::impl_scalar_elementary_trait!(@call_unary_pair_with_fallback $ty, $method, self, $fallback)
		}
	};

	(@method_no_fallback $ty:ty, unary_pair, $method:ident) => {
		#[inline]
		fn $method(self) -> (Self, Self) {
			$crate::scalar::impl_scalar_elementary_trait!(@call_unary_pair $ty, $method, self)
		}
	};

	(@method $ty:ty, binary, $method:ident, $rhs:ty, $fallback:expr) => {
		#[inline]
		fn $method(self, other: $rhs) -> Self {
			$crate::scalar::impl_scalar_elementary_trait!(@call_binary_with_fallback $ty, $method, $rhs, self, other, $fallback)
		}
	};

	(@method_no_fallback $ty:ty, binary, $method:ident, $rhs:ty) => {
		#[inline]
		fn $method(self, other: $rhs) -> Self {
			$crate::scalar::impl_scalar_elementary_trait!(@call_binary $ty, $method, $rhs, self, other)
		}
	};

	(@method_no_fallback $ty:ty, ternary, $method:ident, $rhs1:ty, $rhs2:ty) => {
		#[inline]
		fn $method(self, arg1: $rhs1, arg2: $rhs2) -> Self {
			$crate::scalar::impl_scalar_elementary_trait!(@call_ternary_inherent $ty, $method, $rhs1, $rhs2, self, arg1, arg2)
		}
	};

	(@method $ty:ty, ternary, $method:ident, $rhs1:ty, $rhs2:ty, $fallback:expr) => {
		#[inline]
		fn $method(self, arg1: $rhs1, arg2: $rhs2) -> Self {
			$crate::scalar::impl_scalar_elementary_trait!(@call_ternary_with_fallback $ty, $method, $rhs1, $rhs2, self, arg1, arg2, $fallback)
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

	(@call_ternary_with_fallback $ty:ty, $method:ident, $rhs1:ty, $rhs2:ty, $value:expr, $arg1:expr, $arg2:expr, $fallback:expr) => {{
		#[cfg(feature = "std")]
		{
			let call: fn($ty, $rhs1, $rhs2) -> $ty = |x: $ty, y: $rhs1, z: $rhs2| <$ty>::$method(x, y, z);
			call($value, $arg1, $arg2)
		}

		#[cfg(not(feature = "std"))]
		{
			($fallback)($value, $arg1, $arg2)
		}
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

pub(crate) use impl_scalar_elementary_trait;
