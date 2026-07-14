use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Index, IndexMut},
};

use crate::tuple::Tuple;

// Implement Default for Tuple<T, N> where T: Default
impl<T: Default, const N: usize> Default for Tuple<T, N> {
    #[inline]
    fn default() -> Self {
        Self { inner: core::array::from_fn(|_| T::default()) }
    }
}

// Index trait
impl<T, const N: usize> Index<usize> for Tuple<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

// IndexMut trait
impl<T, const N: usize> IndexMut<usize> for Tuple<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

// AsRef array trait
impl<T, const N: usize> AsRef<[T; N]> for Tuple<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        &self.inner
    }
}

// AsMut array trait
impl<T, const N: usize> AsMut<[T; N]> for Tuple<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.inner
    }
}

// AsRef slice trait
impl<T, const N: usize> AsRef<[T]> for Tuple<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.inner
    }
}

// AsMut slice trait
impl<T, const N: usize> AsMut<[T]> for Tuple<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.inner
    }
}

// Borrow slice trait
impl<T, const N: usize> Borrow<[T]> for Tuple<T, N> {
    #[inline]
    fn borrow(&self) -> &[T] {
        &self.inner
    }
}

// BorrowMut slice trait
impl<T, const N: usize> BorrowMut<[T]> for Tuple<T, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self.inner
    }
}

// IntoIterator trait for ref
impl<'a, T, const N: usize> IntoIterator for &'a Tuple<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

// IntoIterator trait for mutable ref
impl<'a, T, const N: usize> IntoIterator for &'a mut Tuple<T, N> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

// IntoIterator trait for owned Tuple
impl<T, const N: usize> IntoIterator for Tuple<T, N> {
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

// From array to tuple trait
impl<T, const N: usize> From<[T; N]> for Tuple<T, N> {
    #[inline]
    fn from(inner: [T; N]) -> Self {
        Self { inner }
    }
}

// From tuple to array trait
impl<T, const N: usize> From<Tuple<T, N>> for [T; N] {
    #[inline]
    fn from(tuple: Tuple<T, N>) -> Self {
        tuple.inner
    }
}
