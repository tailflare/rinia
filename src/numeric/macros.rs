#[macro_export]
macro_rules! impl_numeric_casts_wrapper {
    (@emit_ctor fields, $wrapper:ident, $self:ident, $cast:path, [$($field:ident),+]) => {
        $wrapper {
            $($field: $cast($self.$field)),+
        }
    };

    (@emit_ctor transparent, $wrapper:ident, $self:ident, $cast:path) => {
        $wrapper($cast($self.0))
    };

    (@emit_try_ctor fields, $wrapper:ident, $self:ident, $cast:path, [$($field:ident),+]) => {
        Ok($wrapper {
            $($field: $cast($self.$field)?),+
        })
    };

    (@emit_try_ctor transparent, $wrapper:ident, $self:ident, $cast:path) => {
        $cast($self.0).map($wrapper)
    };

    (@impl
        [$($impl_generics:tt)*],
        wrapper: $wrapper:ident,
        $outer:ty => $cast_outer:ty,
        item: $item:ty,
        mode: $mode:ident $(, [$($field:ident),+])?
    ) => {
        // Cast inherents
        impl<$($impl_generics)*> $outer {
            /// Returns a new value cast to type `U`.
            #[inline]
            pub fn cast<U>(self) -> $cast_outer
            where
                $item: $crate::numeric::Cast<U>,
            {
                $crate::impl_numeric_casts_wrapper!(@emit_ctor
                    $mode,
                    $wrapper,
                    self,
                    $crate::numeric::Cast::cast
                    $(, [$($field),+])?
                )
            }

            /// Casts `value` into `Self`.
            #[inline]
            pub fn cast_from<U>(value: $cast_outer) -> Self
            where
                U: $crate::numeric::Cast<$item>,
            {
                value.cast::<$item>()
            }

            /// Returns a new value lossy cast to type `U`.
            #[inline]
            pub fn lossy_cast<U>(self) -> $cast_outer
            where
                $item: $crate::numeric::LossyCast<U>,
            {
                $crate::impl_numeric_casts_wrapper!(@emit_ctor
                    $mode,
                    $wrapper,
                    self,
                    $crate::numeric::LossyCast::lossy_cast
                    $(, [$($field),+])?
                )
            }

            /// Lossy casts `value` into `Self`.
            #[inline]
            pub fn lossy_cast_from<U>(value: $cast_outer) -> Self
            where
                U: $crate::numeric::LossyCast<$item>,
            {
                value.lossy_cast::<$item>()
            }

            /// Returns a new value saturating cast to type `U`.
            #[inline]
            pub fn saturating_cast<U>(self) -> $cast_outer
            where
                $item: $crate::numeric::SaturatingCast<U>,
            {
                $crate::impl_numeric_casts_wrapper!(@emit_ctor
                    $mode,
                    $wrapper,
                    self,
                    $crate::numeric::SaturatingCast::saturating_cast
                    $(, [$($field),+])?
                )
            }

            /// Saturating casts `value` into `Self`.
            #[inline]
            pub fn saturating_cast_from<U>(value: $cast_outer) -> Self
            where
                U: $crate::numeric::SaturatingCast<$item>,
            {
                value.saturating_cast::<$item>()
            }

            /// Returns a new value with try cast to type `U`.
            #[inline]
            pub fn try_cast<U>(self) -> Result<$cast_outer, $crate::numeric::CastError>
            where
                $item: $crate::numeric::TryCast<U>,
            {
                $crate::impl_numeric_casts_wrapper!(@emit_try_ctor
                    $mode,
                    $wrapper,
                    self,
                    $crate::numeric::TryCast::try_cast
                    $(, [$($field),+])?
                )
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

            /// Returns a new value with try exact cast to type `U`.
            #[inline]
            pub fn try_exact_cast<U>(
                self,
            ) -> Result<$cast_outer, $crate::numeric::CastError>
            where
                $item: $crate::numeric::TryExactCast<U>,
            {
                $crate::impl_numeric_casts_wrapper!(@emit_try_ctor
                    $mode,
                    $wrapper,
                    self,
                    $crate::numeric::TryExactCast::try_exact_cast
                    $(, [$($field),+])?
                )
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

    // Transparent complex form delegates to the core impl with one field.
    (
        [$($impl_generics:tt)*],
        wrapper: $wrapper:ident,
        $outer:ty => $cast_outer:ty,
        item: $item:ty,
        field: $field:ident
        $(,)?
    ) => {
        $crate::impl_numeric_casts_wrapper!(@impl
            [$($impl_generics)*],
            wrapper: $wrapper,
            $outer => $cast_outer,
            item: $item,
            mode: fields, [$field]
        );
    };

    // Named fields complex form.
    (
        [$($impl_generics:tt)*],
        wrapper: $wrapper:ident,
        $outer:ty => $cast_outer:ty,
        item: $item:ty,
        fields: [$($field:ident),+ $(,)?]
        $(,)?
    ) => {
        $crate::impl_numeric_casts_wrapper!(@impl
            [$($impl_generics)*],
            wrapper: $wrapper,
            $outer => $cast_outer,
            item: $item,
            mode: fields, [$($field),+]
        );
    };

    // Transparent shorthand forwards to the core impl.
    ($wrapper:ident<$t:ident>) => {
        $crate::impl_numeric_casts_wrapper!(@impl
            [$t],
            wrapper: $wrapper,
            $wrapper<$t> => $wrapper<U>,
            item: $t,
            mode: transparent
        );
    };

    // Named fields shorthand forwards to the complex form.
    ($wrapper:ident<$t:ident>, [$($field:ident),+ $(,)?]) => {
        $crate::impl_numeric_casts_wrapper!(
            [$t],
            wrapper: $wrapper,
            $wrapper<$t> => $wrapper<U>,
            item: $t,
            fields: [$($field),+],
        );
    };
}
