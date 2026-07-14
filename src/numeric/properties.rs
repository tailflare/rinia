use crate::scalar::{SignedInt, UnsignedInt};

/// Provides the corresponding signed integer type.
pub trait SignedEquivalent {
    type Signed: SignedInt;
}

/// Provides the corresponding unsigned integer type.
pub trait UnsignedEquivalent {
    type Unsigned: UnsignedInt;
}
