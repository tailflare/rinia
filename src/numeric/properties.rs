use crate::scalar::{SignedInt, UnsignedInt};

/// Provides the corresponding signed integer type.
pub trait SignedEquivalent {
    /// The corresponding signed integer type.
    type Signed: SignedInt;
}

/// Provides the corresponding unsigned integer type.
pub trait UnsignedEquivalent {
    /// The corresponding unsigned integer type.
    type Unsigned: UnsignedInt;
}
