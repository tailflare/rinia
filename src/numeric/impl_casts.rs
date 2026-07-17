use crate::numeric::{
    Cast, CastError, CastFrom, LossyCast, LossyCastFrom, SaturatingCast, SaturatingCastFrom,
    SignedCast, SignedCastFrom, TryCast, TryCastFrom, TryExactCast, TryExactCastFrom, UnsignedCast,
    UnsignedCastFrom,
};

// Blanket impls for the From variants
impl<T, U> CastFrom<T> for U
where
    T: Cast<U>,
{
    #[inline]
    fn cast_from(value: T) -> Self {
        value.cast()
    }
}

impl<T, U> LossyCastFrom<T> for U
where
    T: LossyCast<U>,
{
    #[inline]
    fn lossy_cast_from(value: T) -> Self {
        value.lossy_cast()
    }
}

impl<T, U> SaturatingCastFrom<T> for U
where
    T: SaturatingCast<U>,
{
    #[inline]
    fn saturating_cast_from(value: T) -> Self {
        value.saturating_cast()
    }
}

impl<T, U> SignedCastFrom<T> for U
where
    T: SignedCast<U>,
{
    #[inline]
    fn signed_cast_from(value: T) -> Self {
        value.signed_cast()
    }
}

impl<T, U> UnsignedCastFrom<T> for U
where
    T: UnsignedCast<U>,
{
    #[inline]
    fn unsigned_cast_from(value: T) -> Self {
        value.unsigned_cast()
    }
}

impl<T, U> TryCastFrom<T> for U
where
    T: TryCast<U>,
{
    #[inline]
    fn try_cast_from(value: T) -> Result<Self, CastError> {
        value.try_cast()
    }
}

impl<T, U> TryExactCastFrom<T> for U
where
    T: TryExactCast<U>,
{
    #[inline]
    fn try_exact_cast_from(value: T) -> Result<Self, CastError> {
        value.try_exact_cast()
    }
}

// Blanket impls for arrays
impl<T, U, const N: usize> Cast<[U; N]> for [T; N]
where
    T: Cast<U>,
{
    #[inline]
    fn cast(self) -> [U; N] {
        self.map(Cast::cast)
    }
}

impl<T, U, const N: usize> LossyCast<[U; N]> for [T; N]
where
    T: LossyCast<U>,
{
    #[inline]
    fn lossy_cast(self) -> [U; N] {
        self.map(LossyCast::lossy_cast)
    }
}

impl<T, U, const N: usize> SaturatingCast<[U; N]> for [T; N]
where
    T: SaturatingCast<U>,
{
    #[inline]
    fn saturating_cast(self) -> [U; N] {
        self.map(SaturatingCast::saturating_cast)
    }
}

impl<T, U, const N: usize> TryCast<[U; N]> for [T; N]
where
    T: TryCast<U>,
{
    #[inline]
    fn try_cast(self) -> Result<[U; N], CastError> {
        let mut output: [core::mem::MaybeUninit<U>; N] =
            [const { core::mem::MaybeUninit::uninit() }; N];

        for (index, value) in self.into_iter().enumerate() {
            output[index].write(value.try_cast()?);
        }

        // SAFETY: Every element was initialized before this point.
        let output =
            unsafe { core::mem::transmute_copy::<[core::mem::MaybeUninit<U>; N], [U; N]>(&output) };

        Ok(output)
    }
}

impl<T, U, const N: usize> TryExactCast<[U; N]> for [T; N]
where
    T: TryExactCast<U>,
{
    #[inline]
    fn try_exact_cast(self) -> Result<[U; N], CastError> {
        let mut output: [core::mem::MaybeUninit<U>; N] =
            [const { core::mem::MaybeUninit::uninit() }; N];

        for (index, value) in self.into_iter().enumerate() {
            output[index].write(value.try_exact_cast()?);
        }

        // SAFETY: Every element was initialized before this point.
        let output =
            unsafe { core::mem::transmute_copy::<[core::mem::MaybeUninit<U>; N], [U; N]>(&output) };

        Ok(output)
    }
}

#[cfg(any(feature = "alloc", feature = "std", test))]
mod vec_casts {
    use alloc::vec::Vec;

    use crate::numeric::{Cast, CastError, LossyCast, SaturatingCast, TryCast, TryExactCast};

    impl<T, U> Cast<Vec<U>> for Vec<T>
    where
        T: Cast<U>,
    {
        #[inline]
        fn cast(self) -> Vec<U> {
            self.into_iter().map(Cast::cast).collect()
        }
    }

    impl<T, U> LossyCast<Vec<U>> for Vec<T>
    where
        T: LossyCast<U>,
    {
        #[inline]
        fn lossy_cast(self) -> Vec<U> {
            self.into_iter().map(LossyCast::lossy_cast).collect()
        }
    }

    impl<T, U> SaturatingCast<Vec<U>> for Vec<T>
    where
        T: SaturatingCast<U>,
    {
        #[inline]
        fn saturating_cast(self) -> Vec<U> {
            self.into_iter().map(SaturatingCast::saturating_cast).collect()
        }
    }

    impl<T, U> TryCast<Vec<U>> for Vec<T>
    where
        T: TryCast<U>,
    {
        #[inline]
        fn try_cast(self) -> Result<Vec<U>, CastError> {
            self.into_iter().map(TryCast::try_cast).collect()
        }
    }

    impl<T, U> TryExactCast<Vec<U>> for Vec<T>
    where
        T: TryExactCast<U>,
    {
        #[inline]
        fn try_exact_cast(self) -> Result<Vec<U>, CastError> {
            self.into_iter().map(TryExactCast::try_exact_cast).collect()
        }
    }
}
