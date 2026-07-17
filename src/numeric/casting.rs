/// A trait which represents an infallible cast from one type to another.
///
/// Implementations must guarantee that the conversion is always valid and
/// does not lose information.
pub trait Cast<T> {
    /// Casts `self` to type `T`.
    fn cast(self) -> T;
}

/// A trait which represents an infallible cast into `Self` from another type.
///
/// See [Cast] for more information.
pub trait CastFrom<T> {
    /// Casts `value` to type `Self`.
    fn cast_from(value: T) -> Self;
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

/// A trait which represents a potentially lossy cast into `Self` from another type.
///
/// See [LossyCast] for more information.
pub trait LossyCastFrom<T> {
    /// Casts `value` to type `Self`, potentially losing information.
    fn lossy_cast_from(value: T) -> Self;
}

/// A trait which represents a saturating cast from one type to another.
///
/// A saturating cast always performs the conversion. If the value is outside
/// the range representable by the target type, it is clamped to the nearest
/// representable value.
///
/// Non-finite floating-point values are handled as out-of-range values:
/// NaN converts to zero, positive infinity converts to the target maximum,
/// and negative infinity converts to the target minimum.
pub trait SaturatingCast<T> {
    /// Casts `self` to type `T`, saturating the value if it is out of range.
    fn saturating_cast(self) -> T;
}

/// A trait which represents a saturating cast into `Self` from another type.
///
/// See [SaturatingCast] for more information.
pub trait SaturatingCastFrom<T> {
    /// Casts `value` to type `Self`, saturating the value if it is out of range.
    fn saturating_cast_from(value: T) -> Self;
}

/// A trait which represents a cast from one type to the corresponding signed type.
pub trait SignedCast<T> {
    /// Casts `self` to type `T`, which is the signed equivalent of the original type.
    fn signed_cast(self) -> T;
}

/// A trait which represents a cast into `Self` from another type that is the corresponding signed type.
pub trait SignedCastFrom<T> {
    /// Casts `value` to type `Self`, which is the signed equivalent of the original type.
    fn signed_cast_from(value: T) -> Self;
}

/// A trait which represents a cast from one type to the corresponding unsigned type.
pub trait UnsignedCast<T> {
    /// Casts `self` to type `T`, which is the unsigned equivalent of the original type.
    fn unsigned_cast(self) -> T;
}

pub trait UnsignedCastFrom<T> {
    /// Casts `value` to type `Self`, which is the unsigned equivalent of the original type.
    fn unsigned_cast_from(value: T) -> Self;
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

/// A trait which represents a fallible cast into `Self` from another type that preserves the value exactly.
///
/// See [TryExactCast] for more information.
pub trait TryExactCastFrom<T>
where
    Self: Sized,
{
    /// Attempts to cast `value` to type `Self` without loss of information.
    fn try_exact_cast_from(value: T) -> Result<Self, CastError>;
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

/// A trait which represents a fallible cast into `Self` from another type.
///
/// See [TryCast] for more information.
pub trait TryCastFrom<T>
where
    Self: Sized,
{
    /// Attempts to cast `value` to type `Self`.
    fn try_cast_from(value: T) -> Result<Self, CastError>;
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
