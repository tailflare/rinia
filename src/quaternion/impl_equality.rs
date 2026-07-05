use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{Quat, QuatScalar, ops::Negate};

#[inline]
fn canonicalize_other_sign<T>(lhs: &Quat<T>, rhs: &Quat<T>) -> Quat<T>
where
    T: QuatScalar,
{
    let dot = lhs.dot(*rhs);
    if dot < T::ZERO { rhs.negate() } else { *rhs }
}

#[inline]
fn components_eq<T, F>(lhs: &Quat<T>, rhs: &Quat<T>, mut cmp: F) -> bool
where
    T: QuatScalar,
    F: FnMut(&T, &T) -> bool,
{
    let rhs = canonicalize_other_sign(lhs, rhs);
    cmp(&lhs.x, &rhs.x) && cmp(&lhs.y, &rhs.y) && cmp(&lhs.z, &rhs.z) && cmp(&lhs.w, &rhs.w)
}

impl<T> AbsDiffEq for Quat<T>
where
    T: QuatScalar + AbsDiffEq<Epsilon = T>,
{
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        components_eq(self, other, |a, b| T::abs_diff_eq(a, b, epsilon))
    }
}

impl<T> RelativeEq for Quat<T>
where
    T: QuatScalar + RelativeEq<Epsilon = T>,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        components_eq(self, other, |a, b| T::relative_eq(a, b, epsilon, max_relative))
    }
}

impl<T> UlpsEq for Quat<T>
where
    T: QuatScalar + UlpsEq<Epsilon = T>,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        components_eq(self, other, |a, b| T::ulps_eq(a, b, epsilon, max_ulps))
    }
}
