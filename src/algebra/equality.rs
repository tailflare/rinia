/// A trait for types that support approximate equality checks with absolute tolerance.
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

/// A trait for types that support approximate equality checks with relative tolerance.
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

/// Returns whether two values are approximately equal using [`ApproxEqAbs`].
/// Pass a third argument to override the default absolute tolerance.
#[macro_export]
macro_rules! approx_eq_abs {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        $crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs)
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        $crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol)
    }};
}

/// Returns whether two values are not approximately equal using [`ApproxEqAbs`].
/// Pass a third argument to override the default absolute tolerance.
#[macro_export]
macro_rules! approx_ne_abs {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        !$crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs)
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        !$crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol)
    }};
}

/// Asserts that two values are approximately equal using [`ApproxEqAbs`].
/// Pass a third argument to override the default absolute tolerance, or
/// `message = ...` to provide a custom assertion message while using the
/// default tolerance.
#[macro_export]
macro_rules! assert_approx_eq_abs {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            $crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs),
            "assertion failed: approx_eq_abs!({}, {})",
            stringify!($lhs),
            stringify!($rhs)
        );
    }};
    ($lhs:expr, $rhs:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!($crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs), $($arg)+);
    }};
    ($lhs:expr, $rhs:expr, $tol:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!($crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol), $($arg)+);
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            $crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol),
            "assertion failed: approx_eq_abs!({}, {}, {})",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($tol)
        );
    }};
}

/// Asserts that two values are not approximately equal using [`ApproxEqAbs`].
/// Pass a third argument to override the default absolute tolerance, or
/// `message = ...` to provide a custom assertion message while using the
/// default tolerance.
#[macro_export]
macro_rules! assert_approx_ne_abs {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            !$crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs),
            "assertion failed: approx_ne_abs!({}, {})",
            stringify!($lhs),
            stringify!($rhs)
        );
    }};
    ($lhs:expr, $rhs:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(!$crate::algebra::ApproxEqAbs::approx_eq_abs(lhs, rhs), $($arg)+);
    }};
    ($lhs:expr, $rhs:expr, $tol:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            !$crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol),
            $($arg)+
        );
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            !$crate::algebra::ApproxEqAbs::approx_eq_abs_tol(lhs, rhs, tol),
            "assertion failed: approx_ne_abs!({}, {}, {})",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($tol)
        );
    }};
}

/// Returns whether two values are approximately equal using [`ApproxEqRel`].
/// Pass a third argument to override the default relative tolerance.
#[macro_export]
macro_rules! approx_eq_rel {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        $crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs)
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        $crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol)
    }};
}

/// Returns whether two values are not approximately equal using [`ApproxEqRel`].
/// Pass a third argument to override the default relative tolerance.
#[macro_export]
macro_rules! approx_ne_rel {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        !$crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs)
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        !$crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol)
    }};
}

/// Asserts that two values are approximately equal using [`ApproxEqRel`].
/// Pass a third argument to override the default relative tolerance, or
/// `message = ...` to provide a custom assertion message while using the
/// default tolerance.
#[macro_export]
macro_rules! assert_approx_eq_rel {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            $crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs),
            "assertion failed: approx_eq_rel!({}, {})",
            stringify!($lhs),
            stringify!($rhs)
        );
    }};
    ($lhs:expr, $rhs:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!($crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs), $($arg)+);
    }};
    ($lhs:expr, $rhs:expr, $tol:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!($crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol), $($arg)+);
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            $crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol),
            "assertion failed: approx_eq_rel!({}, {}, {})",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($tol)
        );
    }};
}

/// Asserts that two values are not approximately equal using [`ApproxEqRel`].
/// Pass a third argument to override the default relative tolerance, or
/// `message = ...` to provide a custom assertion message while using the
/// default tolerance.
#[macro_export]
macro_rules! assert_approx_ne_rel {
    ($lhs:expr, $rhs:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(
            !$crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs),
            "assertion failed: approx_ne_rel!({}, {})",
            stringify!($lhs),
            stringify!($rhs)
        );
    }};
    ($lhs:expr, $rhs:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        assert!(!$crate::algebra::ApproxEqRel::approx_eq_rel(lhs, rhs), $($arg)+);
    }};
    ($lhs:expr, $rhs:expr, $tol:expr, message = $($arg:tt)+) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            !$crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol),
            $($arg)+
        );
    }};
    ($lhs:expr, $rhs:expr, $tol:expr $(,)?) => {{
        let lhs = $lhs;
        let rhs = $rhs;
        let tol = $tol;
        assert!(
            !$crate::algebra::ApproxEqRel::approx_eq_rel_tol(lhs, rhs, tol),
            "assertion failed: approx_ne_rel!({}, {}, {})",
            stringify!($lhs),
            stringify!($rhs),
            stringify!($tol)
        );
    }};
}
