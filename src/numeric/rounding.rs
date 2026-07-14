pub trait Rounding: Floor + Ceil + Trunc + Round + Fract {}

/// Trait for rounding down to the nearest integer.
pub trait Floor {
    /// Rounds the value down to the nearest integer.
    fn floor(self) -> Self;
}

/// Trait for rounding up to the nearest integer.
pub trait Ceil {
    /// Rounds the value up to the nearest integer.
    fn ceil(self) -> Self;
}

/// Trait for rounding towards zero to the nearest integer.
pub trait Trunc {
    /// Rounds the value towards zero to the nearest integer.
    fn trunc(self) -> Self;
}

/// Trait for rounding to the nearest integer, with ties rounding away from zero.
pub trait Round {
    /// Rounds the value to the nearest integer, with ties rounding away from zero.
    fn round(self) -> Self;
}

/// Trait for extracting the fractional part of a number.
pub trait Fract {
    /// Returns the fractional part of the number.
    fn fract(self) -> Self;
}
