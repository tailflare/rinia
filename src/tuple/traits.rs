/// A trait for types that behave like tuples, meaning they have a fixed number of elements of the
/// same type.
pub trait TupleLike {
    /// The type of the elements in the tuple.
    type Item;

    /// The number of elements in the tuple.
    const LEN: usize;

    /// Returns a slice of the elements in the tuple.
    fn as_slice(&self) -> &[Self::Item];

    /// Returns a mutable slice of the elements in the tuple.
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}
