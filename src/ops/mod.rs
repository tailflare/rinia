/// A trait representing the additive identity for a type.
pub trait Zero {
    /// The additive identity for the type (typically 0).
    const ZERO: Self;

    /// Checks if the value is equal to the additive identity (zero).
    #[allow(clippy::wrong_self_convention)]
    fn is_zero(self) -> bool;
}

/// A trait representing the multiplicative identity for a type.
pub trait One {
    /// The multiplicative identity for the type (typically 1).
    const ONE: Self;
}

/// A trait for selecting values based on comparisons.
pub trait Select {
    /// Returns the minimum of two values.
    fn min_val(self, other: Self) -> Self;

    /// Returns the maximum of two values.
    fn max_val(self, other: Self) -> Self;
}

/// A trait representing a type that has an associated scalar type.
pub trait HasScalar {
    /// The associated scalar type for the implementing type.
    type Scalar;
}
