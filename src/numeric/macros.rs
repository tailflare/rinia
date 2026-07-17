#[macro_export]
macro_rules! impl_numeric_casts_transparent {
    (
        [$($impl_generics:tt)*],
        wrapper: $wrapper:ident,
        $outer:ty => $cast_outer:ty,
        item: $item:ty,
        field: $field:ident
        $(,)?
    ) => {
        // Cast inherents
        impl<$($impl_generics)*> $outer {
            /// Returns a new wrapper with the inner field cast to type `U`.
            #[inline]
            pub fn cast<U>(self) -> $cast_outer
            where
                $item: $crate::numeric::Cast<U>,
            {
                $wrapper {
                    $field: $crate::numeric::Cast::cast(self.$field),
                }
            }

            /// Casts `value` into `Self`.
            #[inline]
            pub fn cast_from<U>(value: $cast_outer) -> Self
            where
                U: $crate::numeric::Cast<$item>,
            {
                value.cast::<$item>()
            }

            /// Returns a new wrapper with the inner field lossy cast to type `U`.
            #[inline]
            pub fn lossy_cast<U>(self) -> $cast_outer
            where
                $item: $crate::numeric::LossyCast<U>,
            {
                $wrapper {
                    $field: $crate::numeric::LossyCast::lossy_cast(self.$field),
                }
            }

            /// Lossy casts `value` into `Self`.
            #[inline]
            pub fn lossy_cast_from<U>(value: $cast_outer) -> Self
            where
                U: $crate::numeric::LossyCast<$item>,
            {
                value.lossy_cast::<$item>()
            }

            /// Returns a new wrapper with the inner field saturating cast to type `U`.
            #[inline]
            pub fn saturating_cast<U>(self) -> $cast_outer
            where
                $item: $crate::numeric::SaturatingCast<U>,
            {
                $wrapper {
                    $field: $crate::numeric::SaturatingCast::saturating_cast(self.$field),
                }
            }

            /// Saturating casts `value` into `Self`.
            #[inline]
            pub fn saturating_cast_from<U>(value: $cast_outer) -> Self
            where
                U: $crate::numeric::SaturatingCast<$item>,
            {
                value.saturating_cast::<$item>()
            }

            /// Returns a new wrapper with the inner field try cast to type `U`.
            #[inline]
            pub fn try_cast<U>(self) -> Result<$cast_outer, $crate::numeric::CastError>
            where
                $item: $crate::numeric::TryCast<U>,
            {
                $crate::numeric::TryCast::try_cast(self.$field).map(|$field| $wrapper { $field })
            }

            /// Try casts `value` into `Self`.
            #[inline]
            pub fn try_cast_from<U>(
                value: $cast_outer,
            ) -> Result<Self, $crate::numeric::CastError>
            where
                U: $crate::numeric::TryCast<$item>,
            {
                value.try_cast::<$item>()
            }

            /// Returns a new wrapper with the inner field try exact cast to type `U`.
            #[inline]
            pub fn try_exact_cast<U>(
                self,
            ) -> Result<$cast_outer, $crate::numeric::CastError>
            where
                $item: $crate::numeric::TryExactCast<U>,
            {
                $crate::numeric::TryExactCast::try_exact_cast(self.$field)
                    .map(|$field| $wrapper { $field })
            }

            /// Try exact casts `value` into `Self`.
            #[inline]
            pub fn try_exact_cast_from<U>(
                value: $cast_outer,
            ) -> Result<Self, $crate::numeric::CastError>
            where
                U: $crate::numeric::TryExactCast<$item>,
            {
                value.try_exact_cast::<$item>()
            }
        }

        // Cast trait
        impl<$($impl_generics)*, U> $crate::numeric::Cast<$cast_outer> for $outer
        where
            $item: $crate::numeric::Cast<U>,
        {
            #[inline]
            fn cast(self) -> $cast_outer {
                <$outer>::cast::<U>(self)
            }
        }

        // LossyCast trait
        impl<$($impl_generics)*, U> $crate::numeric::LossyCast<$cast_outer> for $outer
        where
            $item: $crate::numeric::LossyCast<U>,
        {
            #[inline]
            fn lossy_cast(self) -> $cast_outer {
                <$outer>::lossy_cast::<U>(self)
            }
        }

        // SaturatingCast trait
        impl<$($impl_generics)*, U> $crate::numeric::SaturatingCast<$cast_outer> for $outer
        where
            $item: $crate::numeric::SaturatingCast<U>,
        {
            #[inline]
            fn saturating_cast(self) -> $cast_outer {
                <$outer>::saturating_cast::<U>(self)
            }
        }

        // TryCast trait
        impl<$($impl_generics)*, U> $crate::numeric::TryCast<$cast_outer> for $outer
        where
            $item: $crate::numeric::TryCast<U>,
        {
            #[inline]
            fn try_cast(self) -> Result<$cast_outer, $crate::numeric::CastError> {
                <$outer>::try_cast::<U>(self)
            }
        }

        // TryExactCast trait
        impl<$($impl_generics)*, U> $crate::numeric::TryExactCast<$cast_outer> for $outer
        where
            $item: $crate::numeric::TryExactCast<U>,
        {
            #[inline]
            fn try_exact_cast(self) -> Result<$cast_outer, $crate::numeric::CastError> {
                <$outer>::try_exact_cast::<U>(self)
            }
        }
    };

    ($wrapper:ident<$t:ident>) => {
        // Cast inherents
        impl<$t> $wrapper<$t> {
            /// Returns a new value with the inner value cast to type `U`.
            #[inline]
            pub fn cast<U>(self) -> $wrapper<U>
            where
                $t: $crate::numeric::Cast<U>,
            {
                $wrapper($crate::numeric::Cast::cast(self.0))
            }

            /// Casts `value` into `Self`.
            #[inline]
            pub fn cast_from<U>(value: $wrapper<U>) -> Self
            where
                U: $crate::numeric::Cast<$t>,
            {
                value.cast::<$t>()
            }

            /// Returns a new value with the inner value lossy cast to type `U`.
            #[inline]
            pub fn lossy_cast<U>(self) -> $wrapper<U>
            where
                $t: $crate::numeric::LossyCast<U>,
            {
                $wrapper($crate::numeric::LossyCast::lossy_cast(self.0))
            }

            /// Lossy casts `value` into `Self`.
            #[inline]
            pub fn lossy_cast_from<U>(value: $wrapper<U>) -> Self
            where
                U: $crate::numeric::LossyCast<$t>,
            {
                value.lossy_cast::<$t>()
            }

            /// Returns a new value with the inner value saturating cast to type `U`.
            #[inline]
            pub fn saturating_cast<U>(self) -> $wrapper<U>
            where
                $t: $crate::numeric::SaturatingCast<U>,
            {
                $wrapper($crate::numeric::SaturatingCast::saturating_cast(self.0))
            }

            /// Saturating casts `value` into `Self`.
            #[inline]
            pub fn saturating_cast_from<U>(value: $wrapper<U>) -> Self
            where
                U: $crate::numeric::SaturatingCast<$t>,
            {
                value.saturating_cast::<$t>()
            }

            /// Returns a new value with the inner value try cast to type `U`.
            #[inline]
            pub fn try_cast<U>(self) -> Result<$wrapper<U>, $crate::numeric::CastError>
            where
                $t: $crate::numeric::TryCast<U>,
            {
                $crate::numeric::TryCast::try_cast(self.0).map($wrapper)
            }

            /// Try casts `value` into `Self`.
            #[inline]
            pub fn try_cast_from<U>(value: $wrapper<U>) -> Result<Self, $crate::numeric::CastError>
            where
                U: $crate::numeric::TryCast<$t>,
            {
                value.try_cast::<$t>()
            }

            /// Returns a new value with the inner value try exact cast to type `U`.
            #[inline]
            pub fn try_exact_cast<U>(self) -> Result<$wrapper<U>, $crate::numeric::CastError>
            where
                $t: $crate::numeric::TryExactCast<U>,
            {
                $crate::numeric::TryExactCast::try_exact_cast(self.0).map($wrapper)
            }

            /// Try exact casts `value` into `Self`.
            #[inline]
            pub fn try_exact_cast_from<U>(
                value: $wrapper<U>,
            ) -> Result<Self, $crate::numeric::CastError>
            where
                U: $crate::numeric::TryExactCast<$t>,
            {
                value.try_exact_cast::<$t>()
            }
        }

        // Cast trait
        impl<$t, U> $crate::numeric::Cast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::Cast<U>,
        {
            #[inline]
            fn cast(self) -> $wrapper<U> {
                $wrapper::cast(self)
            }
        }

        // LossyCast trait
        impl<$t, U> $crate::numeric::LossyCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::LossyCast<U>,
        {
            #[inline]
            fn lossy_cast(self) -> $wrapper<U> {
                $wrapper::lossy_cast(self)
            }
        }

        // SaturatingCast trait
        impl<$t, U> $crate::numeric::SaturatingCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::SaturatingCast<U>,
        {
            #[inline]
            fn saturating_cast(self) -> $wrapper<U> {
                $wrapper::saturating_cast(self)
            }
        }

        // TryCast trait
        impl<$t, U> $crate::numeric::TryCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::TryCast<U>,
        {
            #[inline]
            fn try_cast(self) -> Result<$wrapper<U>, $crate::numeric::CastError> {
                $wrapper::try_cast(self)
            }
        }

        // TryExactCast trait
        impl<$t, U> $crate::numeric::TryExactCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::TryExactCast<U>,
        {
            #[inline]
            fn try_exact_cast(self) -> Result<$wrapper<U>, $crate::numeric::CastError> {
                $wrapper::try_exact_cast(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_numeric_casts_named_fields {
    ($wrapper:ident<$t:ident>, [$($field:ident),+ $(,)?]) => {
        // Cast inherents
        impl<$t> $wrapper<$t> {
            /// Returns a new value with the fields cast to type `U`.
            #[inline]
            pub fn cast<U>(self) -> $wrapper<U>
            where
                $t: $crate::numeric::Cast<U>,
            {
                $wrapper {
                    $($field: $crate::numeric::Cast::cast(self.$field)),+
                }
            }

            /// Casts `value` into `Self`.
            #[inline]
            pub fn cast_from<U>(value: $wrapper<U>) -> Self
            where
                U: $crate::numeric::Cast<$t>,
            {
                value.cast::<$t>()
            }

            /// Returns a new value with the fields lossy cast to type `U`.
            #[inline]
            pub fn lossy_cast<U>(self) -> $wrapper<U>
            where
                $t: $crate::numeric::LossyCast<U>,
            {
                $wrapper {
                    $($field: $crate::numeric::LossyCast::lossy_cast(self.$field)),+
                }
            }

            /// Lossy casts `value` into `Self`.
            #[inline]
            pub fn lossy_cast_from<U>(value: $wrapper<U>) -> Self
            where
                U: $crate::numeric::LossyCast<$t>,
            {
                value.lossy_cast::<$t>()
            }

            /// Returns a new value with the fields saturating cast to type `U`.
            #[inline]
            pub fn saturating_cast<U>(self) -> $wrapper<U>
            where
                $t: $crate::numeric::SaturatingCast<U>,
            {
                $wrapper {
                    $($field: $crate::numeric::SaturatingCast::saturating_cast(self.$field)),+
                }
            }

            /// Saturating casts `value` into `Self`.
            #[inline]
            pub fn saturating_cast_from<U>(value: $wrapper<U>) -> Self
            where
                U: $crate::numeric::SaturatingCast<$t>,
            {
                value.saturating_cast::<$t>()
            }

            /// Returns a new value with the fields try cast to type `U`.
            #[inline]
            pub fn try_cast<U>(self) -> Result<$wrapper<U>, $crate::numeric::CastError>
            where
                $t: $crate::numeric::TryCast<U>,
            {
                Ok($wrapper {
                    $($field: $crate::numeric::TryCast::try_cast(self.$field)?),+
                })
            }

            /// Try casts `value` into `Self`.
            #[inline]
            pub fn try_cast_from<U>(value: $wrapper<U>) -> Result<Self, $crate::numeric::CastError>
            where
                U: $crate::numeric::TryCast<$t>,
            {
                value.try_cast::<$t>()
            }

            /// Returns a new value with the fields try exact cast to type `U`.
            #[inline]
            pub fn try_exact_cast<U>(self) -> Result<$wrapper<U>, $crate::numeric::CastError>
            where
                $t: $crate::numeric::TryExactCast<U>,
            {
                Ok($wrapper {
                    $($field: $crate::numeric::TryExactCast::try_exact_cast(self.$field)?),+
                })
            }

            /// Try exact casts `value` into `Self`.
            #[inline]
            pub fn try_exact_cast_from<U>(
                value: $wrapper<U>,
            ) -> Result<Self, $crate::numeric::CastError>
            where
                U: $crate::numeric::TryExactCast<$t>,
            {
                value.try_exact_cast::<$t>()
            }
        }

        // Cast trait
        impl<$t, U> $crate::numeric::Cast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::Cast<U>,
        {
            #[inline]
            fn cast(self) -> $wrapper<U> {
                $wrapper::cast(self)
            }
        }

        // LossyCast trait
        impl<$t, U> $crate::numeric::LossyCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::LossyCast<U>,
        {
            #[inline]
            fn lossy_cast(self) -> $wrapper<U> {
                $wrapper::lossy_cast(self)
            }
        }

        // SaturatingCast trait
        impl<$t, U> $crate::numeric::SaturatingCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::SaturatingCast<U>,
        {
            #[inline]
            fn saturating_cast(self) -> $wrapper<U> {
                $wrapper::saturating_cast(self)
            }
        }

        // TryCast trait
        impl<$t, U> $crate::numeric::TryCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::TryCast<U>,
        {
            #[inline]
            fn try_cast(self) -> Result<$wrapper<U>, $crate::numeric::CastError> {
                $wrapper::try_cast(self)
            }
        }

        // TryExactCast trait
        impl<$t, U> $crate::numeric::TryExactCast<$wrapper<U>> for $wrapper<$t>
        where
            $t: $crate::numeric::TryExactCast<U>,
        {
            #[inline]
            fn try_exact_cast(self) -> Result<$wrapper<U>, $crate::numeric::CastError> {
                $wrapper::try_exact_cast(self)
            }
        }
    };
}
