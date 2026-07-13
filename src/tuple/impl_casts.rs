use crate::{
    algebra::{Cast, CastError, LossyCast, TryCast, TryExactCast},
    tuple::Tuple,
};

// Cast inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Returns a new tuple with all elements cast to type `U`.
    #[inline]
    pub fn cast<U>(self) -> Tuple<U, N>
    where
        T: Cast<U>,
    {
        self.map(|x| x.cast())
    }
}

// Cast trait
impl<T, U, const N: usize> Cast<Tuple<U, N>> for Tuple<T, N>
where
    T: Cast<U>,
{
    #[inline]
    fn cast(self) -> Tuple<U, N> {
        Tuple::cast(self)
    }
}

// LossyCast inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Returns a new tuple with all elements lossy cast to type `U`.
    #[inline]
    pub fn lossy_cast<U>(self) -> Tuple<U, N>
    where
        T: LossyCast<U>,
    {
        self.map(|x| x.lossy_cast())
    }
}

// LossyCast trait
impl<T, U, const N: usize> LossyCast<Tuple<U, N>> for Tuple<T, N>
where
    T: LossyCast<U>,
{
    #[inline]
    fn lossy_cast(self) -> Tuple<U, N> {
        Tuple::lossy_cast(self)
    }
}

// TryCast inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Returns a new tuple with all elements try cast to type `U`.
    #[inline]
    pub fn try_cast<U>(self) -> Result<Tuple<U, N>, CastError>
    where
        T: TryCast<U>,
    {
        self.try_map(|x| x.try_cast())
    }
}

// TryCast trait
impl<T, U, const N: usize> TryCast<Tuple<U, N>> for Tuple<T, N>
where
    T: TryCast<U>,
{
    #[inline]
    fn try_cast(self) -> Result<Tuple<U, N>, CastError> {
        Tuple::try_cast(self)
    }
}

// TryExactCast inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Returns a new tuple with all elements try exact cast to type `U`.
    #[inline]
    pub fn try_exact_cast<U>(self) -> Result<Tuple<U, N>, CastError>
    where
        T: TryExactCast<U>,
    {
        self.try_map(|x| x.try_exact_cast())
    }
}

// TryExactCast trait
impl<T, U, const N: usize> TryExactCast<Tuple<U, N>> for Tuple<T, N>
where
    T: TryExactCast<U>,
{
    #[inline]
    fn try_exact_cast(self) -> Result<Tuple<U, N>, CastError> {
        Tuple::try_exact_cast(self)
    }
}
