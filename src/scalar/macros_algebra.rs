macro_rules! _impl_scalar_approx_eq {
	($([$ty:ty, abs: $default_abs_tol:expr, rel: $default_rel_tol:expr]),+ $(,)?) => {
		$(
            // ApproxEqAbs trait
			impl $crate::algebra::ApproxEqAbs for $ty {
				type Tolerance = $ty;
				const DEFAULT_TOLERANCE_ABS: Self::Tolerance = $default_abs_tol;

				#[inline]
				fn approx_eq_abs_tol(self, other: Self, tol: Self::Tolerance) -> bool {
					(self - other).abs() <= tol
				}
			}

            // ApproxEqRel trait
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

macro_rules! _impl_scalar_lerp {
	($($ty:ty),+ $(,)?) => {
		$(
            // Lerp trait
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

pub(crate) use _impl_scalar_approx_eq;
pub(crate) use _impl_scalar_lerp;
