use crate::{
    numeric::{Cast, CastError, LossyCast, SaturatingCast, TryCast, TryExactCast},
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

// CastFrom inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Casts `value` into `Self`.
    #[inline]
    pub fn cast_from<U>(value: Tuple<U, N>) -> Self
    where
        U: Cast<T>,
    {
        value.cast::<T>()
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

// LossyCastFrom inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Lossy casts `value` into `Self`.
    #[inline]
    pub fn lossy_cast_from<U>(value: Tuple<U, N>) -> Self
    where
        U: LossyCast<T>,
    {
        value.lossy_cast::<T>()
    }
}

// SaturatingCast inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Returns a new tuple with all elements saturating cast to type `U`.
    #[inline]
    pub fn saturating_cast<U>(self) -> Tuple<U, N>
    where
        T: SaturatingCast<U>,
    {
        self.map(|x| x.saturating_cast())
    }
}

// SaturatingCast trait
impl<T, U, const N: usize> SaturatingCast<Tuple<U, N>> for Tuple<T, N>
where
    T: SaturatingCast<U>,
{
    #[inline]
    fn saturating_cast(self) -> Tuple<U, N> {
        Tuple::saturating_cast(self)
    }
}

// SaturatingCastFrom inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Saturating casts `value` into `Self`.
    #[inline]
    pub fn saturating_cast_from<U>(value: Tuple<U, N>) -> Self
    where
        U: SaturatingCast<T>,
    {
        value.saturating_cast::<T>()
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

// TryCastFrom inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Try casts `value` into `Self`.
    #[inline]
    pub fn try_cast_from<U>(value: Tuple<U, N>) -> Result<Self, CastError>
    where
        U: TryCast<T>,
    {
        value.try_cast::<T>()
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

// TryExactCastFrom inherent
impl<T, const N: usize> Tuple<T, N> {
    /// Try exact casts `value` into `Self`.
    #[inline]
    pub fn try_exact_cast_from<U>(value: Tuple<U, N>) -> Result<Self, CastError>
    where
        U: TryExactCast<T>,
    {
        value.try_exact_cast::<T>()
    }
}
