/// Trait for checking if a value is finite.
pub trait Finite {
    /// Checks if the value is finite.
    #[allow(clippy::wrong_self_convention)]
    fn is_finite(self) -> bool;
}

/// Trait for checking if a value is infinite and for providing positive and negative infinity values.
pub trait Infinite {
    /// The positive infinity value.
    const INFINITY: Self;

    /// The negative infinity value.
    const NEG_INFINITY: Self;

    /// Checks if the value is infinite.
    #[allow(clippy::wrong_self_convention)]
    fn is_infinite(self) -> bool;
}

/// Trait for checking if a value is NaN and for providing a NaN value.
pub trait Nan {
    /// The NaN value.
    const NAN: Self;

    /// Checks if the value is NaN.
    #[allow(clippy::wrong_self_convention)]
    fn is_nan(self) -> bool;
}
