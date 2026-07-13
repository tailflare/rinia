/// A trait which represents an infallible cast from one type to another.
///
/// Implementations must guarantee that the conversion is always valid and
/// does not lose information.
pub trait Cast<T> {
    /// Casts `self` to type `T`.
    fn cast(self) -> T;
}

/// A trait which represents a potentially lossy cast from one type to another.
///
/// A lossy cast always performs the conversion, even when information may be
/// discarded. This may include reduced precision, truncation, clamping, or
/// other changes to the original value.
pub trait LossyCast<T> {
    /// Casts `self` to type `T`, potentially losing information.
    fn lossy_cast(self) -> T;
}

/// A trait which represents a fallible cast that preserves the value exactly.
///
/// The conversion succeeds only when the value can be represented exactly
/// by the target type. Conversions which would lose precision or otherwise
/// alter the value return an error.
pub trait TryExactCast<T> {
    /// Attempts to cast `self` to type `T` without loss of information.
    fn try_exact_cast(self) -> Result<T, CastError>;
}

/// A trait which represents a fallible cast between types.
///
/// The conversion succeeds when the value can be represented by the target
/// type according to the target type's conversion rules. Conversions which
/// cannot be performed safely return an error.
pub trait TryCast<T> {
    /// Attempts to cast `self` to type `T`.
    fn try_cast(self) -> Result<T, CastError>;
}

/// An error returned by fallible cast operations.
#[derive(Debug, PartialEq)]
pub enum CastError {
    /// The value cannot be represented within the range of the target type.
    OutOfRange,

    /// The value contains a fractional component that cannot be represented
    /// by the target type.
    Fractional,

    /// The value is not a finite number, such as NaN or infinity.
    NonFinite,

    /// The conversion would lose precision or otherwise fail to preserve the
    /// original value exactly.
    Inexact,
}
