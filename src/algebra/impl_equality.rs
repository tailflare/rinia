use crate::algebra::{ApproxEqAbs, ApproxEqRel};

// Blanket ApproxEqAbs for Arrays
impl<T, U, const N: usize> ApproxEqAbs<[U; N]> for [T; N]
where
    T: ApproxEqAbs<U>,
{
    type Tolerance = T::Tolerance;

    const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

    #[inline]
    fn approx_eq_abs_tol(&self, other: &[U; N], tol: Self::Tolerance) -> bool {
        self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs.approx_eq_abs_tol(rhs, tol))
    }
}

// Blanket ApproxEqRel for Arrays
impl<T, U, const N: usize> ApproxEqRel<[U; N]> for [T; N]
where
    T: ApproxEqRel<U>,
{
    type Tolerance = T::Tolerance;

    const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

    #[inline]
    fn approx_eq_rel_tol(&self, other: &[U; N], tol: Self::Tolerance) -> bool {
        self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs.approx_eq_rel_tol(rhs, tol))
    }
}

// Blanket ApproxEqAbs for Slices
impl<T, U> ApproxEqAbs<[U]> for [T]
where
    T: ApproxEqAbs<U>,
{
    type Tolerance = T::Tolerance;

    const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

    #[inline]
    fn approx_eq_abs_tol(&self, other: &[U], tol: Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs.approx_eq_abs_tol(rhs, tol))
    }
}

// Blanket ApproxEqRel for Slices
impl<T, U> ApproxEqRel<[U]> for [T]
where
    T: ApproxEqRel<U>,
{
    type Tolerance = T::Tolerance;

    const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

    #[inline]
    fn approx_eq_rel_tol(&self, other: &[U], tol: Self::Tolerance) -> bool {
        self.len() == other.len()
            && self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs.approx_eq_rel_tol(rhs, tol))
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
    {
        type Tolerance = T::Tolerance;

        const DEFAULT_TOLERANCE_ABS: Self::Tolerance = T::DEFAULT_TOLERANCE_ABS;

        #[inline]
        fn approx_eq_abs_tol(&self, other: &Vec<U>, tol: Self::Tolerance) -> bool {
            self.len() == other.len()
                && self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs.approx_eq_abs_tol(rhs, tol))
        }
    }

    // Blanket ApproxEqRel for Vec
    impl<T, U> ApproxEqRel<Vec<U>> for Vec<T>
    where
        T: ApproxEqRel<U>,
    {
        type Tolerance = T::Tolerance;

        const DEFAULT_TOLERANCE_REL: Self::Tolerance = T::DEFAULT_TOLERANCE_REL;

        #[inline]
        fn approx_eq_rel_tol(&self, other: &Vec<U>, tol: Self::Tolerance) -> bool {
            self.len() == other.len()
                && self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs.approx_eq_rel_tol(rhs, tol))
        }
    }
}
