use core::{mem::MaybeUninit, ops::Add};

use crate::{
    common,
    numeric::{One, Zero},
    scalar::{HasScalar, Scalar},
    tuple::Tuple,
};

// Inherent impl for Tuple<T, N>
impl<T, const N: usize> Tuple<T, N> {
    /// Creates a new [Tuple] from an array.
    #[inline]
    pub const fn from_array(inner: [T; N]) -> Self {
        Self { inner }
    }

    /// Creates a new [Tuple] with all elements set to the given value.
    #[inline]
    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self::from_array([value; N])
    }

    /// Creates a new [Tuple] with elements set to the values 0, 1, 2, ..., N-1.
    #[inline]
    pub fn iota() -> Self
    where
        T: Zero + One + Add<Output = T> + Copy,
    {
        let mut out = [T::ZERO; N];
        let mut value = T::ZERO;

        for item in &mut out {
            *item = value;
            value = value + T::ONE;
        }

        Self::from_array(out)
    }

    /// Returns a reference to the inner array.
    #[inline]
    pub const fn as_array(&self) -> &[T; N] {
        &self.inner
    }

    /// Consumes the [Tuple] and returns the inner array.
    #[inline]
    pub fn into_array(self) -> [T; N] {
        self.inner
    }

    /// Returns a slice to the inner array.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    /// Returns a mutable slice to the inner array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner
    }

    /// Returns an iterator over the elements of the [Tuple].
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.inner.iter()
    }

    /// Returns a mutable iterator over the elements of the [Tuple].
    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.inner.iter_mut()
    }

    /// Maps each element of the [Tuple] to a new [Tuple] using the provided function.
    #[inline]
    pub fn map<U, F>(self, f: F) -> Tuple<U, N>
    where
        F: FnMut(T) -> U,
    {
        Tuple::from_array(self.inner.map(f))
    }

    /// Maps each element of the [Tuple] to a new [Tuple] using the provided function.
    ///
    /// Returns an error if the function fails for any element.
    #[inline]
    pub fn try_map<U, E, F>(self, mut f: F) -> Result<Tuple<U, N>, E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let mut output: [MaybeUninit<U>; N] = [const { MaybeUninit::uninit() }; N];

        for (index, value) in self.inner.into_iter().enumerate() {
            output[index].write(f(value)?);
        }

        // SAFETY: Every element was initialized before this point.
        let output = unsafe { core::mem::transmute_copy::<[MaybeUninit<U>; N], [U; N]>(&output) };

        Ok(Tuple::from_array(output))
    }

    /// Maps each element of the [Tuple] to a new [Tuple] using the provided function,
    /// which takes a reference to each element.
    #[inline]
    pub fn map_ref<U, F>(&self, f: F) -> Tuple<U, N>
    where
        F: FnMut(&T) -> U,
    {
        Tuple::from_array(self.inner.each_ref().map(f))
    }

    /// Zips two [Tuple]s together using the provided function, producing a new [Tuple].
    #[inline]
    pub fn zip_with<U, V, F>(&self, rhs: &Tuple<U, N>, mut f: F) -> Tuple<V, N>
    where
        F: FnMut(&T, &U) -> V,
    {
        Tuple::from_array(core::array::from_fn(|i| f(&self.inner[i], &rhs.inner[i])))
    }

    /// Zips two [Tuple]s together, producing a new [Tuple] of tuples containing references to
    /// the elements.
    #[inline]
    pub fn zip<'a, U>(&'a self, rhs: &'a Tuple<U, N>) -> Tuple<(&'a T, &'a U), N> {
        Tuple::from_array(core::array::from_fn(|i| (&self.inner[i], &rhs.inner[i])))
    }
}

// TupleLike trait implementation for Tuple<T, N>
common::impl_tuplelike_for_wrapper!([T, const N: usize], Tuple<T, N>, item: T, len: N);

// Bytemuck trait implementations for Tuple<T, N>
common::impl_bytemuck_basic!([T, const N: usize], Tuple<T, N>, item: T);

// Blanket impl for HasScalar for Tuple<T, N> where T implements Scalar
impl<T, const N: usize> HasScalar for Tuple<T, N>
where
    T: Scalar,
{
    type Scalar = T;
}
