#[cfg(feature = "zerocopy")]
use zerocopy::*;

/// A tuple type that can hold a fixed number of elements of the same type.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(transparent)]
pub struct Tuple<T, const N: usize> {
    pub(crate) inner: [T; N],
}
