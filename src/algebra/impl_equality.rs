use crate::algebra::{ApproxEqAbs, ApproxEqRel};

// Blanket ApproxEqAbs for Arrays
impl<T, U, const N: usize> ApproxEqAbs<[U; N]> for [T; N]
where
    T: ApproxEqAbs<U>,
    T::Tolerance: Clone,
{
    type Tolerance = T::Tolerance;
    const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

    #[inline]
    fn approx_eq_abs_tol(self, other: [U; N], tol: Self::Tolerance) -> bool {
        self.into_iter().zip(other).all(|(lhs, rhs)| lhs.approx_eq_abs_tol(rhs, tol.clone()))
    }
}

// Blanket ApproxEqRel for Arrays
impl<T, U, const N: usize> ApproxEqRel<[U; N]> for [T; N]
where
    T: ApproxEqRel<U>,
    T::Tolerance: Clone,
{
    type Tolerance = T::Tolerance;
    const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

    #[inline]
    fn approx_eq_rel_tol(self, other: [U; N], tol: Self::Tolerance) -> bool {
        self.into_iter().zip(other).all(|(lhs, rhs)| lhs.approx_eq_rel_tol(rhs, tol.clone()))
    }
}

// Blanket ApproxEqAbs for Slices
impl<'b, T, U> ApproxEqAbs<&'b [U]> for &[T]
where
    T: ApproxEqAbs<U> + Clone,
    U: Clone,
    T::Tolerance: Clone,
{
    type Tolerance = T::Tolerance;
    const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

    #[inline]
    fn approx_eq_abs_tol(self, other: &'b [U], tol: Self::Tolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(lhs, rhs)| lhs.clone().approx_eq_abs_tol(rhs.clone(), tol.clone()))
    }
}

// Blanket ApproxEqRel for Slices
impl<'b, T, U> ApproxEqRel<&'b [U]> for &[T]
where
    T: ApproxEqRel<U> + Clone,
    U: Clone,
    T::Tolerance: Clone,
{
    type Tolerance = T::Tolerance;
    const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

    #[inline]
    fn approx_eq_rel_tol(self, other: &'b [U], tol: Self::Tolerance) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other.iter())
                .all(|(lhs, rhs)| lhs.clone().approx_eq_rel_tol(rhs.clone(), tol.clone()))
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
mod vec_equality {
    use alloc::vec::Vec;

    use crate::algebra::{ApproxEqAbs, ApproxEqRel};

    // Blanket ApproxEqAbs for Vec
    impl<T, U> ApproxEqAbs<Vec<U>> for Vec<T>
    where
        T: ApproxEqAbs<U>,
        T::Tolerance: Clone,
    {
        type Tolerance = T::Tolerance;
        const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

        #[inline]
        fn approx_eq_abs_tol(self, other: Vec<U>, tol: Self::Tolerance) -> bool {
            self.len() == other.len()
                && self
                    .into_iter()
                    .zip(other)
                    .all(|(lhs, rhs)| lhs.approx_eq_abs_tol(rhs, tol.clone()))
        }
    }

    // Blanket ApproxEqRel for Vec
    impl<T, U> ApproxEqRel<Vec<U>> for Vec<T>
    where
        T: ApproxEqRel<U>,
        T::Tolerance: Clone,
    {
        type Tolerance = T::Tolerance;
        const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

        #[inline]
        fn approx_eq_rel_tol(self, other: Vec<U>, tol: Self::Tolerance) -> bool {
            self.len() == other.len()
                && self
                    .into_iter()
                    .zip(other)
                    .all(|(lhs, rhs)| lhs.approx_eq_rel_tol(rhs, tol.clone()))
        }
    }
}
