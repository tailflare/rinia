/// Implements approximate-equality surfaces for wrapper types.
///
/// Generates inherent and trait impls for abs/rel comparisons in both `_tol`
/// and default-tolerance forms. Supports transparent wrappers, field forwarding,
/// and callback-based comparison logic with optional extra bounds.
#[macro_export]
macro_rules! impl_approx_eq_wrapper {
	(@emit_cmp abs, $lhs:ident, $rhs:ident, $tol:ident, [$($field:tt),+]) => {
		true $( && $crate::algebra::ApproxEqAbs::approx_eq_abs_tol($lhs.$field, $rhs.$field, $tol) )+
	};

	(@emit_cmp rel, $lhs:ident, $rhs:ident, $tol:ident, [$($field:tt),+]) => {
		true $( && $crate::algebra::ApproxEqRel::approx_eq_rel_tol($lhs.$field, $rhs.$field, $tol) )+
	};

	(@impl_trait
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		bounds: [$($bounds:tt)*],
		trait: $trait_name:ident,
		bound_trait: $bound_trait:ident,
		default_from_trait: $default_from_trait:ident,
		default_const: $default_const:ident,
		method: $method_name:ident,
		body: |$lhs:ident, $rhs:ident, $tol:ident| $expr:expr
	) => {
		impl<$($impl_generics)*> $crate::algebra::$trait_name for $outer
		where
			$item: Copy
				+ $crate::algebra::$bound_trait<Tolerance = $item>
				$($bounds)*
		{
			type Tolerance = $item;
			const $default_const: Self::Tolerance =
				<$item as $crate::algebra::$default_from_trait>::$default_const;

			#[inline]
			fn $method_name(self, other: Self, tol: Self::Tolerance) -> bool {
				let $lhs = self;
				let $rhs = other;
				let $tol = tol;
				$expr
			}
		}
	};

	(@impl_fields
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		bounds: [$($bounds:tt)*],
		fields: [$($field:tt),+ $(,)?]
	) => {
		// ApproxEqAbs inherents
		impl<$($impl_generics)*> $outer
		where
			$item: Copy
				+ $crate::algebra::ApproxEqAbs<Tolerance = $item>
				$($bounds)*
		{
			/// Checks if two values are approximately equal within a given absolute tolerance.
			#[inline]
			pub fn approx_eq_abs_tol(self, other: Self, tol: $item) -> bool {
				$crate::impl_approx_eq_wrapper!(
					@emit_cmp abs,
					self,
					other,
					tol,
					[$($field),+]
				)
			}

			/// Checks if two values are approximately equal using the default absolute tolerance.
			#[inline]
			pub fn approx_eq_abs(self, other: Self) -> bool {
				self.approx_eq_abs_tol(other, <$item as $crate::algebra::ApproxEqAbs>::DEFAULT_TOLERANCE_ABS)
			}
		}

		// ApproxEqRel inherents
		impl<$($impl_generics)*> $outer
		where
			$item: Copy
				+ $crate::algebra::ApproxEqRel<Tolerance = $item>
				$($bounds)*
		{
			/// Checks if two values are approximately equal within a given relative tolerance.
			#[inline]
			pub fn approx_eq_rel_tol(self, other: Self, tol: $item) -> bool {
				$crate::impl_approx_eq_wrapper!(
					@emit_cmp rel,
					self,
					other,
					tol,
					[$($field),+]
				)
			}

			/// Checks if two values are approximately equal using the default relative tolerance.
			#[inline]
			pub fn approx_eq_rel(self, other: Self) -> bool {
				self.approx_eq_rel_tol(other, <$item as $crate::algebra::ApproxEqRel>::DEFAULT_TOLERANCE_REL)
			}
		}

		// ApproxEqAbs trait
		$crate::impl_approx_eq_wrapper!(
			@impl_trait
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [$($bounds)*],
			trait: ApproxEqAbs,
			bound_trait: ApproxEqAbs,
			default_from_trait: ApproxEqAbs,
			default_const: DEFAULT_TOLERANCE_ABS,
			method: approx_eq_abs_tol,
			body: |lhs, rhs, tol| $crate::impl_approx_eq_wrapper!(
				@emit_cmp abs,
				lhs,
				rhs,
				tol,
				[$($field),+]
			)
		);

		// ApproxEqRel trait
		$crate::impl_approx_eq_wrapper!(
			@impl_trait
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [$($bounds)*],
			trait: ApproxEqRel,
			bound_trait: ApproxEqRel,
			default_from_trait: ApproxEqRel,
			default_const: DEFAULT_TOLERANCE_REL,
			method: approx_eq_rel_tol,
			body: |lhs, rhs, tol| $crate::impl_approx_eq_wrapper!(
				@emit_cmp rel,
				lhs,
				rhs,
				tol,
				[$($field),+]
			)
		);
	};

	(@impl_callback
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		bounds: [$($bounds:tt)*],
		compare_abs: |$abs_lhs:ident, $abs_rhs:ident, $abs_tol:ident| $abs_expr:expr,
		compare_rel: |$rel_lhs:ident, $rel_rhs:ident, $rel_tol:ident| $rel_expr:expr
	) => {
		// ApproxEqAbs inherents
		impl<$($impl_generics)*> $outer
		where
			$item: Copy
				+ $crate::algebra::ApproxEqAbs<Tolerance = $item>
				$($bounds)*
		{
			/// Checks if two values are approximately equal within a given absolute tolerance.
			#[inline]
			pub fn approx_eq_abs_tol(self, other: Self, tol: $item) -> bool {
				let $abs_lhs = self;
				let $abs_rhs = other;
				let $abs_tol = tol;
				$abs_expr
			}

			/// Checks if two values are approximately equal using the default absolute tolerance.
			#[inline]
			pub fn approx_eq_abs(self, other: Self) -> bool {
				self.approx_eq_abs_tol(other, <$item as $crate::algebra::ApproxEqAbs>::DEFAULT_TOLERANCE_ABS)
			}
		}

		// ApproxEqRel inherents
		impl<$($impl_generics)*> $outer
		where
			$item: Copy
				+ $crate::algebra::ApproxEqRel<Tolerance = $item>
				$($bounds)*
		{
			/// Checks if two values are approximately equal within a given relative tolerance.
			#[inline]
			pub fn approx_eq_rel_tol(self, other: Self, tol: $item) -> bool {
				let $rel_lhs = self;
				let $rel_rhs = other;
				let $rel_tol = tol;
				$rel_expr
			}

			/// Checks if two values are approximately equal using the default relative tolerance.
			#[inline]
			pub fn approx_eq_rel(self, other: Self) -> bool {
				self.approx_eq_rel_tol(other, <$item as $crate::algebra::ApproxEqRel>::DEFAULT_TOLERANCE_REL)
			}
		}

		// ApproxEqAbs trait
		$crate::impl_approx_eq_wrapper!(
			@impl_trait
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [$($bounds)*],
			trait: ApproxEqAbs,
			bound_trait: ApproxEqAbs,
			default_from_trait: ApproxEqAbs,
			default_const: DEFAULT_TOLERANCE_ABS,
			method: approx_eq_abs_tol,
			body: |$abs_lhs, $abs_rhs, $abs_tol| $abs_expr
		);

		// ApproxEqRel trait
		$crate::impl_approx_eq_wrapper!(
			@impl_trait
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [$($bounds)*],
			trait: ApproxEqRel,
			bound_trait: ApproxEqRel,
			default_from_trait: ApproxEqRel,
			default_const: DEFAULT_TOLERANCE_REL,
			method: approx_eq_rel_tol,
			body: |$rel_lhs, $rel_rhs, $rel_tol| $rel_expr
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		extra_bounds: [$($extra_bounds:tt)+],
		field: $field:tt
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			@impl_fields
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [+ $($extra_bounds)+],
			fields: [$field]
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		field: $field:tt
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			@impl_fields
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [],
			fields: [$field]
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		extra_bounds: [$($extra_bounds:tt)+],
		fields: [$($field:tt),+ $(,)?]
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			@impl_fields
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [+ $($extra_bounds)+],
			fields: [$($field),+]
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		fields: [$($field:tt),+ $(,)?]
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			@impl_fields
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [],
			fields: [$($field),+]
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		extra_bounds: [$($extra_bounds:tt)+]
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			extra_bounds: [$($extra_bounds)+],
			field: 0
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			field: 0
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		extra_bounds: [$($extra_bounds:tt)+],
		compare_abs: |$abs_lhs:ident, $abs_rhs:ident, $abs_tol:ident| $abs_expr:expr,
		compare_rel: |$rel_lhs:ident, $rel_rhs:ident, $rel_tol:ident| $rel_expr:expr
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			@impl_callback
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [+ $($extra_bounds)+],
			compare_abs: |$abs_lhs, $abs_rhs, $abs_tol| $abs_expr,
			compare_rel: |$rel_lhs, $rel_rhs, $rel_tol| $rel_expr
		);
	};

	(
		[$($impl_generics:tt)*],
		impl: $outer:ty,
		item: $item:ident,
		compare_abs: |$abs_lhs:ident, $abs_rhs:ident, $abs_tol:ident| $abs_expr:expr,
		compare_rel: |$rel_lhs:ident, $rel_rhs:ident, $rel_tol:ident| $rel_expr:expr
		$(,)?
	) => {
		$crate::impl_approx_eq_wrapper!(
			@impl_callback
			[$($impl_generics)*],
			impl: $outer,
			item: $item,
			bounds: [],
			compare_abs: |$abs_lhs, $abs_rhs, $abs_tol| $abs_expr,
			compare_rel: |$rel_lhs, $rel_rhs, $rel_tol| $rel_expr
		);
	};
}
