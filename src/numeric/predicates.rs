use crate::numeric::Zero;

/// Trait for checking if a value is zero.
pub trait IsZero: Zero {
    /// Checks if the value is zero.
    #[allow(clippy::wrong_self_convention)]
    fn is_zero(self) -> bool;
}

/// Trait for checking if a value is finite.
pub trait IsFinite {
    /// Checks if the value is finite.
    #[allow(clippy::wrong_self_convention)]
    fn is_finite(self) -> bool;
}

/// Trait for checking if a value is infinite.
pub trait IsInfinite {
    /// Checks if the value is infinite.
    #[allow(clippy::wrong_self_convention)]
    fn is_infinite(self) -> bool;
}

/// Trait for checking if a value is NaN.
pub trait IsNan {
    /// Checks if the value is NaN.
    #[allow(clippy::wrong_self_convention)]
    fn is_nan(self) -> bool;
}
