#[cfg(feature = "zerocopy")]
use zerocopy::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sakka", derive(sakka::Encode, sakka::Decode))]
#[cfg_attr(feature = "zerocopy", derive(FromBytes, Immutable, IntoBytes, KnownLayout))]
#[repr(transparent)]
pub struct Tuple<T, const N: usize> {
    pub(crate) inner: [T; N],
}
