pub trait ApproxEqAbs<Rhs = Self> {
    /// The type of the tolerance used for absolute approximate equality checks.
    type Tolerance;

    /// The default absolute tolerance for approximate equality checks.
    const DEFAULT_TOLERANCE_ABS: Self::Tolerance;

    /// Checks if the value is approximately equal to another value within a given absolute tolerance.
    fn approx_eq_abs_tol(self, other: Rhs, tol: Self::Tolerance) -> bool;

    /// Checks if the value is approximately equal to another value within the default absolute tolerance.
    fn approx_eq_abs(self, other: Rhs) -> bool
    where
        Self: Sized,
    {
        self.approx_eq_abs_tol(other, Self::DEFAULT_TOLERANCE_ABS)
    }
}

/// Returns whether two values are approximately equal using [`ApproxEqAbs`] and the
/// type's default absolute tolerance.
#[macro_export]
macro_rules! approx_eql_abs {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        $crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs)
    }};
}

/// Asserts that two values are approximately equal using [`ApproxEqAbs`] and the
/// type's default absolute tolerance.
#[macro_export]
macro_rules! assert_approx_eql_abs {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            $crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs),
            "assertion failed: approx_eql_abs!({}, {})",
            stringify!($lhs),
            stringify!($rhs)
        );
    }};
    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!($crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs), $($arg)+);
    }};
}

/// Returns whether two values are approximately equal using [`ApproxEqAbs`] and
/// an explicit absolute tolerance.
#[macro_export]
macro_rules! approx_eql_abs_tol {
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        $crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol)
    }};
}

/// Asserts that two values are approximately equal using [`ApproxEqAbs`] and
/// an explicit absolute tolerance.
#[macro_export]
macro_rules! assert_approx_eql_abs_tol {
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            $crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol),
            "assertion failed: approx_eql_abs_tol!({}, {}, {})",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($tol)
        );
    }};
    ($lhs:expr, $rhs:expr, $tol:expr, $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!($crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol), $($arg)+);
    }};
}

pub trait ApproxEqRel<Rhs = Self> {
    /// The type of the tolerance used for relative approximate equality checks.
    type Tolerance;

    /// The default relative tolerance for approximate equality checks.
    const DEFAULT_TOLERANCE_REL: Self::Tolerance;

    /// Checks if the value is approximately equal to another value within a given relative tolerance.
    fn approx_eq_rel_tol(self, other: Rhs, tol: Self::Tolerance) -> bool;

    /// Checks if the value is approximately equal to another value within the default relative tolerance.
    fn approx_eq_rel(self, other: Rhs) -> bool
    where
        Self: Sized,
    {
        self.approx_eq_rel_tol(other, Self::DEFAULT_TOLERANCE_REL)
    }
}

/// Returns whether two values are approximately equal using [`ApproxEqRel`] and the
/// type's default relative tolerance.
#[macro_export]
macro_rules! approx_eql_rel {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        $crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs)
    }};
}

/// Asserts that two values are approximately equal using [`ApproxEqRel`] and the
/// type's default relative tolerance.
#[macro_export]
macro_rules! assert_approx_eql_rel {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            $crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs),
            "assertion failed: approx_eql_rel!({}, {})",
            stringify!($lhs),
            stringify!($rhs)
        );
    }};
    ($lhs:expr, $rhs:expr, $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!($crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs), $($arg)+);
    }};
}

/// Returns whether two values are approximately equal using [`ApproxEqRel`] and
/// an explicit relative tolerance.
#[macro_export]
macro_rules! approx_eql_rel_tol {
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        $crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol)
    }};
}

/// Asserts that two values are approximately equal using [`ApproxEqRel`] and
/// an explicit relative tolerance.
#[macro_export]
macro_rules! assert_approx_eql_rel_tol {
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            $crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol),
            "assertion failed: approx_eql_rel_tol!({}, {}, {})",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($tol)
        );
    }};
    ($lhs:expr, $rhs:expr, $tol:expr, $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!($crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol), $($arg)+);
    }};
}
