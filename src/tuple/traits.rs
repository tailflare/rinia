use core::ops::{Index, IndexMut};

use crate::ops::HasScalar;

pub trait ScalarTuple:
    Sized + HasScalar + AsRef<[Self::Scalar]> + AsMut<[Self::Scalar]> + Index<usize> + IndexMut<usize>
{
    /// The number of scalar elements in the tuple.
    const LEN: usize;

    /// The array type corresponding to the tuple, containing the same number of scalar elements.
    type Array: AsRef<[Self::Scalar]> + AsMut<[Self::Scalar]>;

    /// Creates a new instance of the tuple type with all elements set to the given scalar value.
    fn splat(scalar: Self::Scalar) -> Self;

    /// Returns a raw pointer to the first element of the tuple.
    fn as_ptr(&self) -> *const Self::Scalar;

    /// Returns a mutable raw pointer to the first element of the tuple.
    fn as_mut_ptr(&mut self) -> *mut Self::Scalar;

    /// Returns a slice containing all elements of the tuple.
    fn as_slice(&self) -> &[Self::Scalar];

    /// Returns a mutable slice containing all elements of the tuple.
    fn as_mut_slice(&mut self) -> &mut [Self::Scalar];

    /// Converts the tuple into an array of its scalar elements.
    fn to_array(self) -> Self::Array;

    /// Creates a new instance of the tuple type from an array of scalar elements.
    fn from_array(a: Self::Array) -> Self;
}
