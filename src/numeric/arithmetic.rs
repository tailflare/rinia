/// A trait for saturating addition.
pub trait SaturatingAdd<Rhs = Self> {
    /// The output type for the saturating addition.
    type Output;

    /// Performs saturating addition of `self` and `rhs`.
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

/// A trait for saturating subtraction.
pub trait SaturatingSub<Rhs = Self> {
    /// The output type for the saturating subtraction.
    type Output;

    /// Performs saturating subtraction of `self` and `rhs`.
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

/// A trait for saturating multiplication.
pub trait SaturatingMul<Rhs = Self> {
    /// The output type for the saturating multiplication.
    type Output;

    /// Performs saturating multiplication of `self` and `rhs`.
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

/// A trait for saturating division.
pub trait SaturatingDiv<Rhs = Self> {
    /// The output type for the saturating division.
    type Output;

    /// Performs saturating division of `self` and `rhs`.
    fn saturating_div(self, rhs: Rhs) -> Self::Output;
}

/// A trait for saturating negation.
pub trait SaturatingNeg {
    /// The output type for the saturating negation.
    type Output;

    /// Performs saturating negation of `self`.
    fn saturating_neg(self) -> Self::Output;
}

/// A trait for wrapping addition.
pub trait WrappingAdd<Rhs = Self> {
    /// The output type for the wrapping addition.
    type Output;

    /// Performs wrapping addition of `self` and `rhs`.
    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

/// A trait for wrapping subtraction.
pub trait WrappingSub<Rhs = Self> {
    /// The output type for the wrapping subtraction.
    type Output;

    /// Performs wrapping subtraction of `self` and `rhs`.
    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

/// A trait for wrapping multiplication.
pub trait WrappingMul<Rhs = Self> {
    /// The output type for the wrapping multiplication.
    type Output;

    /// Performs wrapping multiplication of `self` and `rhs`.
    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

/// A trait for wrapping division.
pub trait WrappingDiv<Rhs = Self> {
    /// The output type for the wrapping division.
    type Output;

    /// Performs wrapping division of `self` and `rhs`.
    fn wrapping_div(self, rhs: Rhs) -> Self::Output;
}

/// A trait for wrapping remainder.
pub trait WrappingRem<Rhs = Self> {
    /// The output type for the wrapping remainder.
    type Output;

    /// Performs wrapping remainder of `self` and `rhs`.
    fn wrapping_rem(self, rhs: Rhs) -> Self::Output;
}

/// A trait for wrapping negation.
pub trait WrappingNeg {
    /// The output type for the wrapping negation.
    type Output;

    /// Performs wrapping negation of `self`.
    fn wrapping_neg(self) -> Self::Output;
}

/// A trait for checked addition.
pub trait CheckedAdd<Rhs = Self> {
    /// The output type for the checked addition.
    type Output;

    /// Performs checked addition of `self` and `rhs`.
    fn checked_add(self, rhs: Rhs) -> Self::Output;
}

/// A trait for checked subtraction.
pub trait CheckedSub<Rhs = Self> {
    /// The output type for the checked subtraction.
    type Output;

    /// Performs checked subtraction of `self` and `rhs`.
    fn checked_sub(self, rhs: Rhs) -> Self::Output;
}

/// A trait for checked multiplication.
pub trait CheckedMul<Rhs = Self> {
    /// The output type for the checked multiplication.
    type Output;

    /// Performs checked multiplication of `self` and `rhs`.
    fn checked_mul(self, rhs: Rhs) -> Self::Output;
}

/// A trait for checked division.
pub trait CheckedDiv<Rhs = Self> {
    /// The output type for the checked division.
    type Output;

    /// Performs checked division of `self` and `rhs`.
    fn checked_div(self, rhs: Rhs) -> Self::Output;
}

/// A trait for checked remainder.
pub trait CheckedRem<Rhs = Self> {
    /// The output type for the checked remainder.
    type Output;

    /// Performs checked remainder of `self` and `rhs`.
    fn checked_rem(self, rhs: Rhs) -> Self::Output;
}

/// A trait for checked negation.
pub trait CheckedNeg {
    /// The output type for the checked negation.
    type Output;

    /// Performs checked negation of `self`.
    fn checked_neg(self) -> Self::Output;
}
