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

// Blanket impls for the From variants

impl<T, U> CastFrom<T> for U
where
    T: Cast<U>,
{
    #[inline]
    fn cast_from(value: T) -> Self {
        value.cast()
    }
}

impl<T, U> LossyCastFrom<T> for U
where
    T: LossyCast<U>,
{
    #[inline]
    fn lossy_cast_from(value: T) -> Self {
        value.lossy_cast()
    }
}

impl<T, U> TryCastFrom<T> for U
where
    T: TryCast<U>,
{
    #[inline]
    fn try_cast_from(value: T) -> Result<Self, CastError> {
        value.try_cast()
    }
}

impl<T, U> TryExactCastFrom<T> for U
where
    T: TryExactCast<U>,
{
    #[inline]
    fn try_exact_cast_from(value: T) -> Result<Self, CastError> {
        value.try_exact_cast()
    }
}
